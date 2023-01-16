use crate::aarch64::*;
use crate::bit_patterns::*;
use crate::utils::{extract_bits32, BitReader};
use crate::MachineInstrParserRule;

use std::cell::RefCell;

/// AArch64 instruction parser
pub struct AArch64InstrParserRule;

impl MachineInstrParserRule for AArch64InstrParserRule {
    type MachineInstr = AArch64Instr;

    fn parse<I>(&mut self, buf: &mut BitReader<I>) -> Option<Self::MachineInstr>
    where
        I: Iterator<Item = u8>,
    {
        parse_aarch64_instr(buf)
    }
}

fn parse_aarch64_instr<I>(reader: &mut BitReader<I>) -> Option<AArch64Instr>
where
    I: Iterator<Item = u8>,
{
    // AArch64 instruction has fixed length of 32 bits
    thread_local! {
        pub static MATCHER: RefCell<BitPatternMatcher<AArch64Instr>> = {
            let mut m = BitPatternMatcher::new();
            m .bind("0_xx_0000_xxxxxxxxxxxxxxxxxxxxxxxxx", |raw_instr: u32| {
                todo!("Reserved")
            }).bind("1_xx_0000_xxxxxxxxxxxxxxxxxxxxxxxxx", |raw_instr: u32| {
                todo!("SME encodings")
            }).bind("x_xx_0010_xxxxxxxxxxxxxxxxxxxxxxxxx", |raw_instr: u32| {
                todo!("SVE encodings")
            }).bind("x_xx_100x_xxxxxxxxxxxxxxxxxxxxxxxxx", |raw_instr: u32| {
                parse_aarch64_d_p_i(raw_instr)
            }).bind("x_xx_101x_xxxxxxxxxxxxxxxxxxxxxxxxx", |raw_instr: u32| {
                parse_aarch64_branches_exception_gen_and_sys_instr(raw_instr)
            }).bind("x_xx_x1x0_xxxxxxxxxxxxxxxxxxxxxxxxx", |raw_instr: u32| {
                parse_aarch64_load_and_stores(raw_instr)
            }).bind("x_xx_x101_xxxxxxxxxxxxxxxxxxxxxxxxx", |raw_instr: u32| {
                parse_aarch64_d_p_r(raw_instr)
            }).bind("x_xx_x111_xxxxxxxxxxxxxxxxxxxxxxxxx", |raw_instr: u32| {
                parse_aarch64_dp_sfp_adv_simd(raw_instr)
            });

            RefCell::new(m)
        }
    }

    if let Some(raw_instr) = reader.read32() {
        return MATCHER.with(|v| {
            let mut v = v.try_borrow_mut().unwrap();
            if let Some(instr) = v.handle(raw_instr) {
                return Some(instr);
            } else {
                todo!("Unknown instruction {:032b}", raw_instr);
            }
        });
    }

    None
}

// parse DPI(Data Processing Immediate) instructions in AArch64
fn parse_aarch64_d_p_i(raw_instr: u32) -> AArch64Instr {
    thread_local! {
        pub static MATCHER: RefCell<BitPatternMatcher<AArch64Instr>> = {
            let mut m = BitPatternMatcher::new();
            m .bind("xxx_100_00x_xxxxxxxxxxxxxxxxxxxxxxx", |raw_instr: u32| {
                parse_pc_rel_addressing(raw_instr)
            }).bind("xxx_100_010_xxxxxxxxxxxxxxxxxxxxxxx", |raw_instr: u32| {
                parse_add_sub_immediate(raw_instr)
            }).bind("xxx_100_011_xxxxxxxxxxxxxxxxxxxxxxx", |raw_instr: u32| {
                parse_add_sub_imm_with_tags(raw_instr)
            }).bind("xxx_100_100_xxxxxxxxxxxxxxxxxxxxxxx", |raw_instr: u32| {
                parse_logical_imm(raw_instr)
            }).bind("xxx_100_101_xxxxxxxxxxxxxxxxxxxxxxx", |raw_instr: u32| {
                parse_move_wide_imm(raw_instr)
            }).bind("xxx_100_110_xxxxxxxxxxxxxxxxxxxxxxx", |raw_instr: u32| {
                parse_bitfield(raw_instr)
            }).bind("xxx_100_111_xxxxxxxxxxxxxxxxxxxxxxx", |raw_instr: u32| {
                parse_extract(raw_instr)
            });


            RefCell::new(m)
        }
    }

    return MATCHER.with(|v| {
        let mut v = v.try_borrow_mut().unwrap();
        if let Some(instr) = v.handle(raw_instr) {
            return instr;
        } else {
            todo!("Unknown instruction {:032b}", raw_instr);
        }
    });
}

// parse DPI(Data Processing Register) instructions in AArch64
fn parse_aarch64_d_p_r(raw_instr: u32) -> AArch64Instr {
    thread_local! {
        pub static MATCHER: RefCell<BitPatternMatcher<AArch64Instr>> = {
            let mut m = BitPatternMatcher::new();
            m .bind("x_0_x_1_101_0110_xxxxx_xxxxxx_xxxxxxxxxx", |raw_instr: u32| {
                todo!("Data processing (2 source)")
            }).bind("x_1_x_1_101_0110_xxxxx_xxxxxx_xxxxxxxxxx", |raw_instr: u32| {
                todo!("Data processing (1 source)")
            }).bind("x_x_x_0_101_0xxx_xxxxx_xxxxxx_xxxxxxxxxx", |raw_instr: u32| {
                parse_logical_shifted_register(raw_instr)
            }).bind("x_x_x_0_101_1xx0_xxxxx_xxxxxx_xxxxxxxxxx", |raw_instr: u32| {
                parse_add_sub_shifted_reg(raw_instr)
            }).bind("x_x_x_0_101_1xx1_xxxxx_xxxxxx_xxxxxxxxxx", |raw_instr: u32| {
                parse_add_sub_ext_reg(raw_instr)
            }).bind("x_x_x_1_101_0000_xxxxx_000000_xxxxxxxxxx", |raw_instr: u32| {
                todo!("Add/subtract (with carry)")
            }).bind("x_x_x_1_101_0000_xxxxx_x00001_xxxxxxxxxx", |raw_instr: u32| {
                todo!("Rotate Right into flags")
            }).bind("x_x_x_1_101_0000_xxxxx_xx0010_xxxxxxxxxx", |raw_instr: u32| {
                todo!("Evaluate into flags")
            }).bind("x_x_x_1_101_0010_xxxxx_xxxx0x_xxxxxxxxxx", |raw_instr: u32| {
                todo!("Conditional compare (register)")
            }).bind("x_x_x_1_101_0010_xxxxx_xxxx1x_xxxxxxxxxx", |raw_instr: u32| {
                todo!("Conditional compare (immediate)")
            }).bind("x_x_x_1_101_0100_xxxxx_xxxxxx_xxxxxxxxxx", |raw_instr: u32| {
                parse_cond_sel(raw_instr)
            }).bind("x_x_x_1_101_1xxx_xxxxx_xxxxxx_xxxxxxxxxx", |raw_instr: u32| {
                parse_data_proccessing_3src(raw_instr)
            });

            RefCell::new(m)
        }
    }

    return MATCHER.with(|v| {
        let mut v = v.try_borrow_mut().unwrap();
        if let Some(instr) = v.handle(raw_instr) {
            return instr;
        } else {
            todo!("Unknown instruction {:032b}", raw_instr);
        }
    });
}

