use crate::codegen::flag_policy::FlagPolicy;
use crate::codegen::*;
use crate::error::CodegenError;
use crate::ir::{Ir, Operand, Type};

use std::sync::Arc;

pub struct InterpretCodegen {
    flag_policy: Arc<dyn FlagPolicy>,
}

impl InterpretCodegen {
    pub fn new<F>(flag_policy: F) -> Self
    where
        F: FlagPolicy + 'static,
    {
        Self {
            flag_policy: Arc::new(flag_policy),
        }
    }
}

impl Codegen for InterpretCodegen {
    type Code = Box<dyn CompiledCode>;

    fn compile(&self, ir: Ir) -> Self::Code {
        unsafe { Box::new(compile_ir(&ir, self.flag_policy.clone()).unwrap()) }
    }
}

unsafe fn compile_ir(
    ir: &Ir,
    flag_policy: Arc<dyn FlagPolicy>,
) -> Result<Box<dyn CompiledCode>, CodegenError> {
    // Optimiations
    match ir {
        Ir::Add(Type::U64, Operand::Ip, Operand::Immediate(Type::I64, imm)) => {
            let imm = *imm;
            return Ok(Box::new(move |ctx| (ctx.ip() as i64 + imm as i64).into()));
        }
        Ir::Add(Type::U64, Operand::Ip, Operand::Immediate(Type::U64, imm)) => {
            let imm = *imm;
            return Ok(Box::new(move |ctx| (ctx.ip() + imm).into()));
        }
        Ir::Value(Operand::Ir(ir)) => return compile_ir(ir, flag_policy.clone()),
        Ir::Value(Operand::Immediate(t, imm)) => {
            let imm = *imm;
            let _t = *t;

            return Ok(Box::new(move |_ctx| imm.into()));
        }
        Ir::Value(Operand::Gpr(t, reg)) => {
            let reg = *reg;
            let _t = *t;
            return Ok(Box::new(move |ctx| ctx.gpr(reg).get().into()));
        }
        Ir::LShr(t, op1, Operand::Immediate(t1, val)) => {
            assert!(t == t1, "Type mismatch");
            let op1 = compile_op(op1, flag_policy.clone())?;
            let val = *val;
            let t = *t;

            return Ok(Box::new(move |ctx| {
                let mut ret = op1(ctx);
                match t.size() {
                    1 => *ret.u8() >>= val,
                    2 => *ret.u16() >>= val,
                    4 => *ret.u32() >>= val,
                    8 => *ret.u64() >>= val,
                    _ => unreachable!(),
                }

                ret
            }));
        }
        Ir::And(Type::U64, Operand::Flag, Operand::Immediate(Type::U64, imm)) => {
            let imm = *imm;
            return Ok(Box::new(move |ctx| (ctx.flag() & imm).into()));
        }
        Ir::And(t, Operand::Immediate(t1, imm1), Operand::Immediate(t2, imm2)) => {
            assert!(t1 == t2 && t1 == t, "Type mismatch");
            let imm1 = *imm1;
            let imm2 = *imm2;
            let _t = *t;

            return Ok(Box::new(move |_ctx| (imm1 & imm2).into()));
        }
        Ir::If(t, Operand::Immediate(t1, imm), op1, op2) => {
            assert!(*t1 == Type::Bool, "Type mismatch");
            assert!(op1.get_type() == op2.get_type(), "Type mismatch");
            let imm = *imm;
            let _t = *t;
            let op1 = compile_op(op1, flag_policy.clone())?;
            let op2 = compile_op(op2, flag_policy.clone())?;

            return Ok(Box::new(
                move |ctx| {
                    if imm != 0 {
                        op1(ctx)
                    } else {
                        op2(ctx)
                    }
                },
            ));
        }

        _ => {}
    };

    match ir {
        Ir::Add(t, op1, op2) => gen_add(t, op1, op2, flag_policy),
        Ir::Sub(t, op1, op2) => gen_sub(t, op1, op2, flag_policy),
        Ir::Mul(t, op1, op2) => gen_mul(t, op1, op2, flag_policy),
        Ir::Div(t, op1, op2) => gen_div(t, op1, op2, flag_policy),
        Ir::Addc(t, op1, op2) => gen_addc(t, op1, op2, flag_policy),
        Ir::Subc(t, op1, op2) => gen_subc(t, op1, op2, flag_policy),

        Ir::And(t, op1, op2) => gen_and(t, op1, op2, flag_policy),
        Ir::Or(t, op1, op2) => gen_or(t, op1, op2, flag_policy),
        Ir::Xor(t, op1, op2) => gen_xor(t, op1, op2, flag_policy),
        Ir::Not(t, op) => gen_not(t, op, flag_policy),

        Ir::LShl(t, op1, op2) => gen_lshl(t, op1, op2, flag_policy),
        Ir::LShr(t, op1, op2) => gen_lshr(t, op1, op2, flag_policy),
        Ir::AShr(t, op1, op2) => gen_ashr(t, op1, op2, flag_policy),
        Ir::Rotr(t, op1, op2) => gen_rotr(t, op1, op2, flag_policy),

        Ir::Load(t, op) => gen_load(t, op, flag_policy),

        Ir::ZextCast(t, op) => gen_zext_cast(t, op, flag_policy),
        Ir::SextCast(t, op) => gen_sext_cast(t, op, flag_policy),
        Ir::BitCast(t, op) => gen_bit_cast(t, op, flag_policy),

        Ir::Value(op) => compile_op(op, flag_policy.clone()),
        Ir::Nop => Ok(Box::new(|_| Value::new())),

        Ir::If(t, cond, if_true, if_false) => gen_if(t, cond, if_true, if_false, flag_policy),
        Ir::CmpEq(op1, op2) => gen_cmp_eq(op1, op2, flag_policy),
        Ir::CmpNe(op1, op2) => gen_cmp_ne(op1, op2, flag_policy),
        Ir::CmpGt(op1, op2) => gen_cmp_gt(op1, op2, flag_policy),
        Ir::CmpLt(op1, op2) => gen_cmp_lt(op1, op2, flag_policy),
    }
}