fn parse_aarch64_dp_sfp_adv_simd(raw_instr: u32) -> AArch64Instr {
    thread_local! {
        pub static MATCHER: RefCell<BitPatternMatcher<AArch64Instr>> = {
            let mut m = BitPatternMatcher::new();
            m.bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "0100", "0x", "x101", "00xxxxx10"), |raw_instr: u32| {
                todo!("Cryptographic AES")
            })
            .bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "0101", "0x", "x0xx", "xxx0xxx00"), |raw_instr: u32| {
                todo!("Cryptographic three-register SHA")
            })
            .bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "0101", "0x", "x101", "00xxxxx10"), |raw_instr: u32| {
                todo!("Cryptographic two-register SHA")
            })
            .bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "01x1", "00", "00xx", "xxx0xxxx1"), |raw_instr: u32| {
                todo!("Advanced SIMD scalar copy")
            })
            .bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "01x1", "0x", "10xx", "xxx00xxx1"), |raw_instr: u32| {
                todo!("Advanced SIMD scalar three same FP16")
            })
            .bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "01x1", "0x", "1111", "00xxxxx10"), |raw_instr: u32| {
                todo!("Advanced SIMD scalar two-register miscellaneous FP16")
            })
            .bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "01x1", "0x", "x0xx", "xxx1xxxx1"), |raw_instr: u32| {
                todo!("Advanced SIMD scalar three same extra")
            })
            .bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "01x1", "0x", "x100", "00xxxxx10"), |raw_instr: u32| {
                todo!("Advanced SIMD scalar two-register miscellaneous")
            })
            .bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "01x1", "0x", "x110", "00xxxxx10"), |raw_instr: u32| {
                todo!("Advanced SIMD scalar pairwise")
            })
            .bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "01x1", "0x", "x1xx", "xxxxxxx00"), |raw_instr: u32| {
                todo!("Advanced SIMD scalar three different")
            })
            .bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "01x1", "0x", "x1xx", "xxxxxxxx1"), |raw_instr: u32| {
                todo!("Advanced SIMD scalar three same")
            })
            .bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "01x1", "10", "xxxx", "xxxxxxxx1"), |raw_instr: u32| {
                todo!("Advanced SIMD scalar shifted by immediate")
            })
            .bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "01x1", "1x", "xxxx", "xxxxxxxx0"), |raw_instr: u32| {
                todo!("Advanced SIMD scalar x indexed element")
            })
            .bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "0x00", "0x", "x0xx", "xxx0xxx00"), |raw_instr: u32| {
                todo!("Advanced SIMD table lookup")
            })
            .bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "0x00", "0x", "x0xx", "xxx0xxx10"), |raw_instr: u32| {
                todo!("Advanced SIMD permute")
            })
            .bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "0x10", "0x", "x0xx", "xxx0xxxx0"), |raw_instr: u32| {
                todo!("Advanced SIMD extract")
            })
            .bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "0xx0", "00", "00xx", "xxx0xxxx1"), |raw_instr: u32| {
                todo!("Advanced SIMD copy")
            })
            .bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "0xx0", "0x", "10xx", "xxx00xxx1"), |raw_instr: u32| {
                todo!("Advanced SIMD three same (FP16)")
            })
            .bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "0xx0", "0x", "1111", "00xxxxx10"), |raw_instr: u32| {
                todo!("Advanced SIMD two-register miscellaneous (FP16)")
            })
            .bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "0xx0", "0x", "x0xx", "xxx1xxxx1"), |raw_instr: u32| {
                todo!("Advanced SIMD three-register extension")
            })
            .bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "0xx0", "0x", "x100", "00xxxxx10"), |raw_instr: u32| {
                todo!("Advanced SIMD two-register miscellaneous")
            })
            .bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "0xx0", "0x", "x110", "00xxxxx10"), |raw_instr: u32| {
                todo!("Advanced SIMD across lanes")
            })
            .bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "0xx0", "0x", "x1xx", "xxxxxxx00"), |raw_instr: u32| {
                todo!("Advanced SIMD three different")
            })
            .bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "0xx0", "0x", "x1xx", "xxxxxxxx1"), |raw_instr: u32| {
                todo!("Advanced SIMD three same")
            })
            .bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "0xx0", "10", "xxxx", "xxxxxxxx1"), |raw_instr: u32, op2: Extract<BitRange<19, 23>, u8> | {
                if op2.value == 0b0000 {
                    todo!("Advanced SIMD modified immediate")
                }
                else {
                    todo!("Advanced SIMD shift by immediate")
                }
            })
            .bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "0xx0", "1x", "xxxx", "xxxxxxxx0"), |raw_instr: u32| {
                todo!("Advanced SIMD vector x indexed element")
            })
            .bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "1100", "00", "10xx", "xxx10xxxx"), |raw_instr: u32| {
                todo!("Cryptographic three-register, imm2")
            })
            .bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "1100", "00", "11xx", "xxx1x00xx"), |raw_instr: u32| {
                todo!("Cryptographic three-reigster SHA 512")
            })
            .bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "1100", "00", "xxxx", "xxx0xxxxx"), |raw_instr: u32| {
                todo!("Cryptographic four-register")
            })
            .bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "1100", "01", "00xx", "xxxxxxxxx"), |raw_instr: u32| {
                todo!("XAR")
            })
            .bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "1100", "01", "1000", "0001000xx"), |raw_instr: u32| {
                todo!("Cryptographic two-register SHA 512")
            })
            .bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "x0x1", "0x", "x0xx", "xxxxxxxxx"), |raw_instr: u32| {
                todo!("Conversion between floating-point and fixed point")
            })
            .bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "x0x1", "0x", "x1xx", "xxx000000"), |raw_instr: u32| {
                todo!("Conversion between floating-point and integer")
            })
            .bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "x0x1", "0x", "x1xx", "xxxx10000"), |raw_instr: u32| {
                todo!("Floating-point data-processing (1 source)")
            })
            .bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "x0x1", "0x", "x1xx", "xxxxx1000"), |raw_instr: u32| {
                todo!("Floating-point compare")
            })
            .bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "x0x1", "0x", "x1xx", "xxxxxx100"), |raw_instr: u32| {
                todo!("Floating-point immediate")
            })
            .bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "x0x1", "0x", "x1xx", "xxxxxxx01"), |raw_instr: u32| {
                todo!("Floating-point conditional compare")
            })
            .bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "x0x1", "0x", "x1xx", "xxxxxxx10"), |raw_instr: u32| {
                todo!("Floating-point data-processing (2 source)")
            })
            .bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "x0x1", "0x", "x1xx", "xxxxxxx11"), |raw_instr: u32| {
                todo!("Floating-point conditional select")
            })
            .bind(&format!("{}_xxx_{}_{}_{}_xxxxxxxxxx", "x0x1", "1x", "xxxx", "xxxxxxxxx"), |raw_instr: u32| {
                parse_fp_data_processing_3src(raw_instr)
            });

            RefCell::new(m)
        }
    }

    return MATCHER.with(|v| {
        let mut v = v.try_borrow_mut().unwrap();
        if let Some(instr) = v.handle(raw_instr) {
            return instr;
        } else {
            todo!("Unknown instruction {:032b}", raw_instr);
        }
    });
}

// parse Load and stores instructions in AArch64
fn parse_aarch64_load_and_stores(raw_instr: u32) -> AArch64Instr {
    thread_local! {
        pub static MATCHER: RefCell<BitPatternMatcher<AArch64Instr>> = {
            let mut m = BitPatternMatcher::new();
            m .bind("0x00_1_0_0_00_x_1xxxxx_xxxx_xx_xxxxxxxxxx", |raw_instr: u32| {
                todo!("Compare and swap");
            }).bind("0x00_1_1_0_00_x_000000_xxxx_xx_xxxxxxxxxx", |raw_instr: u32| {
                todo!("Advanced SIMD Load/Store multiple structure");
            }).bind("0x00_1_1_0_01_x_0xxxxx_xxxx_xx_xxxxxxxxxx", |raw_instr: u32| {
                todo!("Advanced SIMD Load/Store multiple structure(post-indexed)");
            }).bind("0x00_1_1_0_10_x_x00000_xxxx_xx_xxxxxxxxxx", |raw_instr: u32| {
                todo!("Advanced SIMD Load/Store single structure");
            }).bind("0x00_1_1_0_11_x_xxxxxx_xxxx_xx_xxxxxxxxxx", |raw_instr: u32| {
                todo!("Advanced SIMD Load/Store single structure(post-indexed)");
            }).bind("1101_1_0_0_1x_x_1xxxxx_xxxx_xx_xxxxxxxxxx", |raw_instr: u32| {
                todo!("Load/store memory tags");
            }).bind("1x00_1_0_0_00_x_1xxxxx_xxxx_xx_xxxxxxxxxx", |raw_instr: u32| {
                todo!("Load/store exclusive pair");
            }).bind("xx00_1_0_0_00_x_0xxxxx_xxxx_xx_xxxxxxxxxx", |raw_instr: u32| {
                todo!("Load/store exclusive register");
            }).bind("xx00_1_0_0_01_x_0xxxxx_xxxx_xx_xxxxxxxxxx", |raw_instr: u32| {
                todo!("Load/store ordered");
            }).bind("xx00_1_0_0_01_x_1xxxxx_xxxx_xx_xxxxxxxxxx", |raw_instr: u32| {
                todo!("Compare and Swap");
            }).bind("xx01_1_0_0_1x_x_0xxxxx_xxxx_00_xxxxxxxxxx", |raw_instr: u32| {
                todo!("LDAPR/STLR(unscaled immediate)");
            }).bind("xx01_1_x_0_0x_x_xxxxxx_xxxx_xx_xxxxxxxxxx", |raw_instr: u32| {
                todo!("Load register (literal)");
            }).bind("xx01_1_x_0_1x_x_0xxxxx_xxxx_01_xxxxxxxxxx", |raw_instr: u32| {
                todo!("Memory Copy and Memory Set");
            }).bind("xx10_1_x_0_00_x_xxxxxx_xxxx_xx_xxxxxxxxxx", |raw_instr: u32| {
                todo!("Load/Store no allocate pair (offset)");
            }).bind("xx10_1_x_0_01_x_xxxxxx_xxxx_xx_xxxxxxxxxx", |raw_instr: u32| {
                todo!("Load/Store register pair (post-indexed)");
            }).bind("xx10_1_x_0_10_x_xxxxxx_xxxx_xx_xxxxxxxxxx", |raw_instr: u32| {
                parse_load_store_reg_pair_offset(raw_instr)
            }).bind("xx10_1_x_0_11_x_xxxxxx_xxxx_xx_xxxxxxxxxx", |raw_instr: u32| {
                todo!("Load/Store register pair (pre-indexed)");
            }).bind("xx11_1_x_0_0x_x_0xxxxx_xxxx_00_xxxxxxxxxx", |raw_instr: u32| {
                todo!("Load/Store register (unscaled immediate)");
            }).bind("xx11_1_x_0_0x_x_0xxxxx_xxxx_01_xxxxxxxxxx", |raw_instr: u32| {
                todo!("Load/Store register (immidiate post-indexed)");
            }).bind("xx11_1_x_0_0x_x_0xxxxx_xxxx_10_xxxxxxxxxx", |raw_instr: u32| {
                todo!("Load/Store register (unprevilaged)");
            }).bind("xx11_1_x_0_0x_x_0xxxxx_xxxx_11_xxxxxxxxxx", |raw_instr: u32| {
                todo!("Load/Store register (immidiate pre-indexed)");
            }).bind("xx11_1_x_0_0x_x_1xxxxx_xxxx_00_xxxxxxxxxx", |raw_instr: u32| {
                todo!("Atomic memory operations");
            }).bind("xx11_1_x_0_0x_x_1xxxxx_xxxx_10_xxxxxxxxxx", |raw_instr: u32| {
                parse_load_store_reg_reg_offset(raw_instr)
            }).bind("xx11_1_x_0_0x_x_1xxxxx_xxxx_x1_xxxxxxxxxx", |raw_instr: u32| {
                todo!("Load/Store register (pac)");
            }).bind("xx11_1_x_0_1x_x_xxxxxx_xxxx_xx_xxxxxxxxxx", |raw_instr: u32| {
                parse_load_store_reg_unsigned_imm(raw_instr)
            });


            RefCell::new(m)
        }
    }

    return MATCHER.with(|v| {
        let mut v = v.try_borrow_mut().unwrap();
        if let Some(instr) = v.handle(raw_instr) {
            return instr;
        } else {
            todo!("Unknown instruction {:032b}", raw_instr);
        }
    });
}