unsafe fn compile_op(
    op: &Operand,
    flag_policy: Arc<dyn FlagPolicy>,
) -> Result<Box<dyn CompiledCode>, CodegenError> {
    Ok(match op {
        Operand::Ir(ir) => compile_ir(ir, flag_policy.clone())?,
        Operand::Gpr(t, reg) => {
            let reg = *reg;
            let _t = *t;
            Box::new(move |vm| vm.gpr(reg).get().into())
        }
        Operand::Fpr(t, reg) => {
            let reg = *reg;
            let _t = *t;
            Box::new(move |vm| vm.fpr(reg).get().to_bits().into())
        }
        Operand::Immediate(t, imm) => {
            let imm = *imm;
            let _t = *t;
            Box::new(move |_| imm.into())
        }
        Operand::VoidIr(ir) => compile_ir(ir, flag_policy.clone())?,
        Operand::Ip => Box::new(move |ctx| ctx.ip().into()),
        Operand::Flag => Box::new(move |ctx| ctx.flag().into()),
        Operand::Dbg(s, op) => {
            let op = compile_op(op, flag_policy.clone())?;
            let _s = s.to_string();
            Box::new(move |ctx| {
                let _val = op(ctx);
                todo!()
            })
        }
        Operand::VmInfo(_info_ty) => {
            todo!()
        }
    })
}

unsafe fn gen_add(
    t: &Type,
    op1: &Operand,
    op2: &Operand,
    flag_policy: Arc<dyn FlagPolicy>,
) -> Result<Box<dyn CompiledCode>, CodegenError> {
    let t1 = op1.get_type();
    let t2 = op2.get_type();

    if t1.is_float() {
        assert!(t2.is_float() && t1.size() == t2.size())
    } else {
        assert!(t1.is_scalar() && t2.is_scalar() && t1.size() == t2.size())
    }

    let lhs = compile_op(op1, flag_policy.clone())?;
    let rhs = compile_op(op2, flag_policy.clone())?;

    let t = *t;
    Ok(match t {
        Type::U8 | Type::I8 => Box::new(move |ctx| {
            let lhs = *lhs(ctx).u8();
            let rhs = *rhs(ctx).u8();

            lhs.overflowing_add(rhs).0.into()
        }),
        Type::U16 | Type::I16 => Box::new(move |ctx| {
            let lhs = *lhs(ctx).u16();
            let rhs = *rhs(ctx).u16();

            lhs.overflowing_add(rhs).0.into()
        }),
        Type::U32 | Type::I32 => Box::new(move |ctx| {
            let lhs = *lhs(ctx).u32();
            let rhs = *rhs(ctx).u32();

            lhs.overflowing_add(rhs).0.into()
        }),
        Type::U64 | Type::I64 => Box::new(move |ctx| {
            let lhs = *lhs(ctx).u64();
            let rhs = *rhs(ctx).u64();

            lhs.overflowing_add(rhs).0.into()
        }),
        _ => unreachable!("invalid type: {:?}", t),
    })
}

unsafe fn gen_sub(
    t: &Type,
    op1: &Operand,
    op2: &Operand,
    flag_policy: Arc<dyn FlagPolicy>,
) -> Result<Box<dyn CompiledCode>, CodegenError> {
    let t1 = op1.get_type();
    let t2 = op2.get_type();

    if t1.is_float() {
        assert!(t2.is_float() && t1.size() == t2.size())
    } else {
        assert!(t1.is_scalar() && t2.is_scalar() && t1.size() == t2.size())
    }

    let lhs = compile_op(op1, flag_policy.clone())?;
    let rhs = compile_op(op2, flag_policy.clone())?;

    let t = *t;
    Ok(match t {
        Type::U8 | Type::I8 => Box::new(move |ctx| {
            let lhs = *lhs(ctx).u8();
            let rhs = *rhs(ctx).u8();

            lhs.overflowing_sub(rhs).0.into()
        }),
        Type::U16 | Type::I16 => Box::new(move |ctx| {
            let lhs = *lhs(ctx).u16();
            let rhs = *rhs(ctx).u16();

            lhs.overflowing_sub(rhs).0.into()
        }),
        Type::U32 | Type::I32 => Box::new(move |ctx| {
            let lhs = *lhs(ctx).u32();
            let rhs = *rhs(ctx).u32();

            lhs.overflowing_sub(rhs).0.into()
        }),
        Type::U64 | Type::I64 => Box::new(move |ctx| {
            let lhs = *lhs(ctx).u64();
            let rhs = *rhs(ctx).u64();

            lhs.overflowing_sub(rhs).0.into()
        }),
        _ => unreachable!("invalid type: {:?}", t),
    })
}

unsafe fn gen_mul(
    t: &Type,
    op1: &Operand,
    op2: &Operand,
    flag_policy: Arc<dyn FlagPolicy>,
) -> Result<Box<dyn CompiledCode>, CodegenError> {
    let lhs = compile_op(op1, flag_policy.clone())?;
    let rhs = compile_op(op2, flag_policy.clone())?;

    Ok(match t {
        Type::U8 | Type::I8 => Box::new(move |ctx| {
            let lhs = *lhs(ctx).u8();
            let rhs = *rhs(ctx).u8();

            lhs.overflowing_mul(rhs).0.into()
        }),
        Type::U16 | Type::I16 => Box::new(move |ctx| {
            let lhs = *lhs(ctx).u16();
            let rhs = *rhs(ctx).u16();

            lhs.overflowing_mul(rhs).0.into()
        }),
        Type::U32 | Type::I32 => Box::new(move |ctx| {
            let lhs = *lhs(ctx).u32();
            let rhs = *rhs(ctx).u32();

            lhs.overflowing_mul(rhs).0.into()
        }),
        Type::U64 | Type::I64 => Box::new(move |ctx| {
            let lhs = *lhs(ctx).u64();
            let rhs = *rhs(ctx).u64();

            lhs.overflowing_mul(rhs).0.into()
        }),
        _ => unreachable!("invalid type: {:?}", t),
    })
}

unsafe fn gen_div(
    t: &Type,
    op1: &Operand,
    op2: &Operand,
    flag_policy: Arc<dyn FlagPolicy>,
) -> Result<Box<dyn CompiledCode>, CodegenError> {
    let lhs = compile_op(op1, flag_policy.clone())?;
    let rhs = compile_op(op2, flag_policy.clone())?;

    Ok(match t {
        Type::U8 | Type::I8 => Box::new(move |ctx| {
            let lhs = *lhs(ctx).u8();
            let rhs = *rhs(ctx).u8();

            lhs.overflowing_div(rhs).0.into()
        }),
        Type::U16 | Type::I16 => Box::new(move |ctx| {
            let lhs = *lhs(ctx).u16();
            let rhs = *rhs(ctx).u16();

            lhs.overflowing_div(rhs).0.into()
        }),
        Type::U32 | Type::I32 => Box::new(move |ctx| {
            let lhs = *lhs(ctx).u32();
            let rhs = *rhs(ctx).u32();

            lhs.overflowing_div(rhs).0.into()
        }),
        Type::U64 | Type::I64 => Box::new(move |ctx| {
            let lhs = *lhs(ctx).u64();
            let rhs = *rhs(ctx).u64();

            lhs.overflowing_div(rhs).0.into()
        }),
        _ => unreachable!("invalid type: {:?}", t),
    })
}