fn parse_aarch64_branches_exception_gen_and_sys_instr(raw_instr: u32) -> AArch64Instr {
    thread_local! {
        pub static MATCHER: RefCell<BitPatternMatcher<AArch64Instr>> = {
            let mut m = BitPatternMatcher::new();
            //--------------------------------------------
            //      |op1|101|      op2     |       | op3 |
            m .bind("010_101_0xxxxxxxxxxxxx_xxxxxxx_xxxxx", |raw_instr: u32| {
                parse_cond_branch_imm(raw_instr)
            }).bind("110_101_00xxxxxxxxxxxx_xxxxxxx_xxxxx", |raw_instr: u32| {
                parse_exception_gen(raw_instr)
            }).bind("110_101_01000000110001_xxxxxxx_xxxxx", |raw_instr: u32| {
                todo!("System Instruction with Register Argument");
            }).bind("110_101_01000000110010_xxxxxxx_11111", |raw_instr: u32| {
                parse_hints(raw_instr)
            }).bind("110_101_01000000110011_xxxxxxx_xxxxx", |raw_instr: u32| {
                todo!("Barriors")
            }).bind("110_101_0100000xxx0100_xxxxxxx_xxxxx", |raw_isntr: u32| {
                todo!("PSTATE")
            }).bind("110_101_0100100xxxxxxx_xxxxxxx_xxxxx", |raw_instr: u32| {
                todo!("System with results")
            }).bind("110_101_0100x01xxxxxxx_xxxxxxx_xxxxx", |raw_instr: u32| {
                todo!("System instructions")
            }).bind("110_101_0100x1xxxxxxxx_xxxxxxx_xxxxx", |raw_instr: u32| {
                todo!("System register move")
            }).bind("110_101_1xxxxxxxxxxxxx_xxxxxxx_xxxxx", |raw_instr: u32| {
                parse_uncond_branch_reg(raw_instr)
            }).bind("x00_101_xxxxxxxxxxxxxx_xxxxxxx_xxxxx", |raw_instr: u32| {
                parse_uncond_branch_imm(raw_instr)
            }).bind("x01_101_0xxxxxxxxxxxxx_xxxxxxx_xxxxx", |raw_instr: u32| {
                parse_cmp_and_branch_imm(raw_instr)
            }).bind("x01_101_1xxxxxxxxxxxxx_xxxxxxx_xxxxx", |raw_instr: u32| {
                parse_test_and_branch_imm(raw_instr)
            });


            RefCell::new(m)
        }
    }

    return MATCHER.with(|v| {
        let mut v = v.try_borrow_mut().unwrap();
        if let Some(instr) = v.handle(raw_instr) {
            return instr;
        } else {
            todo!("Unknown instruction {:032b}", raw_instr);
        }
    });
}

fn parse_add_sub_shifted_reg(raw_instr: u32) -> AArch64Instr {
    thread_local! {
        pub static MATCHER: RefCell<BitPatternMatcher<AArch64Instr>> = {
            let mut m = BitPatternMatcher::new();
            m.bind("xxx_01011_xx_0_xxxxxxxxxxxxxxxxxxxxx",
            |raw_instr: u32,
             sf_op_s: Extract<BitRange<29, 32>, u8>,
             shift: Extract<BitRange<21, 24>, u8>,
             rm: Extract<BitRange<16, 21>, u8>,
             imm6: Extract<BitRange<10, 16>, u8>,
             rn: Extract<BitRange<5, 10>, u8>,
             rd: Extract<BitRange<0, 5>, u8>| {
                let data = RmRnRd {
                    rm: rm.value,
                    rn: rn.value,
                    rd: rd.value,
                };

                match (sf_op_s.value, shift.value, imm6.value) {
                    (0b000, _, _) => AArch64Instr::AddShiftedReg32(data),
                    (0b001, _, _) => AArch64Instr::AddsShiftedReg32(data),
                    (0b010, _, _) => AArch64Instr::SubShiftedReg32(data),
                    (0b011, _, _) => AArch64Instr::SubsShiftedReg32(data),
                    (0b100, _, _) => AArch64Instr::AddShiftedReg64(data),
                    (0b101, _, _) => AArch64Instr::AddsShiftedReg64(data),
                    (0b110, _, _) => AArch64Instr::SubShiftedReg64(data),
                    (0b111, _, _) => AArch64Instr::SubsShiftedReg64(data),

                    _ => todo!("Unknown instruction {:032b}", raw_instr),
                }
            });

            RefCell::new(m)
        }
    }

    return MATCHER.with(|v| {
        let mut v = v.try_borrow_mut().unwrap();
        if let Some(instr) = v.handle(raw_instr) {
            return instr;
        } else {
            todo!("Unknown instruction {:032b}", raw_instr);
        }
    });
}

fn parse_add_sub_immediate(raw_instr: u32) -> AArch64Instr {
    thread_local! {
        pub static MATCHER: RefCell<BitPatternMatcher<AArch64Instr>> = {
            let mut m = BitPatternMatcher::new();
            m.bind("x_x_x_100010_x_xxxxxxxxxxxx_xxxxx_xxxxx",
            |raw_instr: u32,
             sf_op_s: Extract<BitRange<29, 32>, u8>,
             sh: Extract<BitRange<22, 23>, u8>,
             imm12: Extract<BitRange<10, 22>, u16>,
            rn: Extract<BitRange<5, 10>, u8>,
            rd: Extract<BitRange<0, 5>, u8>,
             | {
                let data = ShImm12RnRd {
                    sh: sh.value,
                    imm12: imm12.value,
                    rn: rn.value,
                    rd: rd.value,
                };

                match sf_op_s.value {
                    0b000 => AArch64Instr::AddImmediate32(data),
                    0b001 => AArch64Instr::AddsImmediate32(data),
                    0b010 => AArch64Instr::SubImmediate32(data),
                    0b011 => AArch64Instr::SubsImmediate32(data),
                    0b100 => AArch64Instr::AddImmediate64(data),
                    0b101 => AArch64Instr::AddsImmediate64(data),
                    0b110 => AArch64Instr::SubImmediate64(data),
                    0b111 => AArch64Instr::SubsImmediate64(data),
                    _ => todo!("Unknown instruction {:032b}", raw_instr),
                }
            });

            RefCell::new(m)
        }
    }

    return MATCHER.with(|v| {
        let mut v = v.try_borrow_mut().unwrap();
        if let Some(instr) = v.handle(raw_instr) {
            return instr;
        } else {
            todo!("Unknown instruction {:032b}", raw_instr);
        }
    });
}