unsafe fn gen_addc(
    t: &Type,
    op1: &Operand,
    op2: &Operand,
    flag_policy: Arc<dyn FlagPolicy>,
) -> Result<Box<dyn CompiledCode>, CodegenError> {
    let t1 = op1.get_type();
    let t2 = op2.get_type();

    if t1.is_float() {
        assert!(t2.is_float() && t1.size() == t2.size())
    } else {
        assert!(t1.is_scalar() && t2.is_scalar() && t1.size() == t2.size())
    }

    let lhs = compile_op(op1, flag_policy.clone())?;
    let rhs = compile_op(op2, flag_policy.clone())?;

    let t = *t;
    Ok(match t {
        Type::U8 | Type::I8 => Box::new(move |ctx| {
            let lhs = *lhs(ctx).u8();
            let rhs = *rhs(ctx).u8();

            flag_policy.add_carry(t, lhs as u64, rhs as u64, ctx);
            lhs.overflowing_add(rhs).0.into()
        }),
        Type::U16 | Type::I16 => Box::new(move |ctx| {
            let lhs = *lhs(ctx).u16();
            let rhs = *rhs(ctx).u16();

            flag_policy.add_carry(t, lhs as u64, rhs as u64, ctx);
            lhs.overflowing_add(rhs).0.into()
        }),
        Type::U32 | Type::I32 => Box::new(move |ctx| {
            let lhs = *lhs(ctx).u32();
            let rhs = *rhs(ctx).u32();

            flag_policy.add_carry(t, lhs as u64, rhs as u64, ctx);
            lhs.overflowing_add(rhs).0.into()
        }),
        Type::U64 | Type::I64 => Box::new(move |ctx| {
            let lhs = *lhs(ctx).u64();
            let rhs = *rhs(ctx).u64();

            flag_policy.add_carry(t, lhs, rhs, ctx);
            lhs.overflowing_add(rhs).0.into()
        }),
        _ => unreachable!("invalid type: {:?}", t),
    })
}

unsafe fn gen_subc(
    t: &Type,
    op1: &Operand,
    op2: &Operand,
    flag_policy: Arc<dyn FlagPolicy>,
) -> Result<Box<dyn CompiledCode>, CodegenError> {
    let t1 = op1.get_type();
    let t2 = op2.get_type();

    if t1.is_float() {
        assert!(t2.is_float() && t1.size() == t2.size())
    } else {
        assert!(t1.is_scalar() && t2.is_scalar() && t1.size() == t2.size())
    }

    let lhs = compile_op(op1, flag_policy.clone())?;
    let rhs = compile_op(op2, flag_policy.clone())?;

    let t = *t;
    Ok(match t {
        Type::U8 | Type::I8 => Box::new(move |ctx| {
            let lhs = *lhs(ctx).u8();
            let rhs = *rhs(ctx).u8();

            flag_policy.sub_carry(t, lhs as u64, rhs as u64, ctx);
            lhs.overflowing_sub(rhs).0.into()
        }),
        Type::U16 | Type::I16 => Box::new(move |ctx| {
            let lhs = *lhs(ctx).u16();
            let rhs = *rhs(ctx).u16();

            flag_policy.sub_carry(t, lhs as u64, rhs as u64, ctx);
            lhs.overflowing_sub(rhs).0.into()
        }),
        Type::U32 | Type::I32 => Box::new(move |ctx| {
            let lhs = *lhs(ctx).u32();
            let rhs = *rhs(ctx).u32();

            flag_policy.sub_carry(t, lhs as u64, rhs as u64, ctx);
            lhs.overflowing_sub(rhs).0.into()
        }),
        Type::U64 | Type::I64 => Box::new(move |ctx| {
            let lhs = *lhs(ctx).u64();
            let rhs = *rhs(ctx).u64();

            flag_policy.sub_carry(t, lhs, rhs, ctx);
            lhs.overflowing_sub(rhs).0.into()
        }),
        _ => unreachable!("invalid type: {:?}", t),
    })
}

unsafe fn gen_lshl(
    t: &Type,
    op1: &Operand,
    op2: &Operand,
    flag_policy: Arc<dyn FlagPolicy>,
) -> Result<Box<dyn CompiledCode>, CodegenError> {
    let lhs = compile_op(op1, flag_policy.clone())?;
    let rhs = compile_op(op2, flag_policy.clone())?;

    let t = *t;
    Ok(match t {
        Type::U8 | Type::I8 => Box::new(move |ctx| {
            let mut lhs = lhs(ctx);
            let mut rhs = rhs(ctx);

            *lhs.u8() <<= *rhs.u8();
            lhs
        }),
        Type::U16 | Type::I16 => Box::new(move |ctx| {
            let mut lhs = lhs(ctx);
            let mut rhs = rhs(ctx);

            *lhs.u16() <<= *rhs.u16();
            lhs
        }),
        Type::U32 | Type::I32 => Box::new(move |ctx| {
            let mut lhs = lhs(ctx);
            let mut rhs = rhs(ctx);

            *lhs.u32() <<= *rhs.u32();
            lhs
        }),
        Type::U64 | Type::I64 => Box::new(move |ctx| {
            let mut lhs = lhs(ctx);
            let mut rhs = rhs(ctx);

            *lhs.u64() <<= *rhs.u64();
            lhs
        }),
        _ => unreachable!("invalid type: {:?}", t),
    })
}

unsafe fn gen_lshr(
    t: &Type,
    op1: &Operand,
    op2: &Operand,
    flag_policy: Arc<dyn FlagPolicy>,
) -> Result<Box<dyn CompiledCode>, CodegenError> {
    let lhs = compile_op(op1, flag_policy.clone())?;
    let rhs = compile_op(op2, flag_policy.clone())?;

    Ok(match t {
        Type::U8 | Type::I8 => Box::new(move |ctx| {
            let mut lhs = lhs(ctx);
            let mut rhs = rhs(ctx);

            *lhs.u8() >>= *rhs.u8();
            lhs
        }),
        Type::U16 | Type::I16 => Box::new(move |ctx| {
            let mut lhs = lhs(ctx);
            let mut rhs = rhs(ctx);

            *lhs.u16() >>= *rhs.u16();
            lhs
        }),
        Type::U32 | Type::I32 => Box::new(move |ctx| {
            let mut lhs = lhs(ctx);
            let mut rhs = rhs(ctx);

            *lhs.u32() >>= *rhs.u32();
            lhs
        }),
        Type::U64 | Type::I64 => Box::new(move |ctx| {
            let mut lhs = lhs(ctx);
            let mut rhs = rhs(ctx);

            *lhs.u64() >>= *rhs.u64();
            lhs
        }),
        _ => unreachable!("invalid type: {:?}", t),
    })
}

unsafe fn gen_ashr(
    t: &Type,
    op1: &Operand,
    op2: &Operand,
    flag_policy: Arc<dyn FlagPolicy>,
) -> Result<Box<dyn CompiledCode>, CodegenError> {
    assert!(op2.get_type().is_scalar());
    assert!(op2.get_type().is_unsigned());

    let lhs = compile_op(op1, flag_policy.clone())?;
    let rhs = compile_op(op2, flag_policy.clone())?;

    Ok(match t {
        Type::U8 => Box::new(move |ctx| {
            let mut lhs = lhs(ctx);
            let mut rhs = rhs(ctx);

            *lhs.u8() >>= *rhs.u8();
            lhs
        }),
        Type::U16 => Box::new(move |ctx| {
            let mut lhs = lhs(ctx);
            let mut rhs = rhs(ctx);

            *lhs.u16() >>= *rhs.u16();
            lhs
        }),
        Type::U32 => Box::new(move |ctx| {
            let mut lhs = lhs(ctx);
            let mut rhs = rhs(ctx);

            *lhs.u32() >>= *rhs.u32();
            lhs
        }),
        Type::U64 => Box::new(move |ctx| {
            let mut lhs = lhs(ctx);
            let mut rhs = rhs(ctx);

            *lhs.u64() >>= *rhs.u64();
            lhs
        }),
        Type::I8 => Box::new(move |ctx| {
            let mut lhs = lhs(ctx);
            let mut rhs = rhs(ctx);

            *lhs.i8() >>= *rhs.u8();
            lhs
        }),
        Type::I16 => Box::new(move |ctx| {
            let mut lhs = lhs(ctx);
            let mut rhs = rhs(ctx);

            *lhs.i16() >>= *rhs.u16();
            lhs
        }),
        Type::I32 => Box::new(move |ctx| {
            let mut lhs = lhs(ctx);
            let mut rhs = rhs(ctx);

            *lhs.i32() >>= *rhs.u32();
            lhs
        }),
        Type::I64 => Box::new(move |ctx| {
            let mut lhs = lhs(ctx);
            let mut rhs = rhs(ctx);

            *lhs.i64() >>= *rhs.u64();
            lhs
        }),
        _ => unreachable!("invalid type: {:?}", t),
    })
}

unsafe fn gen_rotr(
    t: &Type,
    op1: &Operand,
    op2: &Operand,
    flag_policy: Arc<dyn FlagPolicy>,
) -> Result<Box<dyn CompiledCode>, CodegenError> {
    assert!(op2.get_type().is_scalar());
    assert!(op2.get_type().is_unsigned());

    let lhs = compile_op(op1, flag_policy.clone())?;
    let rhs = compile_op(op2, flag_policy.clone())?;

    Ok(match t {
        Type::U8 | Type::I8 => Box::new(move |ctx| {
            let mut lhs = lhs(ctx);
            let mut rhs = rhs(ctx);

            lhs.u8().rotate_right(*rhs.u8() as u32).into()
        }),
        Type::U16 | Type::I16 => Box::new(move |ctx| {
            let mut lhs = lhs(ctx);
            let mut rhs = rhs(ctx);

            lhs.u16().rotate_right(*rhs.u16() as u32).into()
        }),
        Type::U32 | Type::I32 => Box::new(move |ctx| {
            let mut lhs = lhs(ctx);
            let mut rhs = rhs(ctx);

            lhs.u32().rotate_right(*rhs.u32()).into()
        }),
        Type::U64 | Type::I64 => Box::new(move |ctx| {
            let mut lhs = lhs(ctx);
            let mut rhs = rhs(ctx);

            lhs.u64().rotate_right(*rhs.u64() as u32).into()
        }),
        _ => unreachable!("invalid type: {:?}", t),
    })
}