fn parse_fp_data_processing_3src(raw_instr: u32) -> AArch64Instr {
    thread_local! {
        pub static MATCHER: RefCell<BitPatternMatcher<AArch64Instr>> = {
            let mut m = BitPatternMatcher::new();
            m.bind("x_0_x_11111_xx_x_xxxxx_x_xxxxx_xxxxx_xxxxx",
            |raw_instr: u32,
             m: Extract<BitRange<31, 32>, u8>,
             s: Extract<BitRange<29, 30>, u8>,
             ptype: Extract<BitRange<22, 24>, u8>,
             o1: Extract<BitRange<21, 22>, u8>,
             rm: Extract<BitRange<16, 21>, u8>,
             o0: Extract<BitRange<15, 16>, u8>,
             ra: Extract<BitRange<10, 15>, u8>,
             rn: Extract<BitRange<5, 10>, u8>,
             rd: Extract<BitRange<0, 5>, u8>,
             | {
                let data = RmRaRnRd {
                    rm: rm.value,
                    ra: ra.value,
                    rn: rn.value,
                    rd: rd.value,
                };

                match (m.value, s.value, ptype.value, o1.value, o0.value) {
                    (0b0, 0b0, 0b00, 0b0, 0b0) => AArch64Instr::FmAddSinglePrecision(data),
                    (0b0, 0b0, 0b00, 0b0, 0b1) => AArch64Instr::FmSubSinglePrecision(data),
                    (0b0, 0b0, 0b00, 0b1, 0b0) => AArch64Instr::FnmAddSinglePrecision(data),
                    (0b0, 0b0, 0b00, 0b1, 0b1) => AArch64Instr::FnmSubSinglePrecision(data),
                    (0b0, 0b0, 0b01, 0b0, 0b0) => AArch64Instr::FmAddDoublePrecision(data),
                    (0b0, 0b0, 0b01, 0b0, 0b1) => AArch64Instr::FmSubDoublePrecision(data),
                    (0b0, 0b0, 0b01, 0b1, 0b0) => AArch64Instr::FnmAddDoublePrecision(data),
                    (0b0, 0b0, 0b01, 0b1, 0b1) => AArch64Instr::FnmSubDoublePrecision(data),
                    (0b0, 0b0, 0b11, 0b0, 0b0) => AArch64Instr::FmAddHalfPrecision(data),
                    (0b0, 0b0, 0b11, 0b0, 0b1) => AArch64Instr::FmSubHalfPrecision(data),
                    (0b0, 0b0, 0b11, 0b1, 0b0) => AArch64Instr::FnmAddHalfPrecision(data),
                    (0b0, 0b0, 0b11, 0b1, 0b1) => AArch64Instr::FnmSubHalfPrecision(data),
                    _ => todo!("Unknown instruction {:032b}", raw_instr),
                }
            });

            RefCell::new(m)
        }
    }

    return MATCHER.with(|v| {
        let mut v = v.try_borrow_mut().unwrap();
        if let Some(instr) = v.handle(raw_instr) {
            return instr;
        } else {
            todo!("Unknown instruction {:032b}", raw_instr);
        }
    });
}

fn parse_load_store_reg_unsigned_imm(raw_instr: u32) -> AArch64Instr {
    thread_local! {
        pub static MATCHER: RefCell<BitPatternMatcher<AArch64Instr>> = {
            let mut m = BitPatternMatcher::new();
            m.bind("xx_111_x_01_xx_xxxxxxxxxxxx_xxxxx_xxxxx",
            |raw_instr: u32,
             size: Extract<BitRange<30, 32>, u8>,
             v: Extract<BitRange<26, 27>, u8>,
             opc: Extract<BitRange<22, 24>, u8>,
             imm12: Extract<BitRange<10, 22>, u16>,
             rm: Extract<BitRange<5, 10>, u8>,
             rt: Extract<BitRange<0, 5>, u8>,
            | {
                let data = Imm12RnRt {
                    imm12: imm12.value,
                    rn: rm.value,
                    rt: rt.value,
                };

                match (size.value, v.value, opc.value) {
                    (0b00, 0b0, 0b00) => AArch64Instr::StrbImm(data),
                    (0b00, 0b0, 0b01) => AArch64Instr::LdrbImm(data),
                    (0b00, 0b0, 0b10) => AArch64Instr::Ldrsb64(data),
                    (0b00, 0b0, 0b11) => AArch64Instr::Ldrsb32(data),
                    (0b00, 0b1, 0b00) => AArch64Instr::StrImmSimdFP8(data),
                    (0b00, 0b1, 0b01) => AArch64Instr::LdrImmSimdFP8(data),
                    (0b00, 0b1, 0b10) => AArch64Instr::StrImmSimdFP128(data),
                    (0b00, 0b1, 0b11) => AArch64Instr::LdrImmSimdFP128(data),
                    (0b01, 0b0, 0b00) => AArch64Instr::StrhImm(data),
                    (0b01, 0b0, 0b01) => AArch64Instr::LdrhImm(data),
                    (0b01, 0b0, 0b10) => AArch64Instr::LdrshImm64(data),
                    (0b01, 0b0, 0b11) => AArch64Instr::LdrshImm32(data),
                    (0b01, 0b1, 0b00) => AArch64Instr::StrImmSimdFP16(data),
                    (0b01, 0b1, 0b01) => AArch64Instr::LdrImmSimdFP16(data),
                    (0b10, 0b0, 0b00) => AArch64Instr::StrImm32(data),
                    (0b10, 0b0, 0b01) => AArch64Instr::LdrImm32(data),
                    (0b10, 0b0, 0b10) => AArch64Instr::LdrswImm(data),
                    (0b10, 0b1, 0b00) => AArch64Instr::StrImmSimdFP32(data),
                    (0b10, 0b1, 0b01) => AArch64Instr::LdrImmSimdFP32(data),
                    (0b11, 0b0, 0b00) => AArch64Instr::StrImm64(data),
                    (0b11, 0b0, 0b01) => AArch64Instr::LdrImm64(data),
                    (0b11, 0b0, 0b10) => AArch64Instr::Prfm(data),
                    (0b11, 0b1, 0b00) => AArch64Instr::StrImmSimdFP64(data),
                    (0b11, 0b1, 0b01) => AArch64Instr::LdrImmSimdFP64(data),
                    _ => todo!("Unknown instruction {:032b}", raw_instr),
                }
            });

            RefCell::new(m)
        }
    }

    return MATCHER.with(|v| {
        let mut v = v.try_borrow_mut().unwrap();
        if let Some(instr) = v.handle(raw_instr) {
            return instr;
        } else {
            todo!("Unknown instruction {:032b}", raw_instr);
        }
    });
}

fn parse_move_wide_imm(raw_instr: u32) -> AArch64Instr {
    thread_local! {
        pub static MATCHER: RefCell<BitPatternMatcher<AArch64Instr>> = {
            let mut m = BitPatternMatcher::new();
            m.bind("x_xx_100101_xx_xxxxxxxxxxxxxxxx_xxxxx",
            |raw_instr: u32,
             sf_opc: Extract<BitRange<29, 32>, u8>,
             hw: Extract<BitRange<21, 23>, u8>,
             imm16: Extract<BitRange<5, 21>, u16>,
             rd: Extract<BitRange<0, 5>, u8>,
            | {
                let data = Imm16Rd {
                    imm16: imm16.value,
                    rd: rd.value,
                };

                match (sf_opc.value, hw.value) {
                    (0b000, 0b00 | 0b01) => AArch64Instr::Movn32(data),
                    (0b010, 0b00 | 0b01) => AArch64Instr::Movz32(data),
                    (0b011, 0b00 | 0b01) => AArch64Instr::Movk32(data),
                    (0b100, _) => AArch64Instr::Movn64(data),
                    (0b110, _) => AArch64Instr::Movz64(data),
                    (0b111, _) => AArch64Instr::Movk64(data),
                    _ => todo!("Unknown instruction {:032b}", raw_instr),
                }
            });

            RefCell::new(m)
        }
    }

    return MATCHER.with(|v| {
        let mut v = v.try_borrow_mut().unwrap();
        if let Some(instr) = v.handle(raw_instr) {
            return instr;
        } else {
            todo!("Unknown instruction {:032b}", raw_instr);
        }
    });
}

fn parse_uncond_branch_reg(raw_instr: u32) -> AArch64Instr {
    thread_local! {
        pub static MATCHER: RefCell<BitPatternMatcher<AArch64Instr>> = {
            let mut m = BitPatternMatcher::new();
            m.bind("1101011_xxxx_xxxxx_xxxxxx_xxxxx_xxxxx",
            |raw_instr: u32,
             opc: Extract<BitRange<21, 25>, u8>,
             op2: Extract<BitRange<16, 21>, u8>,
             op3: Extract<BitRange<10, 16>, u8>,
             rn: Extract<BitRange<5, 10>, u8>,
             op4: Extract<BitRange<0, 5>, u8> |
            {
                let z = extract_bits32(24..25, raw_instr);
                let op = extract_bits32(21..23, raw_instr);
                let a = extract_bits32(11..12, raw_instr);
                let rn = rn.value;
                let rm = op4.value;

                let data = UncondBranchReg {
                    z: z as u8,
                    op: op as u8,
                    a: a as u8,
                    rn: rn,
                    rm: rm,
                };

                match (opc.value, op2.value, op3.value, rn, rm) {
                    (0b0000, 0b11111, 0b000000, _, 0b00000) => AArch64Instr::Br(data),
                    (0b0000, 0b11111, 0b000010, _, 0b11111) => todo!("BRAA, BRAAZ, BRAB, BRABZ. Key A, zero modifier"),
                    (0b0000, 0b11111, 0b000011, _, 0b11111) => todo!("BRAA, BRAAZ, BRAB, BRABZ. Key B, zero modifier"),
                    (0b0001, 0b11111, 0b000000, _, 0b00000) => AArch64Instr::Blr(data),
                    (0b0001, 0b11111, 0b000010, _, 0b11111) => todo!("BLRAA, BLRAAZ, BLRAB, BLRABZ. Key A, zero modifier"),
                    (0b0001, 0b11111, 0b000011, _, 0b11111) => todo!("BLRAA, BLRAAZ, BLRAB, BLRABZ. Key B, zero modifier"),
                    (0b0010, 0b11111, 0b000000, _, 0b00000) => AArch64Instr::Ret(data),
                    (0b0010, 0b11111, 0b000010, 0b11111, 0b11111) => todo!("RETAA, RETAB - RETAA variant"),
                    (0b0010, 0b11111, 0b000011, 0b11111, 0b11111) => todo!("RETAA, RETAB - RETAB variant"),
                    (0b0100, 0b11111, 0b000000, 0b11111, 0b00000) => AArch64Instr::ERet(data),
                    (0b0100, 0b11111, 0b000010, 0b11111, 0b11111) => todo!("ERETAA, ERETAB - ERETAA variant"),
                    (0b0100, 0b11111, 0b000011, 0b11111, 0b11111) => todo!("ERETAA, ERETAB - ERETAB variant"),
                    (0b0101, 0b11111, 0b000000, 0b11111, 0b00000) => AArch64Instr::Drps(data),
                    (0b1000, 0b11111, 0b000010, _, _) => todo!("BRAA, BRAAZ, BRAB, BRABZ - Key A, register modifier"),
                    (0b1000, 0b11111, 0b000011, _, _) => todo!("BRAA, BRAAZ, BRAB, BRABZ - Key B, register modifier"),
                    (0b1001, 0b11111, 0b000010, _, _) => todo!("BLRAA, BLRAAZ, BLRAB, BLRABZ - Key A, register modifier"),
                    (0b1001, 0b11111, 0b000011, _, _) => todo!("BLRAA, BLRAAZ, BLRAB, BLRABZ - Key B, register modifier"),
                    _ => todo!("Unknown instruction {:032b}", raw_instr),
                }
            });

            RefCell::new(m)
        }
    }

    return MATCHER.with(|v| {
        let mut v = v.try_borrow_mut().unwrap();
        if let Some(instr) = v.handle(raw_instr) {
            return instr;
        } else {
            todo!("Unknown instruction {:032b}", raw_instr);
        }
    });
}

fn parse_uncond_branch_imm(raw_instr: u32) -> AArch64Instr {
    thread_local! {
        pub static MATCHER: RefCell<BitPatternMatcher<AArch64Instr>> = {
            let mut m = BitPatternMatcher::new();
            m.bind("x_00101_xxxxxxxxxxxxxxxxxxxxxxxxxx",
            |raw_instr: u32,
             op: Extract<BitRange<31, 32>, u8>,
             imm26: Extract<BitRange<0, 26>, u32>,
            | {
                let data = Imm26 {
                    imm26: imm26.value,
                };

                match op.value {
                    0b0 => AArch64Instr::BImm(data),
                    0b1 => AArch64Instr::BlImm(data),
                    _ => todo!("Unknown instruction {:032b}", raw_instr),
                }
            });

            RefCell::new(m)
        }
    }

    return MATCHER.with(|v| {
        let mut v = v.try_borrow_mut().unwrap();
        if let Some(instr) = v.handle(raw_instr) {
            return instr;
        } else {
            todo!("Unknown instruction {:032b}", raw_instr);
        }
    });
}

fn parse_cond_branch_imm(raw_instr: u32) -> AArch64Instr {
    thread_local! {
        pub static MATCHER: RefCell<BitPatternMatcher<AArch64Instr>> = {
            let mut m = BitPatternMatcher::new();
            m.bind("0101010_x_xxxxxxxxxxxxxxxxxxx_x_xxxx",
            |raw_instr: u32,
             o1: Extract<BitRange<24, 25>, u8>,
             imm19: Extract<BitRange<5, 24>, u32>,
             o0: Extract<BitRange<4, 5>, u8>,
             cond: Extract<BitRange<0, 4>, u8>,
             | {
                let data = Imm19Cond {
                    imm19: imm19.value,
                    cond: cond.value,
                };

                match (o1.value, o0.value) {
                    (0b0, 0b0) => AArch64Instr::BCond(data),
                    (0b0, 0b1) => AArch64Instr::BcCond(data),
                    _ => todo!("Unknown instruction {:032b}", raw_instr),
                }
            });

            RefCell::new(m)
        }
    }

    return MATCHER.with(|v| {
        let mut v = v.try_borrow_mut().unwrap();
        if let Some(instr) = v.handle(raw_instr) {
            return instr;
        } else {
            todo!("Unknown instruction {:032b}", raw_instr);
        }
    });
}

fn parse_cond_sel(raw_instr: u32) -> AArch64Instr {
    thread_local! {
        pub static MATCHER: RefCell<BitPatternMatcher<AArch64Instr>> = {
            let mut m = BitPatternMatcher::new();
            m.bind("x_x_x_11010100_xxxxx_xxxx_xx_xxxxx_xxxxx",
            |raw_instr: u32,
             sf_op_s: Extract<BitRange<29, 32>, u8>,
             rm: Extract<BitRange<16, 21>, u8>,
             cond: Extract<BitRange<12, 16>, u8>,
             op2: Extract<BitRange<10, 12>, u8>,
             rn: Extract<BitRange<5, 10>, u8>,
             rd: Extract<BitRange<0, 5>, u8>,
             | {
                let data = RmCondRnRd {
                    rm: rm.value,
                    cond: cond.value,
                    rn: rn.value,
                    rd: rd.value,
                };

                match (sf_op_s.value, op2.value) {
                    (0b000, 0b00) => AArch64Instr::Csel32(data),
                    (0b000, 0b01) => AArch64Instr::Csel32(data),
                    (0b010, 0b00) => AArch64Instr::Csel32(data),
                    (0b010, 0b01) => AArch64Instr::Csel32(data),
                    (0b100, 0b00) => AArch64Instr::Csel32(data),
                    (0b100, 0b01) => AArch64Instr::Csel32(data),
                    (0b110, 0b00) => AArch64Instr::Csel32(data),
                    (0b110, 0b01) => AArch64Instr::Csel32(data),
                    _ => todo!("Unknown instruction {:032b}", raw_instr),
                }
            });

            RefCell::new(m)
        }
    }

    return MATCHER.with(|v| {
        let mut v = v.try_borrow_mut().unwrap();
        if let Some(instr) = v.handle(raw_instr) {
            return instr;
        } else {
            todo!("Unknown instruction {:032b}", raw_instr);
        }
    });
}

fn parse_test_and_branch_imm(raw_instr: u32) -> AArch64Instr {
    thread_local! {
        pub static MATCHER: RefCell<BitPatternMatcher<AArch64Instr>> = {
            let mut m = BitPatternMatcher::new();
            m.bind("x_011011_x_xxxxx_xxxxxxxxxxxxxx_xxxxx",
            |raw_instr: u32,
             b5: Extract<BitRange<31, 32>, u8>,
             op: Extract<BitRange<24, 25>, u8>,
             b40: Extract<BitRange<19, 24>, u8>,
             imm14: Extract<BitRange<5, 19>, u16>,
             rt: Extract<BitRange<0, 5>, u8>,
             | {
                let data = B5B40Imm14Rt {
                    b5: b5.value,
                    b40: b40.value,
                    imm14: imm14.value,
                    rt: rt.value,
                };

                match op.value {
                    0b0 => AArch64Instr::Tbz(data),
                    0b1 => AArch64Instr::Tbnz(data),
                    _ => todo!("Unknown instruction {:032b}", raw_instr),
                }
            });

            RefCell::new(m)
        }
    }

    return MATCHER.with(|v| {
        let mut v = v.try_borrow_mut().unwrap();
        if let Some(instr) = v.handle(raw_instr) {
            return instr;
        } else {
            todo!("Unknown instruction {:032b}", raw_instr);
        }
    });
}