unsafe fn gen_load(
    t: &Type,
    op: &Operand,
    flag_policy: Arc<dyn FlagPolicy>,
) -> Result<Box<dyn CompiledCode>, CodegenError> {
    let op = compile_op(op, flag_policy.clone())?;
    Ok(match t {
        Type::Bool => Box::new(move |ctx| {
            let mut var = op.execute(ctx);
            (ctx.mem(*var.u64()).read_u8().unwrap() & 0b1).into()
        }),
        Type::U8 | Type::I8 => Box::new(move |ctx| {
            let mut var = op.execute(ctx);
            ctx.mem(*var.u64()).read_u8().unwrap().into()
        }),
        Type::U16 | Type::I16 => Box::new(move |ctx| {
            let mut var = op.execute(ctx);
            ctx.mem(*var.u64()).read_u16().unwrap().into()
        }),
        Type::U32 | Type::I32 | Type::F32 => Box::new(move |ctx| {
            let mut var = op.execute(ctx);
            ctx.mem(*var.u64()).read_u32().unwrap().into()
        }),
        Type::U64 | Type::I64 | Type::F64 => Box::new(move |ctx| {
            let mut var = op.execute(ctx);
            ctx.mem(*var.u64()).read_u64().unwrap().into()
        }),
        _ => unreachable!("invalid type: {:?}", t),
    })
}

unsafe fn gen_zext_cast(
    t: &Type,
    op: &Operand,
    flag_policy: Arc<dyn FlagPolicy>,
) -> Result<Box<dyn CompiledCode>, CodegenError> {
    let op = compile_op(op, flag_policy.clone())?;
    Ok(match t {
        Type::U8
        | Type::U16
        | Type::U32
        | Type::U64
        | Type::I8
        | Type::I16
        | Type::I32
        | Type::I64 => Box::new(move |ctx| op.execute(ctx)),
        _ => unreachable!("invalid type: {:?}", t),
    })
}

unsafe fn gen_sext_cast(
    t: &Type,
    op: &Operand,
    flag_policy: Arc<dyn FlagPolicy>,
) -> Result<Box<dyn CompiledCode>, CodegenError> {
    let from = op.get_type();
    let to = t.gen_mask() as i64;
    let op = compile_op(op, flag_policy.clone())?;

    Ok(match from {
        Type::U8 | Type::U16 | Type::U32 | Type::U64 => op,
        Type::I8 => Box::new(move |ctx| {
            let v: i64 = (*op.execute(ctx).i8()).into();
            (v & to).into()
        }),
        Type::I16 => Box::new(move |ctx| {
            let v: i64 = (*op.execute(ctx).i16()).into();
            (v & to).into()
        }),
        Type::I32 => Box::new(move |ctx| {
            let v: i64 = (*op.execute(ctx).i32()).into();
            (v & to).into()
        }),
        Type::I64 => Box::new(move |ctx| {
            let v: i64 = *op.execute(ctx).i64();
            (v & to).into()
        }),
        _ => unreachable!("invalid type: {:?}", t),
    })
}

unsafe fn gen_bit_cast(
    t: &Type,
    op: &Operand,
    flag_policy: Arc<dyn FlagPolicy>,
) -> Result<Box<dyn CompiledCode>, CodegenError> {
    let op = compile_op(op, flag_policy.clone())?;
    let t = *t;

    Ok(Box::new(move |ctx| unsafe {
        op.execute(ctx).truncate_to(t)
    }))
}

unsafe fn gen_and(
    t: &Type,
    op1: &Operand,
    op2: &Operand,
    flag_policy: Arc<dyn FlagPolicy>,
) -> Result<Box<dyn CompiledCode>, CodegenError> {
    let lhs = compile_op(op1, flag_policy.clone())?;
    let rhs = compile_op(op2, flag_policy.clone())?;

    let t = *t;
    Ok(match t {
        Type::Bool
        | Type::U8
        | Type::U16
        | Type::U32
        | Type::U64
        | Type::I8
        | Type::I16
        | Type::I32
        | Type::I64 => Box::new(move |ctx| {
            let mut lhs = lhs.execute(ctx);
            let mut rhs = rhs.execute(ctx);

            *lhs.u64() &= *rhs.u64();
            lhs
        }),
        _ => unreachable!("invalid type: {:?}", t),
    })
}