fn parse_logical_shifted_register(raw_instr: u32) -> AArch64Instr {
    thread_local! {
        pub static MATCHER: RefCell<BitPatternMatcher<AArch64Instr>> = {
            let mut m = BitPatternMatcher::new();
            m.bind("x_xx_01010_xx_x_xxxxx_xxxxxx_xxxxx_xxxxx",
            |raw_instr: u32,
             sf: Extract<BitRange<31, 32>, u8>,
             opc: Extract<BitRange<29, 31>, u8>,
             shift: Extract<BitRange<22, 24>, u8>,
             n: Extract<BitRange<21, 22>, u8>,
             rm: Extract<BitRange<16, 21>, u8>,
             imm6: Extract<BitRange<10, 16>, u8>,
             rn: Extract<BitRange<5, 10>, u8>,
             rd: Extract<BitRange<0, 5>, u8>,
             | {
                let data = ShiftRmImm6RnRd {
                    shift: shift.value,
                    rm: rm.value,
                    imm6: imm6.value,
                    rn: rn.value,
                    rd: rd.value,
                };

                match (sf.value, opc.value, n.value) {
                    (0b0, _, _) if imm6.value & 0b100000 == 0b100000 => unreachable!(),
                    (0b0, 0b00, 0b0) => AArch64Instr::AndShiftedReg32(data),
                    (0b0, 0b00, 0b1) => AArch64Instr::BicShiftedReg32(data),
                    (0b0, 0b01, 0b0) => AArch64Instr::OrrShiftedReg32(data),
                    (0b0, 0b01, 0b1) => AArch64Instr::OrnShiftedReg32(data),
                    (0b0, 0b10, 0b0) => AArch64Instr::EorShiftedReg32(data),
                    (0b0, 0b10, 0b1) => AArch64Instr::EonShiftedReg32(data),
                    (0b0, 0b11, 0b0) => AArch64Instr::AndsShiftedReg32(data),
                    (0b0, 0b11, 0b1) => AArch64Instr::BicsShiftedReg32(data),
                    (0b1, 0b00, 0b0) => AArch64Instr::AndShiftedReg64(data),
                    (0b1, 0b00, 0b1) => AArch64Instr::BicShiftedReg64(data),
                    (0b1, 0b01, 0b0) => AArch64Instr::OrrShiftedReg64(data),
                    (0b1, 0b01, 0b1) => AArch64Instr::OrnShiftedReg64(data),
                    (0b1, 0b10, 0b0) => AArch64Instr::EorShiftedReg64(data),
                    (0b1, 0b10, 0b1) => AArch64Instr::EonShiftedReg64(data),
                    (0b1, 0b11, 0b0) => AArch64Instr::AndsShiftedReg64(data),
                    (0b1, 0b11, 0b1) => AArch64Instr::BicsShiftedReg64(data),
                    _ => todo!("Unknown instruction {:032b}", raw_instr),
                }
            });

            RefCell::new(m)
        }
    }

    return MATCHER.with(|v| {
        let mut v = v.try_borrow_mut().unwrap();
        if let Some(instr) = v.handle(raw_instr) {
            return instr;
        } else {
            todo!("Unknown instruction {:032b}", raw_instr);
        }
    });
}

fn parse_hints(raw_instr: u32) -> AArch64Instr {
    thread_local! {
        pub static MATCHER: RefCell<BitPatternMatcher<AArch64Instr>> = {
            let mut m = BitPatternMatcher::new();
            m.bind("11010101000000110010_xxxx_xxx_11111",
            |raw_instr: u32,
             crm: Extract<BitRange<8, 12>, u8>,
             op2: Extract<BitRange<5, 8>, u8>,
            | {
                match (crm.value, op2.value) {
                    (0b0000, 0b000) => AArch64Instr::Nop,
                    (0b0000, 0b001) => AArch64Instr::Yield,
                    (0b0000, 0b010) => AArch64Instr::Wfe,
                    (0b0000, 0b011) => AArch64Instr::Wfi,
                    (0b0000, 0b100) => AArch64Instr::Sev,
                    (0b0000, 0b101) => AArch64Instr::Sevl,
                    _ => todo!("Unknown instruction {:032b}", raw_instr),
                }
            });

            RefCell::new(m)
        }
    }

    return MATCHER.with(|v| {
        let mut v = v.try_borrow_mut().unwrap();
        if let Some(instr) = v.handle(raw_instr) {
            return instr;
        } else {
            todo!("Unknown instruction {:032b}", raw_instr);
        }
    });
}

fn parse_pc_rel_addressing(raw_instr: u32) -> AArch64Instr {
    thread_local! {
        pub static MATCHER: RefCell<BitPatternMatcher<AArch64Instr>> = {
            let mut m = BitPatternMatcher::new();
            m.bind("x_xx_10000_xxxxxxxxxxxxxxxxxxx_xxxxx",
            |raw_instr: u32,
             op: Extract<BitRange<31, 32>, u8>,
             immlo: Extract<BitRange<29, 30>, u8>,
             immhi: Extract<BitRange<5, 24>, u32>,
             rd: Extract<BitRange<0, 5>, u8>,
            | {
                let data = PcRelAddressing {
                    immlo: immlo.value,
                    immhi: immhi.value,
                    rd: rd.value,
                };

                match op.value {
                    0b0 => AArch64Instr::Adr(data),
                    0b1 => AArch64Instr::Adrp(data),
                    _ => todo!("Unknown instruction {:032b}", raw_instr),
                }
            });

            RefCell::new(m)
        }
    }

    return MATCHER.with(|v| {
        let mut v = v.try_borrow_mut().unwrap();
        if let Some(instr) = v.handle(raw_instr) {
            return instr;
        } else {
            todo!("Unknown instruction {:032b}", raw_instr);
        }
    });
}

fn parse_exception_gen(raw_instr: u32) -> AArch64Instr {
    thread_local! {
        pub static MATCHER: RefCell<BitPatternMatcher<AArch64Instr>> = {
            let mut m = BitPatternMatcher::new();
            m.bind("11010100_xxx_xxxxxxxxxxxxxxxx_xxx_xx",
            |raw_instr: u32,
             opc: Extract<BitRange<21, 24>, u8>,
             imm16: Extract<BitRange<5, 21>, u16>,
             op2: Extract<BitRange<2, 5>, u8>,
             ll: Extract<BitRange<0, 2>, u8>,
            | {
                let data = ExceptionGen {
                    opc: opc.value,
                    imm16: imm16.value,
                    op2: op2.value,
                    ll: ll.value,
                };

                match (opc.value, op2.value, ll.value) {
                    (0b000, 0b000, 0b01) => AArch64Instr::Svc(data),
                    (0b000, 0b000, 0b10) => AArch64Instr::Hvc(data),
                    (0b000, 0b000, 0b11) => AArch64Instr::Smc(data),
                    (0b001, 0b000, 0b00) => AArch64Instr::Brk(data),
                    (0b010, 0b000, 0b00) => AArch64Instr::Hlt(data),
                    (0b011, 0b000, 0b00) => AArch64Instr::TCancle(data),
                    (0b101, 0b000, 0b01) => AArch64Instr::DcpS1(data),
                    (0b101, 0b000, 0b10) => AArch64Instr::DcpS2(data),
                    (0b101, 0b000, 0b11) => AArch64Instr::DcpS3(data),
                    _ => todo!("Unknown instruction {:032b}", raw_instr),
                }
            });

            RefCell::new(m)
        }
    }

    return MATCHER.with(|v| {
        let mut v = v.try_borrow_mut().unwrap();
        if let Some(instr) = v.handle(raw_instr) {
            return instr;
        } else {
            todo!("Unknown instruction {:032b}", raw_instr);
        }
    });
}