unsafe fn gen_or(
    t: &Type,
    op1: &Operand,
    op2: &Operand,
    flag_policy: Arc<dyn FlagPolicy>,
) -> Result<Box<dyn CompiledCode>, CodegenError> {
    let lhs = compile_op(op1, flag_policy.clone())?;
    let rhs = compile_op(op2, flag_policy.clone())?;
    let t = *t;
    Ok(match t {
        Type::Bool
        | Type::U8
        | Type::U16
        | Type::U32
        | Type::U64
        | Type::I8
        | Type::I16
        | Type::I32
        | Type::I64 => Box::new(move |ctx| {
            let mut lhs = lhs.execute(ctx);
            let mut rhs = rhs.execute(ctx);

            *lhs.u64() |= *rhs.u64();
            lhs
        }),
        _ => unreachable!("invalid type: {:?}", t),
    })
}

unsafe fn gen_xor(
    t: &Type,
    op1: &Operand,
    op2: &Operand,
    flag_policy: Arc<dyn FlagPolicy>,
) -> Result<Box<dyn CompiledCode>, CodegenError> {
    let lhs = compile_op(op1, flag_policy.clone())?;
    let rhs = compile_op(op2, flag_policy.clone())?;

    let t = *t;

    Ok(match t {
        Type::Bool
        | Type::U8
        | Type::U16
        | Type::U32
        | Type::U64
        | Type::I8
        | Type::I16
        | Type::I32
        | Type::I64 => Box::new(move |ctx| {
            let mut lhs = lhs.execute(ctx);
            let mut rhs = rhs.execute(ctx);

            *lhs.u64() ^= *rhs.u64();
            lhs
        }),
        _ => unreachable!("invalid type: {:?}", t),
    })
}

unsafe fn gen_not(
    t: &Type,
    op: &Operand,
    flag_policy: Arc<dyn FlagPolicy>,
) -> Result<Box<dyn CompiledCode>, CodegenError> {
    let op = compile_op(op, flag_policy.clone())?;

    let t = *t;

    Ok(match t {
        Type::Bool
        | Type::U8
        | Type::U16
        | Type::U32
        | Type::U64
        | Type::I8
        | Type::I16
        | Type::I32
        | Type::I64 => Box::new(move |ctx| ((!*op.execute(ctx).u64()) & t.gen_mask()).into()),
        _ => unreachable!("invalid type: {:?}", t),
    })
}

unsafe fn gen_if(
    t: &Type,
    cond: &Operand,
    if_true: &Operand,
    if_false: &Operand,
    flag_policy: Arc<dyn FlagPolicy>,
) -> Result<Box<dyn CompiledCode>, CodegenError> {
    assert_eq!(cond.get_type(), Type::Bool);

    let cond = compile_op(cond, flag_policy.clone())?;
    let if_true = compile_op(if_true, flag_policy.clone())?;
    let if_false = compile_op(if_false, flag_policy.clone())?;

    let _t = *t;
    Ok(Box::new(move |ctx| {
        if *cond.execute(ctx).u64() != 0 {
            if_true.execute(ctx)
        } else {
            if_false.execute(ctx)
        }
    }))
}

unsafe fn gen_cmp_eq(
    op1: &Operand,
    op2: &Operand,
    flag_policy: Arc<dyn FlagPolicy>,
) -> Result<Box<dyn CompiledCode>, CodegenError> {
    let lhs = compile_op(op1, flag_policy.clone())?;
    let rhs = compile_op(op2, flag_policy.clone())?;

    let t = op1.get_type();
    assert_eq!(op1.get_type(), op2.get_type());

    Ok(match t {
        Type::U8 | Type::I8 => Box::new(move |ctx| {
            let lhs = *lhs.execute(ctx).u8();
            let rhs = *rhs.execute(ctx).u8();

            (lhs == rhs).into()
        }),
        Type::U16 | Type::I16 => Box::new(move |ctx| {
            let lhs = *lhs.execute(ctx).u16();
            let rhs = *rhs.execute(ctx).u16();

            (lhs == rhs).into()
        }),
        Type::U32 | Type::I32 | Type::F32 => Box::new(move |ctx| {
            let lhs = *lhs.execute(ctx).u32();
            let rhs = *rhs.execute(ctx).u32();

            (lhs == rhs).into()
        }),
        Type::U64 | Type::I64 | Type::F64 => Box::new(move |ctx| {
            let lhs = *lhs.execute(ctx).u64();
            let rhs = *rhs.execute(ctx).u64();

            (lhs == rhs).into()
        }),
        _ => unreachable!("invalid type: {:?}", t),
    })
}