fn parse_load_store_reg_reg_offset(raw_instr: u32) -> AArch64Instr {
    thread_local! {
        pub static MATCHER: RefCell<BitPatternMatcher<AArch64Instr>> = {
            let mut m = BitPatternMatcher::new();
            m.bind("xx_111_x_00_xx_1_xxxxx_xxx_x_10_xxxxx_xxxxx",
            |raw_instr: u32,
             size: Extract<BitRange<30, 32>, u8>,
             v: Extract<BitRange<26, 27>, u8>,
             opc: Extract<BitRange<22, 24>, u8>,
             rm: Extract<BitRange<16, 21>, u8>,
             option: Extract<BitRange<13, 16>, u8>,
             s: Extract<BitRange<12, 13>, u8>,
             rn: Extract<BitRange<5, 10>, u8>,
             rt: Extract<BitRange<0, 5>, u8>,
            | {
                let data = LoadStoreRegRegOffset {
                    size: size.value,
                    v: v.value,
                    opc: opc.value,
                    rm: rm.value,
                    option: option.value,
                    s: s.value,
                    rn: rn.value,
                    rt: rt.value,
                };

                match (size.value, v.value, opc.value, option.value) {
                    (0b00, 0b0, 0b00, _) if option.value != 0b011 => AArch64Instr::StrbRegExtReg(data),
                    (0b00, 0b0, 0b00, 0b011) => AArch64Instr::StrbRegShiftedReg(data),
                    (0b00, 0b0, 0b01, _) if option.value != 0b011 => AArch64Instr::LdrbRegExtReg(data),
                    (0b00, 0b0, 0b01, 0b011) => AArch64Instr::LdrbRegShiftedReg(data),
                    (0b00, 0b0, 0b10, _) if option.value != 0b011 => AArch64Instr::LdrsbRegExtReg64(data),
                    (0b00, 0b0, 0b10, 0b011) => AArch64Instr::LdrsbRegShiftedReg64(data),
                    (0b00, 0b0, 0b11, _) if option.value != 0b011 => AArch64Instr::LdrsbRegExtReg32(data),
                    (0b00, 0b0, 0b11, 0b011) => AArch64Instr::LdrsbRegShiftedReg32(data),
                    (_, 0b1, 0b00, _) | (0b00, 0b1, 0b10, _) => AArch64Instr::StrRegSimdFP(data),
                    (_, 0b1, 0b01, _) | (0b00, 0b1, 0b11, _) => AArch64Instr::LdrRegSimdFP(data),
                    (0b01, 0b0, 0b00, _) => AArch64Instr::StrhReg(data),
                    (0b01, 0b0, 0b01, _) => AArch64Instr::LdrhReg(data),
                    (0b01, 0b0, 0b10, _) => AArch64Instr::LdrshReg64(data),
                    (0b01, 0b0, 0b11, _) => AArch64Instr::LdrshReg32(data),
                    (0b10, 0b0, 0b00, _) => AArch64Instr::StrReg32(data),
                    (0b10, 0b0, 0b01, _) => AArch64Instr::LdrReg32(data),
                    (0b10, 0b0, 0b10, _) => AArch64Instr::LdrswReg(data),
                    (0b11, 0b0, 0b00, _) => AArch64Instr::StrReg64(data),
                    (0b11, 0b0, 0b01, _) => AArch64Instr::LdrReg64(data),
                    (0b11, 0b0, 0b10, _) => AArch64Instr::PrfmReg(data),
                    _ => todo!("Unknown instruction {:032b}", raw_instr),
                }
            });

            RefCell::new(m)
        }
    }

    return MATCHER.with(|v| {
        let mut v = v.try_borrow_mut().unwrap();
        if let Some(instr) = v.handle(raw_instr) {
            return instr;
        } else {
            todo!("Unknown instruction {:032b}", raw_instr);
        }
    });
}

fn parse_add_sub_ext_reg(raw_instr: u32) -> AArch64Instr {
    thread_local! {
        pub static MATCHER: RefCell<BitPatternMatcher<AArch64Instr>> = {
            let mut m = BitPatternMatcher::new();
            m.bind("x_x_x_01011_xx_1_xxxxx_xxx_xxx_xxxxx_xxxxx",
            |raw_instr: u32,
             sf_op_s: Extract<BitRange<29, 32>, u8>,
             opt: Extract<BitRange<22, 24>, u8>,
             rm: Extract<BitRange<16, 21>, u8>,
             option: Extract<BitRange<13, 16>, u8>,
             imm3: Extract<BitRange<10, 13>, u8>,
             rn: Extract<BitRange<5, 10>, u8>,
             rd: Extract<BitRange<0, 5>, u8>,
            | {
                let data = AddSubtractExtReg {
                    rm: rm.value,
                    option: option.value,
                    imm3: imm3.value,
                    rn: rn.value,
                    rd: rd.value,
                };

                match (sf_op_s.value, opt.value) {
                    (0b000, 0b00) => AArch64Instr::AddExtReg32(data),
                    (0b001, 0b00) => AArch64Instr::AddsExtReg32(data),
                    (0b010, 0b00) => AArch64Instr::SubExtReg32(data),
                    (0b011, 0b00) => AArch64Instr::SubsExtReg32(data),
                    (0b100, 0b00) => AArch64Instr::AddExtReg64(data),
                    (0b101, 0b00) => AArch64Instr::AddsExtReg64(data),
                    (0b110, 0b00) => AArch64Instr::SubExtReg64(data),
                    (0b111, 0b00) => AArch64Instr::SubsExtReg64(data),
                    _ => todo!("Unknown instruction {:032b}", raw_instr),
                }
            });

            RefCell::new(m)
        }
    }

    return MATCHER.with(|v| {
        let mut v = v.try_borrow_mut().unwrap();
        if let Some(instr) = v.handle(raw_instr) {
            return instr;
        } else {
            todo!("Unknown instruction {:032b}", raw_instr);
        }
    });
}

fn parse_bitfield(raw_instr: u32) -> AArch64Instr {
    thread_local! {
        pub static MATCHER: RefCell<BitPatternMatcher<AArch64Instr>> = {
            let mut m = BitPatternMatcher::new();
            m.bind("x_xx_100110_x_xxxxxx_xxxxxx_xxxxx_xxxxx",
            |raw_instr: u32,
             sf: Extract<BitRange<31, 32>, u8>,
             opc: Extract<BitRange<29, 31>, u8>,
             n: Extract<BitRange<22, 23>, u8>,
             immr: Extract<BitRange<16, 22>, u8>,
             imms: Extract<BitRange<10, 16>, u8>,
             rn: Extract<BitRange<5, 10>, u8>,
             rd: Extract<BitRange<0, 5>, u8>,
            | {
                let data = Bitfield {
                    immr: immr.value,
                    imms: imms.value,
                    rn: rn.value,
                    rd: rd.value,
                };

                match (sf.value, opc.value, n.value) {
                    (0b0, 0b00, 0b0) => AArch64Instr::Sbfm32(data),
                    (0b0, 0b01, 0b0) => AArch64Instr::Bfm32(data),
                    (0b0, 0b10, 0b0) => AArch64Instr::Ubfm32(data),
                    (0b1, 0b00, 0b1) => AArch64Instr::Sbfm64(data),
                    (0b1, 0b01, 0b1) => AArch64Instr::Bfm64(data),
                    (0b1, 0b10, 0b1) => AArch64Instr::Ubfm64(data),

                    _ => todo!("Unknown instruction {:032b}", raw_instr),
                }
            });

            RefCell::new(m)
        }
    }

    return MATCHER.with(|v| {
        let mut v = v.try_borrow_mut().unwrap();
        if let Some(instr) = v.handle(raw_instr) {
            return instr;
        } else {
            todo!("Unknown instruction {:032b}", raw_instr);
        }
    });
}

fn parse_logical_imm(raw_instr: u32) -> AArch64Instr {
    thread_local! {
        pub static MATCHER: RefCell<BitPatternMatcher<AArch64Instr>> = {
            let mut m = BitPatternMatcher::new();
            m.bind("x_xx_100100_x_xxxxxx_xxxxxx_xxxxx_xxxxx",
            |raw_instr: u32,
             sf: Extract<BitRange<31, 32>, u8>,
             opc: Extract<BitRange<29, 31>, u8>,
             n: Extract<BitRange<22, 23>, u8>,
             immr: Extract<BitRange<16, 22>, u8>,
             imms: Extract<BitRange<10, 16>, u8>,
             rn: Extract<BitRange<5, 10>, u8>,
             rd: Extract<BitRange<0, 5>, u8>,
            | {
                let data = LogicalImm {
                    immr: immr.value,
                    imms: imms.value,
                    rn: rn.value,
                    rd: rd.value,
                };

                match (sf.value, opc.value, n.value) {
                    (0b0, 0b00, 0b0) => AArch64Instr::AndImm32(data),
                    (0b0, 0b01, 0b0) => AArch64Instr::OrrImm32(data),
                    (0b0, 0b10, 0b0) => AArch64Instr::EorImm32(data),
                    (0b0, 0b11, 0b0) => AArch64Instr::AndsImm32(data),
                    (0b1, 0b00, _) => AArch64Instr::AndImm64(data),
                    (0b1, 0b01, _) => AArch64Instr::OrrImm64(data),
                    (0b1, 0b10, _) => AArch64Instr::EorImm64(data),
                    (0b1, 0b11, _) => AArch64Instr::AndsImm64(data),

                    _ => todo!("Unknown instruction {:032b}", raw_instr),
                }
            });

            RefCell::new(m)
        }
    }

    return MATCHER.with(|v| {
        let mut v = v.try_borrow_mut().unwrap();
        if let Some(instr) = v.handle(raw_instr) {
            return instr;
        } else {
            todo!("Unknown instruction {:032b}", raw_instr);
        }
    });
}

fn parse_load_store_reg_pair_offset(raw_instr: u32) -> AArch64Instr {
    thread_local! {
        pub static MATCHER: RefCell<BitPatternMatcher<AArch64Instr>> = {
            let mut m = BitPatternMatcher::new();
            m.bind("xx_101_x_010_x_xxxxxxx_xxxxx_xxxxx_xxxxx",
            |raw_instr: u32,
             opc: Extract<BitRange<30, 32>, u8>,
             v: Extract<BitRange<26, 27>, u8>,
             l: Extract<BitRange<22, 23>, u8>,
             imm7: Extract<BitRange<15, 22>, u8>,
             rt2: Extract<BitRange<10, 15>, u8>,
             rn: Extract<BitRange<5, 10>, u8>,
             rt: Extract<BitRange<0, 5>, u8>,
            | {
                let data = LoadStoreRegPairOffset{
                    imm7: imm7.value,
                    rt2: rt2.value,
                    rn: rn.value,
                    rt: rt.value,
                };

                match (opc.value, v.value, l.value) {
                    (0b00, 0b0, 0b0) => AArch64Instr::Stp32(data),
                    (0b00, 0b0, 0b1) => AArch64Instr::Ldp32(data),
                    (0b00, 0b1, 0b0) => AArch64Instr::StpSimdFP32(data),
                    (0b00, 0b1, 0b1) => AArch64Instr::LdpSimdFP32(data),
                    (0b01, 0b0, 0b0) => AArch64Instr::Stgp(data),
                    (0b01, 0b0, 0b1) => AArch64Instr::Ldpsw(data),
                    (0b01, 0b1, 0b0) => AArch64Instr::StpSimdFP64(data),
                    (0b01, 0b1, 0b1) => AArch64Instr::LdpSimdFP64(data),
                    (0b10, 0b0, 0b0) => AArch64Instr::Stp64(data),
                    (0b10, 0b0, 0b1) => AArch64Instr::Ldp64(data),
                    (0b10, 0b1, 0b0) => AArch64Instr::StpSimdFP128(data),
                    (0b10, 0b1, 0b1) => AArch64Instr::LdpSimdFP128(data),
                    _ => todo!("Unknown instruction {:032b}", raw_instr),
                }
            });

            RefCell::new(m)
        }
    }

    return MATCHER.with(|v| {
        let mut v = v.try_borrow_mut().unwrap();
        if let Some(instr) = v.handle(raw_instr) {
            return instr;
        } else {
            todo!("Unknown instruction {:032b}", raw_instr);
        }
    });
}

fn parse_add_sub_imm_with_tags(raw_instr: u32) -> AArch64Instr {
    thread_local! {
        pub static MATCHER: RefCell<BitPatternMatcher<AArch64Instr>> = {
            let mut m = BitPatternMatcher::new();
            m.bind("x_x_x_100011_x_xxxxxx_xx_xxxx_xxxxx_xxxxx",
            |raw_instr: u32,
             sf_op_s: Extract<BitRange<29, 32>, u8>,
             o2: Extract<BitRange<22, 23>, u8>,
             uimm6: Extract<BitRange<16, 22>, u8>,
             op3: Extract<BitRange<14, 16>, u8>,
             uimm4: Extract<BitRange<10, 14>, u8>,
             rn: Extract<BitRange<5, 10>, u8>,
             rd: Extract<BitRange<0, 5>, u8>,
            | {
                let data = AddSubImmWithTags{
                    o2: o2.value,
                    uimm6: uimm6.value,
                    op3: op3.value,
                    uimm4: uimm4.value,
                    rn: rn.value,
                    rd: rd.value,
                };

                match (sf_op_s.value, o2.value) {
                    (0b100, 0b0) => AArch64Instr::Addg(data),
                    (0b110, 0b0) => AArch64Instr::Subg(data),
                    _ => todo!("Unknown instruction {:032b}", raw_instr),
                }
            });

            RefCell::new(m)
        }
    }

    return MATCHER.with(|v| {
        let mut v = v.try_borrow_mut().unwrap();
        if let Some(instr) = v.handle(raw_instr) {
            return instr;
        } else {
            todo!("Unknown instruction {:032b}", raw_instr);
        }
    });
}

fn parse_extract(raw_instr: u32) -> AArch64Instr {
    thread_local! {
        pub static MATCHER: RefCell<BitPatternMatcher<AArch64Instr>> = {
            let mut m = BitPatternMatcher::new();
            m.bind("x_xx_100111_x_x_xxxxx_xxxxxx_xxxxx_xxxxx",
            |raw_instr: u32,
             sf_op21: Extract<BitRange<29, 32>, u8>,
             n: Extract<BitRange<22, 23>, u8>,
             o0: Extract<BitRange<21, 22>, u8>,
             rm: Extract<BitRange<16, 21>, u8>,
             imms: Extract<BitRange<10, 16>, u8>,
             rn: Extract<BitRange<5, 10>, u8>,
             rd: Extract<BitRange<0, 5>, u8>,
            | {
                let data = ExtractImm{
                    rm: rm.value,
                    imms: imms.value,
                    rn: rn.value,
                    rd: rd.value,
                };

                match (sf_op21.value, n.value, o0.value, imms.value) {
                    (0b000, 0b0, 0b0, imms) if ((imms ^ 0b000000) & 0b100000) == 0b000000 => AArch64Instr::Extr32(data),
                    (0b100, 1, 0, _) => AArch64Instr::Extr64(data),
                    _ => todo!("Unknown instruction {:032b}", raw_instr),
                }
            });

            RefCell::new(m)
        }
    }

    return MATCHER.with(|v| {
        let mut v = v.try_borrow_mut().unwrap();
        if let Some(instr) = v.handle(raw_instr) {
            return instr;
        } else {
            todo!("Unknown instruction {:032b}", raw_instr);
        }
    });
}

fn parse_cmp_and_branch_imm(raw_instr: u32) -> AArch64Instr {
    thread_local! {
        pub static MATCHER: RefCell<BitPatternMatcher<AArch64Instr>> = {
            let mut m = BitPatternMatcher::new();
            m.bind("x_011010_x_xxxxxxxxxxxxxxxxxxx_xxxxx",
            |raw_instr: u32,
             sf: Extract<BitRange<31, 32>, u8>,
             op: Extract<BitRange<24, 25>, u8>,
             imm19: Extract<BitRange<5, 24>, u32>,
             rt: Extract<BitRange<0, 5>, u8>,
            | {
                let data = CmpAndBranchImm{
                    imm19: imm19.value,
                    rt: rt.value,
                };

                match (sf.value, op.value) {
                    (0b0, 0b0) => AArch64Instr::Cbz32(data),
                    (0b0, 0b1) => AArch64Instr::Cbnz32(data),
                    (0b1, 0b0) => AArch64Instr::Cbz64(data),
                    (0b1, 0b1) => AArch64Instr::Cbnz64(data),
                    _ => todo!("Unknown instruction {:032b}", raw_instr),
                }
            });

            RefCell::new(m)
        }
    }

    return MATCHER.with(|v| {
        let mut v = v.try_borrow_mut().unwrap();
        if let Some(instr) = v.handle(raw_instr) {
            return instr;
        } else {
            todo!("Unknown instruction {:032b}", raw_instr);
        }
    });
}

fn parse_data_proccessing_3src(raw_instr: u32) -> AArch64Instr {
    thread_local! {
        pub static MATCHER: RefCell<BitPatternMatcher<AArch64Instr>> = {
            let mut m = BitPatternMatcher::new();
            m.bind("x_xx_11011_xxx_xxxxx_x_xxxxx_xxxxx_xxxxx",
            |raw_instr: u32,
             sf: Extract<BitRange<31, 32>, u8>,
             op54: Extract<BitRange<29, 31>, u8>,
             op31: Extract<BitRange<21, 24>, u8>,
             rm: Extract<BitRange<16, 21>, u8>,
             o0: Extract<BitRange<15, 16>, u8>,
             ra: Extract<BitRange<10, 15>, u8>,
             rn: Extract<BitRange<5, 10>, u8>,
             rd: Extract<BitRange<0, 5>, u8>,
            | {
                let data = DataProc3Src{
                    rm: rm.value,
                    ra: ra.value,
                    rn: rn.value,
                    rd: rd.value,
                };

                match (sf.value, op54.value, op31.value, o0.value) {
                    (0b0, 0b00, 0b000, 0b0) => AArch64Instr::Madd32(data),
                    (0b0, 0b00, 0b000, 0b1) => AArch64Instr::Msub32(data),
                    (0b1, 0b00, 0b000, 0b0) => AArch64Instr::Madd64(data),
                    (0b1, 0b00, 0b000, 0b1) => AArch64Instr::Msub64(data),
                    (0b1, 0b00, 0b001, 0b0) => AArch64Instr::Smaddl(data),
                    (0b1, 0b00, 0b001, 0b1) => AArch64Instr::Smsubl(data),
                    (0b1, 0b00, 0b010, 0b0) => AArch64Instr::Smulh(data),
                    (0b1, 0b00, 0b101, 0b0) => AArch64Instr::Umaddl(data),
                    (0b1, 0b00, 0b101, 0b1) => AArch64Instr::Umsubl(data),
                    (0b1, 0b00, 0b111, 0b0) => AArch64Instr::Umulh(data),
                    _ => todo!("Unknown instruction {:032b}", raw_instr),
                }
            });

            RefCell::new(m)
        }
    }

    return MATCHER.with(|v| {
        let mut v = v.try_borrow_mut().unwrap();
        if let Some(instr) = v.handle(raw_instr) {
            return instr;
        } else {
            todo!("Unknown instruction {:032b}", raw_instr);
        }
    });
}