unsafe fn gen_cmp_ne(
    op1: &Operand,
    op2: &Operand,
    flag_policy: Arc<dyn FlagPolicy>,
) -> Result<Box<dyn CompiledCode>, CodegenError> {
    let lhs = compile_op(op1, flag_policy.clone())?;
    let rhs = compile_op(op2, flag_policy.clone())?;

    let t = op1.get_type();
    assert_eq!(op1.get_type(), op2.get_type());

    Ok(match t {
        Type::U8 | Type::I8 => Box::new(move |ctx| {
            let lhs = *lhs.execute(ctx).u8();
            let rhs = *rhs.execute(ctx).u8();

            (lhs != rhs).into()
        }),
        Type::U16 | Type::I16 => Box::new(move |ctx| {
            let lhs = *lhs.execute(ctx).u16();
            let rhs = *rhs.execute(ctx).u16();

            (lhs != rhs).into()
        }),
        Type::U32 | Type::I32 | Type::F32 => Box::new(move |ctx| {
            let lhs = *lhs.execute(ctx).u32();
            let rhs = *rhs.execute(ctx).u32();

            (lhs != rhs).into()
        }),
        Type::U64 | Type::I64 | Type::F64 => Box::new(move |ctx| {
            let lhs = *lhs.execute(ctx).u64();
            let rhs = *rhs.execute(ctx).u64();

            (lhs != rhs).into()
        }),
        _ => unreachable!("invalid type: {:?}", t),
    })
}

unsafe fn gen_cmp_gt(
    op1: &Operand,
    op2: &Operand,
    flag_policy: Arc<dyn FlagPolicy>,
) -> Result<Box<dyn CompiledCode>, CodegenError> {
    let lhs = compile_op(op1, flag_policy.clone())?;
    let rhs = compile_op(op2, flag_policy.clone())?;

    let t = op1.get_type();
    assert_eq!(op1.get_type(), op2.get_type());

    Ok(match t {
        Type::U8 | Type::I8 => Box::new(move |ctx| {
            let lhs = *lhs.execute(ctx).u8();
            let rhs = *rhs.execute(ctx).u8();

            (lhs > rhs).into()
        }),
        Type::U16 | Type::I16 => Box::new(move |ctx| {
            let lhs = *lhs.execute(ctx).u16();
            let rhs = *rhs.execute(ctx).u16();

            (lhs > rhs).into()
        }),
        Type::U32 | Type::I32 | Type::F32 => Box::new(move |ctx| {
            let lhs = *lhs.execute(ctx).u32();
            let rhs = *rhs.execute(ctx).u32();

            (lhs > rhs).into()
        }),
        Type::U64 | Type::I64 | Type::F64 => Box::new(move |ctx| {
            let lhs = *lhs.execute(ctx).u64();
            let rhs = *rhs.execute(ctx).u64();

            (lhs > rhs).into()
        }),
        _ => unreachable!("invalid type: {:?}", t),
    })
}

unsafe fn gen_cmp_lt(
    op1: &Operand,
    op2: &Operand,
    flag_policy: Arc<dyn FlagPolicy>,
) -> Result<Box<dyn CompiledCode>, CodegenError> {
    let lhs = compile_op(op1, flag_policy.clone())?;
    let rhs = compile_op(op2, flag_policy.clone())?;

    let t = op1.get_type();
    assert_eq!(op1.get_type(), op2.get_type());

    Ok(match t {
        Type::U8 | Type::I8 => Box::new(move |ctx| {
            let lhs = *lhs.execute(ctx).u8();
            let rhs = *rhs.execute(ctx).u8();

            (lhs < rhs).into()
        }),
        Type::U16 | Type::I16 => Box::new(move |ctx| {
            let lhs = *lhs.execute(ctx).u16();
            let rhs = *rhs.execute(ctx).u16();

            (lhs < rhs).into()
        }),
        Type::U32 | Type::I32 | Type::F32 => Box::new(move |ctx| {
            let lhs = *lhs.execute(ctx).u32();
            let rhs = *rhs.execute(ctx).u32();

            (lhs < rhs).into()
        }),
        Type::U64 | Type::I64 | Type::F64 => Box::new(move |ctx| {
            let lhs = *lhs.execute(ctx).u64();
            let rhs = *rhs.execute(ctx).u64();

            (lhs < rhs).into()
        }),
        _ => unreachable!("invalid type: {:?}", t),
    })
}
