use core::ir::BasicBlock;
use core::Instruction;
use std::fmt::Debug;

use crate::aarch64::inst_operand::*;
use crate::aarch64::{compile_aarch64_to_ir, decode_aarch64_inst};

// AArch64 instruction
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AArch64Inst {
    AddImm32(ShImm12RnRd),
    AddsImm32(ShImm12RnRd),
    SubImm32(ShImm12RnRd),
    SubsImm32(ShImm12RnRd),
    AddImm64(ShImm12RnRd),
    AddsImm64(ShImm12RnRd),
    SubImm64(ShImm12RnRd),
    SubsImm64(ShImm12RnRd),

    AndImm32(LogicalImm),
    OrrImm32(LogicalImm),
    EorImm32(LogicalImm),
    AndsImm32(LogicalImm),
    AndImm64(LogicalImm),
    OrrImm64(LogicalImm),
    EorImm64(LogicalImm),
    AndsImm64(LogicalImm),

    Addg(AddSubImmWithTags),
    Subg(AddSubImmWithTags),

    Extr32(ExtractImm),
    Extr64(ExtractImm),

    Clrex(Barriers),
    DsbEncoding(Barriers),
    Dmb(Barriers),
    Isb(Barriers),

    Sbfm32(Bitfield),
    Bfm32(Bitfield),
    Ubfm32(Bitfield),
    Sbfm64(Bitfield),
    Bfm64(Bitfield),
    Ubfm64(Bitfield),

    AddShiftedReg32(ShiftRmImm6RnRd),
    AddsShiftedReg32(ShiftRmImm6RnRd),
    SubShiftedReg32(ShiftRmImm6RnRd),
    SubsShiftedReg32(ShiftRmImm6RnRd),
    AddShiftedReg64(ShiftRmImm6RnRd),
    AddsShiftedReg64(ShiftRmImm6RnRd),
    SubShiftedReg64(ShiftRmImm6RnRd),
    SubsShiftedReg64(ShiftRmImm6RnRd),

    AddExtReg32(AddSubtractExtReg),
    AddsExtReg32(AddSubtractExtReg),
    SubExtReg32(AddSubtractExtReg),
    SubsExtReg32(AddSubtractExtReg),
    AddExtReg64(AddSubtractExtReg),
    AddsExtReg64(AddSubtractExtReg),
    SubExtReg64(AddSubtractExtReg),
    SubsExtReg64(AddSubtractExtReg),

    AdcVar32(RmRnRd),
    AdcsVar32(RmRnRd),
    SbcVar32(RmRnRd),
    SbcsVar32(RmRnRd),
    AdcVar64(RmRnRd),
    AdcsVar64(RmRnRd),
    SbcVar64(RmRnRd),
    SbcsVar64(RmRnRd),

    FmAddSinglePrecision(RmRaRnRd),
    FmSubSinglePrecision(RmRaRnRd),
    FnmAddSinglePrecision(RmRaRnRd),
    FnmSubSinglePrecision(RmRaRnRd),
    FmAddDoublePrecision(RmRaRnRd),
    FmSubDoublePrecision(RmRaRnRd),
    FnmAddDoublePrecision(RmRaRnRd),
    FnmSubDoublePrecision(RmRaRnRd),
    FmAddHalfPrecision(RmRaRnRd),
    FmSubHalfPrecision(RmRaRnRd),
    FnmAddHalfPrecision(RmRaRnRd),
    FnmSubHalfPrecision(RmRaRnRd),

    CaspVar32(CompareAndSwapPair),
    CasplVar32(CompareAndSwapPair),
    CaspaVar32(CompareAndSwapPair),
    CaspalVar32(CompareAndSwapPair),
    CaspVar64(CompareAndSwapPair),
    CasplVar64(CompareAndSwapPair),
    CaspaVar64(CompareAndSwapPair),
    CaspalVar64(CompareAndSwapPair),

    StgEncoding(LoadStoreMemoryTags),
    Stzgm(LoadStoreMemoryTags),
    Ldg(LoadStoreMemoryTags),
    StzgEncoding(LoadStoreMemoryTags),
    St2gEncoding(LoadStoreMemoryTags),
    Stgm(LoadStoreMemoryTags),
    Stz2gEncoding(LoadStoreMemoryTags),
    Ldgm(LoadStoreMemoryTags),

    StxpVar32(LoadStoreExclusivePair),
    StlxpVar32(LoadStoreExclusivePair),
    LdxpVar32(LoadStoreExclusivePair),
    LdaxpVar32(LoadStoreExclusivePair),
    StxpVar64(LoadStoreExclusivePair),
    StlxpVar64(LoadStoreExclusivePair),
    LdxpVar64(LoadStoreExclusivePair),
    LdaxpVar64(LoadStoreExclusivePair),

    Stlurb(Imm9RnRt),
    Ldapurb(Imm9RnRt),
    LdapursbVar64(Imm9RnRt),
    LdapursbVar32(Imm9RnRt),
    Stlurh(Imm9RnRt),
    Ldapurh(Imm9RnRt),
    LdapurshVar64(Imm9RnRt),
    LdapurshVar32(Imm9RnRt),
    StlurVar32(Imm9RnRt),
    LdapurVar32(Imm9RnRt),
    Ldapursw(Imm9RnRt),
    StlurVar64(Imm9RnRt),
    LdapurVar64(Imm9RnRt),

    StnpVar32(LdStNoAllocPairOffset),
    LdnpVar32(LdStNoAllocPairOffset),
    StnpSimdFPVar32(LdStNoAllocPairOffset),
    LdnpSimdFPVar32(LdStNoAllocPairOffset),
    StnpSimdFPVar64(LdStNoAllocPairOffset),
    LdnpSimdFPVar64(LdStNoAllocPairOffset),
    StnpVar64(LdStNoAllocPairOffset),
    LdnpVar64(LdStNoAllocPairOffset),
    StnpSimdFPVar128(LdStNoAllocPairOffset),
    LdnpSimdFPVar128(LdStNoAllocPairOffset),

    Sttrb(Imm9RnRt),
    Ldtrb(Imm9RnRt),
    LdtrsbVar64(Imm9RnRt),
    LdtrsbVar32(Imm9RnRt),
    Sttrh(Imm9RnRt),
    Ldtrh(Imm9RnRt),
    LdtrshVar64(Imm9RnRt),
    LdtrshVar32(Imm9RnRt),
    SttrVar32(Imm9RnRt),
    LdtrVar32(Imm9RnRt),
    Ldtrsw(Imm9RnRt),
    SttrVar64(Imm9RnRt),
    LdtrVar64(Imm9RnRt),

    StrbImm(OpcSizeImm12RnRt),
    LdrbImm(OpcSizeImm12RnRt),
    LdrsbImm32(OpcSizeImm12RnRt),
    LdrsbImm64(OpcSizeImm12RnRt),
    StrImmSimdFP8(OpcSizeImm12RnRt),
    LdrImmSimdFP8(OpcSizeImm12RnRt),
    StrImmSimdFP128(OpcSizeImm12RnRt),
    LdrImmSimdFP128(OpcSizeImm12RnRt),
    StrhImm(OpcSizeImm12RnRt),
    LdrhImm(OpcSizeImm12RnRt),
    LdrshImm32(OpcSizeImm12RnRt),
    LdrshImm64(OpcSizeImm12RnRt),
    StrImmSimdFP16(OpcSizeImm12RnRt),
    LdrImmSimdFP16(OpcSizeImm12RnRt),
    StrImm32(OpcSizeImm12RnRt),
    LdrImm32(OpcSizeImm12RnRt),
    LdrswImm(OpcSizeImm12RnRt),
    StrImmSimdFP32(OpcSizeImm12RnRt),
    LdrImmSimdFP32(OpcSizeImm12RnRt),
    StrImm64(OpcSizeImm12RnRt),
    LdrImm64(OpcSizeImm12RnRt),
    PrfmImm(OpcSizeImm12RnRt),
    StrImmSimdFP64(OpcSizeImm12RnRt),
    LdrImmSimdFP64(OpcSizeImm12RnRt),

    StrbRegExtReg(LoadStoreRegRegOffset),
    StrbRegShiftedReg(LoadStoreRegRegOffset),
    LdrbRegExtReg(LoadStoreRegRegOffset),
    LdrbRegShiftedReg(LoadStoreRegRegOffset),
    LdrsbRegExtReg64(LoadStoreRegRegOffset),
    LdrsbRegShiftedReg64(LoadStoreRegRegOffset),
    LdrsbRegExtReg32(LoadStoreRegRegOffset),
    LdrsbRegShiftedReg32(LoadStoreRegRegOffset),
    StrRegSimdFP(LoadStoreRegRegOffset),
    LdrRegSimdFP(LoadStoreRegRegOffset),
    StrhReg(LoadStoreRegRegOffset),
    LdrhReg(LoadStoreRegRegOffset),
    LdrshReg64(LoadStoreRegRegOffset),
    LdrshReg32(LoadStoreRegRegOffset),
    StrReg32(LoadStoreRegRegOffset),
    LdrReg32(LoadStoreRegRegOffset),
    LdrswReg(LoadStoreRegRegOffset),
    StrReg64(LoadStoreRegRegOffset),
    LdrReg64(LoadStoreRegRegOffset),
    PrfmReg(LoadStoreRegRegOffset),

    Stgp(LoadStoreRegPair),
    Ldpsw(LoadStoreRegPair),

    Sturb(LdStRegUnscaledImm),
    Ldurb(LdStRegUnscaledImm),
    Ldursb64(LdStRegUnscaledImm),
    Ldursb32(LdStRegUnscaledImm),
    SturSimdFP8(LdStRegUnscaledImm),
    LdurSimdFP8(LdStRegUnscaledImm),
    SturSimdFP128(LdStRegUnscaledImm),
    LdurSimdFP128(LdStRegUnscaledImm),
    Sturh(LdStRegUnscaledImm),
    Ldurh(LdStRegUnscaledImm),
    Ldursh64(LdStRegUnscaledImm),
    Ldursh32(LdStRegUnscaledImm),
    SturSimdFP16(LdStRegUnscaledImm),
    LdurSimdFP16(LdStRegUnscaledImm),
    Stur32(LdStRegUnscaledImm),
    Ldur32(LdStRegUnscaledImm),
    Ldursw(LdStRegUnscaledImm),
    SturSimdFP32(LdStRegUnscaledImm),
    LdurSimdFP32(LdStRegUnscaledImm),
    Stur64(LdStRegUnscaledImm),
    Ldur64(LdStRegUnscaledImm),
    Prefum(LdStRegUnscaledImm),
    SturSimdFP64(LdStRegUnscaledImm),
    LdurSimdFP64(LdStRegUnscaledImm),

    StpVar32(LoadStoreRegPair),
    LdpVar32(LoadStoreRegPair),
    StpSimdFPVar32(LoadStoreRegPair),
    LdpSimdFPVar32(LoadStoreRegPair),
    StpSimdFPVar64(LoadStoreRegPair),
    LdpSimdFPVar64(LoadStoreRegPair),
    StpVar64(LoadStoreRegPair),
    LdpVar64(LoadStoreRegPair),
    StpSimdFpVar128(LoadStoreRegPair),
    LdpSimdFpVar128(LoadStoreRegPair),

    Stxrb(RsRt2RnRt),
    Ldxrb(RsRt2RnRt),
    Stxrh(RsRt2RnRt),
    Ldxrh(RsRt2RnRt),
    StxrVar32(RsRt2RnRt),
    LdxrVar32(RsRt2RnRt),
    StxrVar64(RsRt2RnRt),
    LdxrVar64(RsRt2RnRt),
    Stlxrb(RsRt2RnRt),
    Ldaxrb(RsRt2RnRt),
    Stlxrh(RsRt2RnRt),
    Ldaxrh(RsRt2RnRt),
    StlxrVar32(RsRt2RnRt),
    LdaxrVar32(RsRt2RnRt),
    StlxrVar64(RsRt2RnRt),
    LdaxrVar64(RsRt2RnRt),

    Stlrb(RsRt2RnRt),
    Ldarb(RsRt2RnRt),
    Stlrh(RsRt2RnRt),
    Ldarh(RsRt2RnRt),
    StlrVar32(RsRt2RnRt),
    LdarVar32(RsRt2RnRt),
    StlrVar64(RsRt2RnRt),
    LdarVar64(RsRt2RnRt),

    LdrLitVar32(Imm19Rt),
    LdrLitSimdFPVar32(Imm19Rt),
    LdrLitVar64(Imm19Rt),
    LdrLitSimdFPVar64(Imm19Rt),
    LdrswLit(Imm19Rt),
    LdrLitSimdFPVar128(Imm19Rt),
    PrfmLit(Imm19Rt),

    BImm(Imm26),
    BlImm(Imm26),

    BCond(Imm19Cond),
    BcCond(Imm19Cond),

    Tbz(B5B40Imm14Rt),
    Tbnz(B5B40Imm14Rt),

    Cbz32(Imm19Rt),
    Cbnz32(Imm19Rt),
    Cbz64(Imm19Rt),
    Cbnz64(Imm19Rt),

    Wfet(Rt),
    Wfit(Rt),

    MsrImm(PstateOp),
    Cfinv(PstateOp),
    Xaflag(PstateOp),
    Axflag(PstateOp),

    Tstart(Rt),
    Ttest(Rt),

    Sys(SystemInstructions),
    Sysl(SystemInstructions),

    MsrReg(SysRegMov),
    Mrs(SysRegMov),

    Csel32(RmCondRnRd),
    Csinc32(RmCondRnRd),
    Csinv32(RmCondRnRd),
    Csneg32(RmCondRnRd),
    Csel64(RmCondRnRd),
    Csinc64(RmCondRnRd),
    Csinv64(RmCondRnRd),
    Csneg64(RmCondRnRd),

    MovnVar32(HwImm16Rd),
    MovzVar32(HwImm16Rd),
    MovkVar32(HwImm16Rd),
    MovnVar64(HwImm16Rd),
    MovzVar64(HwImm16Rd),
    MovkVar64(HwImm16Rd),

    AndShiftedReg32(ShiftRmImm6RnRd),
    BicShiftedReg32(ShiftRmImm6RnRd),
    OrrShiftedReg32(ShiftRmImm6RnRd),
    OrnShiftedReg32(ShiftRmImm6RnRd),
    EorShiftedReg32(ShiftRmImm6RnRd),
    EonShiftedReg32(ShiftRmImm6RnRd),
    AndsShiftedReg32(ShiftRmImm6RnRd),
    BicsShiftedReg32(ShiftRmImm6RnRd),

    AndShiftedReg64(ShiftRmImm6RnRd),
    BicShiftedReg64(ShiftRmImm6RnRd),
    OrrShiftedReg64(ShiftRmImm6RnRd),
    OrnShiftedReg64(ShiftRmImm6RnRd),
    EorShiftedReg64(ShiftRmImm6RnRd),
    EonShiftedReg64(ShiftRmImm6RnRd),
    AndsShiftedReg64(ShiftRmImm6RnRd),
    BicsShiftedReg64(ShiftRmImm6RnRd),

    Madd32(DataProc3Src),
    Msub32(DataProc3Src),
    Madd64(DataProc3Src),
    Msub64(DataProc3Src),
    Smaddl(DataProc3Src),
    Smsubl(DataProc3Src),
    Smulh(DataProc3Src),
    Umaddl(DataProc3Src),
    Umsubl(DataProc3Src),
    Umulh(DataProc3Src),

    UdivVar32(DataProc2Src),
    SdivVar32(DataProc2Src),
    LslvVar32(DataProc2Src),
    LsrvVar32(DataProc2Src),
    AsrvVar32(DataProc2Src),
    RorvVar32(DataProc2Src),
    UdivVar64(DataProc2Src),
    SdivVar64(DataProc2Src),
    LslvVar64(DataProc2Src),
    LsrvVar64(DataProc2Src),
    AsrvVar64(DataProc2Src),
    RorvVar64(DataProc2Src),
    Pacga(DataProc2Src),

    Rmif(RotateRightIntoFlags),

    SetfVar8(Rn),
    SetfVar16(Rn),

    CcmnRegVar32(CondCmpReg),
    CcmpRegVar32(CondCmpReg),
    CcmnRegVar64(CondCmpReg),
    CcmpRegVar64(CondCmpReg),

    CcmnImmVar32(CondCmpImm),
    CcmpImmVar32(CondCmpImm),
    CcmnImmVar64(CondCmpImm),
    CcmpImmVar64(CondCmpImm),

    RbitVar32(RnRd),
    Rev16Var32(RnRd),
    RevVar32(RnRd),
    ClzVar32(RnRd),
    ClsVar32(RnRd),
    RbitVar64(RnRd),
    Rev16Var64(RnRd),
    Rev32(RnRd),
    RevVar64(RnRd),
    ClzVar64(RnRd),
    ClsVar64(RnRd),

    Br(UncondBranchReg),
    Blr(UncondBranchReg),
    Ret(UncondBranchReg),
    ERet(UncondBranchReg),
    Drps(UncondBranchReg),

    Hint,
    Nop,
    Yield,
    Wfe,
    Wfi,
    Sev,
    Sevl,
    Xpaclri,
    Pacia1716Var,
    Pacib1716Var,
    Autia1716Var,
    Autib1716Var,
    PaciazVar,
    PaciaspVar,
    PacibzVar,
    PacibspVar,
    AutiazVar,
    AutiaspVar,
    AutibzVar,
    AutibspVar,

    Adr(PcRelAddressing),
    Adrp(PcRelAddressing),

    Svc(ExceptionGen),
    Hvc(ExceptionGen),
    Smc(ExceptionGen),
    Brk(ExceptionGen),
    Hlt(ExceptionGen),
    TCancle(ExceptionGen),
    DcpS1(ExceptionGen),
    DcpS2(ExceptionGen),
    DcpS3(ExceptionGen),

    DupElement(AdvancedSimdCopy),
    DupGeneral(AdvancedSimdCopy),
    Smov(AdvancedSimdCopy),
    Umov(AdvancedSimdCopy),
    InsGeneral(AdvancedSimdCopy),
    InsElement(AdvancedSimdCopy),

    St1SingleStructureVar8(AdvSimdLdStSingleStructure),
    St3SingleStructureVar8(AdvSimdLdStSingleStructure),
    St1SingleStructureVar16(AdvSimdLdStSingleStructure),
    St3SingleStructureVar16(AdvSimdLdStSingleStructure),
    St1SingleStructureVar32(AdvSimdLdStSingleStructure),
    St1SingleStructureVar64(AdvSimdLdStSingleStructure),
    St3SingleStructureVar32(AdvSimdLdStSingleStructure),
    St3SingleStructureVar64(AdvSimdLdStSingleStructure),
    St2SingleStructureVar8(AdvSimdLdStSingleStructure),
    St4SingleStructureVar8(AdvSimdLdStSingleStructure),
    St2SingleStructureVar16(AdvSimdLdStSingleStructure),
    St4SingleStructureVar16(AdvSimdLdStSingleStructure),
    St2SingleStructureVar32(AdvSimdLdStSingleStructure),
    St2SingleStructureVar64(AdvSimdLdStSingleStructure),
    St4SingleStructureVar32(AdvSimdLdStSingleStructure),
    St4SingleStructureVar64(AdvSimdLdStSingleStructure),

    Ld1SingleStructureVar8(AdvSimdLdStSingleStructure),
    Ld3SingleStructureVar8(AdvSimdLdStSingleStructure),
    Ld1SingleStructureVar16(AdvSimdLdStSingleStructure),
    Ld3SingleStructureVar16(AdvSimdLdStSingleStructure),
    Ld1SingleStructureVar32(AdvSimdLdStSingleStructure),
    Ld1SingleStructureVar64(AdvSimdLdStSingleStructure),
    Ld3SingleStructureVar32(AdvSimdLdStSingleStructure),
    Ld3SingleStructureVar64(AdvSimdLdStSingleStructure),
    Ld1r(AdvSimdLdStSingleStructure),
    Ld3r(AdvSimdLdStSingleStructure),
    Ld2SingleStructureVar8(AdvSimdLdStSingleStructure),
    Ld4SingleStructureVar8(AdvSimdLdStSingleStructure),
    Ld2SingleStructureVar16(AdvSimdLdStSingleStructure),
    Ld4SingleStructureVar16(AdvSimdLdStSingleStructure),
    Ld2SingleStructureVar32(AdvSimdLdStSingleStructure),
    Ld2SingleStructureVar64(AdvSimdLdStSingleStructure),
    Ld4SingleStructureVar32(AdvSimdLdStSingleStructure),
    Ld4SingleStructureVar64(AdvSimdLdStSingleStructure),
    Ld2r(AdvSimdLdStSingleStructure),
    Ld4r(AdvSimdLdStSingleStructure),

    St4MulStructures(AdvSimdLdStMultiStructures),
    St1MulStructures4RegsVar(AdvSimdLdStMultiStructures),
    St3MulStructures(AdvSimdLdStMultiStructures),
    St1MulStructures3RegsVar(AdvSimdLdStMultiStructures),
    St1MulStructures1RegsVar(AdvSimdLdStMultiStructures),
    St2MulStructures(AdvSimdLdStMultiStructures),
    St1MulStructures2RegsVar(AdvSimdLdStMultiStructures),
    Ld4MulStructures(AdvSimdLdStMultiStructures),
    Ld1MulStructures4RegsVar(AdvSimdLdStMultiStructures),
    Ld3MulStructures(AdvSimdLdStMultiStructures),
    Ld1MulStructures3RegsVar(AdvSimdLdStMultiStructures),
    Ld1MulStructures1RegsVar(AdvSimdLdStMultiStructures),
    Ld2MulStructures(AdvSimdLdStMultiStructures),
    Ld1MulStructures2RegsVar(AdvSimdLdStMultiStructures),

    St4MulStructuresRegOffsetVar(AdvSimdLdStMultiStructuresPostIndexed),
    St1MulStructures4RegRegOffsetVar(AdvSimdLdStMultiStructuresPostIndexed),
    St3MulStructuresRegOffsetVar(AdvSimdLdStMultiStructuresPostIndexed),
    St1MulStructures3RegRegOffsetVar(AdvSimdLdStMultiStructuresPostIndexed),
    St1MulStructures1RegRegOffsetVar(AdvSimdLdStMultiStructuresPostIndexed),
    St2MulStructuresRegOffsetVar(AdvSimdLdStMultiStructuresPostIndexed),
    St1MulStructures2RegRegOffsetVar(AdvSimdLdStMultiStructuresPostIndexed),
    St4MulStructuresImmOffsetVar(AdvSimdLdStMultiStructuresPostIndexed),
    St1MulStructures4RegImmOffsetVar(AdvSimdLdStMultiStructuresPostIndexed),
    St3MulStructuresImmOffsetVar(AdvSimdLdStMultiStructuresPostIndexed),
    St1MulStructures3RegImmOffsetVar(AdvSimdLdStMultiStructuresPostIndexed),
    St1MulStructures1RegImmOffsetVar(AdvSimdLdStMultiStructuresPostIndexed),
    St2MulStructuresImmOffsetVar(AdvSimdLdStMultiStructuresPostIndexed),
    St1MulStructures2RegImmOffsetVar(AdvSimdLdStMultiStructuresPostIndexed),

    Ld4MulStructuresRegOffsetVar(AdvSimdLdStMultiStructuresPostIndexed),
    Ld1MulStructures4RegRegOffsetVar(AdvSimdLdStMultiStructuresPostIndexed),
    Ld3MulStructuresRegOffsetVar(AdvSimdLdStMultiStructuresPostIndexed),
    Ld1MulStructures3RegRegOffsetVar(AdvSimdLdStMultiStructuresPostIndexed),
    Ld1MulStructures1RegRegOffsetVar(AdvSimdLdStMultiStructuresPostIndexed),
    Ld2MulStructuresRegOffsetVar(AdvSimdLdStMultiStructuresPostIndexed),
    Ld1MulStructures2RegRegOffsetVar(AdvSimdLdStMultiStructuresPostIndexed),
    Ld4MulStructuresImmOffsetVar(AdvSimdLdStMultiStructuresPostIndexed),
    Ld1MulStructures4RegImmOffsetVar(AdvSimdLdStMultiStructuresPostIndexed),
    Ld3MulStructuresImmOffsetVar(AdvSimdLdStMultiStructuresPostIndexed),
    Ld1MulStructures3RegImmOffsetVar(AdvSimdLdStMultiStructuresPostIndexed),
    Ld1MulStructures1RegImmOffsetVar(AdvSimdLdStMultiStructuresPostIndexed),
    Ld2MulStructuresImmOffsetVar(AdvSimdLdStMultiStructuresPostIndexed),
    Ld1MulStructures2RegImmOffsetVar(AdvSimdLdStMultiStructuresPostIndexed),

    FcvtnsScalarSinglePrecisionTo32(RnRd),
    FcvtnuScalarSinglePrecisionTo32(RnRd),
    ScvtfScalarInt32ToSinglePrecision(RnRd),
    UcvtfScalarInt32ToSinglePrecision(RnRd),
    FcvtasScalarSinglePrecisionTo32(RnRd),
    FcvtauScalarSinglePrecisionTo32(RnRd),
    FmovGeneralSinglePrecisionTo32(RnRd),
    FmovGeneral32ToSinglePrecision(RnRd),
    FcvtpsScalarSinglePrecisionTo32(RnRd),
    FcvtpuScalarSinglePrecisionTo32(RnRd),
    FcvtmsScalarSinglePrecisionTo32(RnRd),
    FcvtmuScalarSinglePrecisionTo32(RnRd),
    FcvtzsScalarIntSinglePrecisionTo32(RnRd),
    FcvtzuScalarIntSinglePrecisionTo32(RnRd),
    FcvtnsScalarDoublePrecisionTo32(RnRd),
    FcvtnuScalarDoublePrecisionTo32(RnRd),
    ScvtfScalarInt32ToDoublePrecision(RnRd),
    UcvtfScalarInt32ToDoublePrecision(RnRd),
    FcvtasScalarDoublePrecisionTo32(RnRd),
    FcvtauScalarDoublePrecisionTo32(RnRd),
    FcvtpsScalarDoublePrecisionTo32(RnRd),
    FcvtpuScalarDoublePrecisionTo32(RnRd),
    FcvtmsScalarDoublePrecisionTo32(RnRd),
    FcvtmuScalarDoublePrecisionTo32(RnRd),
    FcvtzsScalarIntDoublePrecisionTo32(RnRd),
    FcvtzuScalarIntDoublePrecisionTo32(RnRd),
    Fjcvtzs(RnRd),
    FcvtnsScalarSinglePrecisionTo64(RnRd),
    FcvtnuScalarSinglePrecisionTo64(RnRd),
    ScvtfScalarInt64ToSinglePrecision(RnRd),
    UcvtfScalarInt64ToSinglePrecision(RnRd),
    FcvtasScalarSinglePrecisionTo64(RnRd),
    FcvtauScalarSinglePrecisionTo64(RnRd),
    FcvtpsScalarSinglePrecisionTo64(RnRd),
    FcvtpuScalarSinglePrecisionTo64(RnRd),
    FcvtmsScalarSinglePrecisionTo64(RnRd),
    FcvtmuScalarSinglePrecisionTo64(RnRd),
    FcvtzsScalarIntSinglePrecisionTo64(RnRd),
    FcvtzuScalarIntSinglePrecisionTo64(RnRd),
    FcvtnsScalarDoublePrecisionTo64(RnRd),
    FcvtnuScalarDoublePrecisionTo64(RnRd),
    ScvtfScalarInt64ToDoublePrecision(RnRd),
    UcvtfScalarInt64ToDoublePrecision(RnRd),
    FcvtasScalarDoublePrecisionTo64(RnRd),
    FcvtauScalarDoublePrecisionTo64(RnRd),
    FmovGeneralDoublePrecisionTo64(RnRd),
    FmovGeneral64ToDoublePrecision(RnRd),
    FcvtpsScalarDoublePrecisionTo64(RnRd),
    FcvtpuScalarDoublePrecisionTo64(RnRd),
    FcvtmsScalarDoublePrecisionTo64(RnRd),
    FcvtmuScalarDoublePrecisionTo64(RnRd),
    FcvtzsScalarIntDoublePrecisionTo64(RnRd),
    FcvtzuScalarIntDoublePrecisionTo64(RnRd),
    FmovGeneralTopHalfOf128To64(RnRd),
    FmovGeneral64toTopHalfOf128(RnRd),

    MoviShiftedImmVar32(AdvSimdModifiedImm),
    OrrVecImmVar32(AdvSimdModifiedImm),
    MoviShiftedImmVar16(AdvSimdModifiedImm),
    OrrVecImmVar16(AdvSimdModifiedImm),
    MoviShiftingOnesVar32(AdvSimdModifiedImm),
    MoviVar8(AdvSimdModifiedImm),
    FmovVecImmSinglePrecisionVar(AdvSimdModifiedImm),
    MvniShiftedImmVar32(AdvSimdModifiedImm),
    BicVecImmVar32(AdvSimdModifiedImm),
    MvniShiftedImmVar16(AdvSimdModifiedImm),
    BicVecImmVar16(AdvSimdModifiedImm),
    MvniShiftingOnesVar32(AdvSimdModifiedImm),
    MoviScalarVar64(AdvSimdModifiedImm),
    MoviVectorVar64(AdvSimdModifiedImm),
    FmovVecImmDoublePrecisionVar(AdvSimdModifiedImm),

    Ext(AdvancedSimdExtract),

    Shadd(QSizeRmRnRd),
    Sqadd(QSizeRmRnRd),
    Srhadd(QSizeRmRnRd),
    Shsub(QSizeRmRnRd),
    Sqsub(QSizeRmRnRd),
    CmgtReg(QSizeRmRnRd),
    CmgeReg(QSizeRmRnRd),
    Sshl(QSizeRmRnRd),
    SqshlReg(QSizeRmRnRd),
    Srshl(QSizeRmRnRd),
    Sqrshl(QSizeRmRnRd),
    Smax(QSizeRmRnRd),
    Smin(QSizeRmRnRd),
    Sabd(QSizeRmRnRd),
    Saba(QSizeRmRnRd),
    AddVec(QSizeRmRnRd),
    Cmtst(QSizeRmRnRd),
    MlaVec(QSizeRmRnRd),
    MulVec(QSizeRmRnRd),
    Smaxp(QSizeRmRnRd),
    Sminp(QSizeRmRnRd),
    SqdmulhVec(QSizeRmRnRd),
    AddpVec(QSizeRmRnRd),
    FmaxnmVec(QSizeRmRnRd),
    FmlaVec(QSizeRmRnRd),
    FaddVec(QSizeRmRnRd),
    Fmulx(QSizeRmRnRd),
    FcmeqReg(QSizeRmRnRd),
    FmaxVec(QSizeRmRnRd),
    Frecps(QSizeRmRnRd),
    AndVec(QSizeRmRnRd),
    BicVecReg(QSizeRmRnRd),
    FminnmVec(QSizeRmRnRd),
    FmlsVec(QSizeRmRnRd),
    FsubVec(QSizeRmRnRd),
    FminVec(QSizeRmRnRd),
    Frsqrts(QSizeRmRnRd),
    OrrVecReg(QSizeRmRnRd),
    OrnVec(QSizeRmRnRd),
    Uhadd(QSizeRmRnRd),
    Uqadd(QSizeRmRnRd),
    Urhadd(QSizeRmRnRd),
    Uhsub(QSizeRmRnRd),
    Uqsub(QSizeRmRnRd),
    CmhiReg(QSizeRmRnRd),
    CmhsReg(QSizeRmRnRd),
    Ushl(QSizeRmRnRd),
    UqshlReg(QSizeRmRnRd),
    Urshl(QSizeRmRnRd),
    Uqrshl(QSizeRmRnRd),
    Umax(QSizeRmRnRd),
    Umin(QSizeRmRnRd),
    Uabd(QSizeRmRnRd),
    Uaba(QSizeRmRnRd),
    SubVec(QSizeRmRnRd),
    CmeqReg(QSizeRmRnRd),
    MlsVec(QSizeRmRnRd),
    Pmul(QSizeRmRnRd),
    Umaxp(QSizeRmRnRd),
    Uminp(QSizeRmRnRd),
    SqrdmulhVec(QSizeRmRnRd),
    FmaxnmpVec(QSizeRmRnRd),
    FaddpVec(QSizeRmRnRd),
    FmulVec(QSizeRmRnRd),
    FcmgeReg(QSizeRmRnRd),
    Facge(QSizeRmRnRd),
    FmaxpVec(QSizeRmRnRd),
    FdivVec(QSizeRmRnRd),
    EorVec(QSizeRmRnRd),
    Bsl(QSizeRmRnRd),
    FminnmpVec(QSizeRmRnRd),
    Fabd(QSizeRmRnRd),
    FcmgtReg(QSizeRmRnRd),
    Facgt(QSizeRmRnRd),
    FminpVec(QSizeRmRnRd),
    Bit(QSizeRmRnRd),
    Bif(QSizeRmRnRd),

    Sshr(AdvSimdShiftByImm),
    Ssra(AdvSimdShiftByImm),
    Srshr(AdvSimdShiftByImm),
    Srsra(AdvSimdShiftByImm),
    Shl(AdvSimdShiftByImm),
    SqshlImm(AdvSimdShiftByImm),
    Shrn(AdvSimdShiftByImm),
    Rshrn(AdvSimdShiftByImm),
    Sqshrn(AdvSimdShiftByImm),
    Sqrshrn(AdvSimdShiftByImm),
    Sshll(AdvSimdShiftByImm),
    ScvtfVecFixedPt(AdvSimdShiftByImm),
    FcvtzsVecFixedPt(AdvSimdShiftByImm),
    Ushr(AdvSimdShiftByImm),
    Usra(AdvSimdShiftByImm),
    Urshr(AdvSimdShiftByImm),
    Ursra(AdvSimdShiftByImm),
    Sri(AdvSimdShiftByImm),
    Sli(AdvSimdShiftByImm),
    Sqshlu(AdvSimdShiftByImm),
    UqshlImm(AdvSimdShiftByImm),
    Sqshrun(AdvSimdShiftByImm),
    Sqrshrun(AdvSimdShiftByImm),
    Uqshrn(AdvSimdShiftByImm),
    Uqrshrn(AdvSimdShiftByImm),
    Ushll(AdvSimdShiftByImm),
    UcvtfVecFixedPt(AdvSimdShiftByImm),
    FcvtzuVecFixedPt(AdvSimdShiftByImm),

    FmovRegSinglePrecisionVar(RnRd),
    FabsScalarSinglePrecisionVar(RnRd),
    FnegScalarSinglePrecisionVar(RnRd),
    FsqrtScalarSinglePrecisionVar(RnRd),
    FcvtSingleToDoublePrecisionVar(RnRd),
    FcvtSingleToHalfPrecisionVar(RnRd),
    FrintnScalarSinglePrecisionVar(RnRd),
    FrintpScalarSinglePrecisionVar(RnRd),
    FrintmScalarSinglePrecisionVar(RnRd),
    FrintzScalarSinglePrecisionVar(RnRd),
    FrintaScalarSinglePrecisionVar(RnRd),
    FrintxScalarSinglePrecisionVar(RnRd),
    FrintiScalarSinglePrecisionVar(RnRd),
    FmovRegDoublePrecisionVar(RnRd),
    FabsScalarDoublePrecisionVar(RnRd),
    FnegScalarDoublePrecisionVar(RnRd),
    FsqrtScalarDoublePrecisionVar(RnRd),
    FcvtDoubleToSinglePrecisionVar(RnRd),
    FcvtDoubleToHalfPrecisionVar(RnRd),
    FrintnScalarDoublePrecisionVar(RnRd),
    FrintpScalarDoublePrecisionVar(RnRd),
    FrintmScalarDoublePrecisionVar(RnRd),
    FrintzScalarDoublePrecisionVar(RnRd),
    FrintaScalarDoublePrecisionVar(RnRd),
    FrintxScalarDoublePrecisionVar(RnRd),
    FrintiScalarDoublePrecisionVar(RnRd),

    AddpScalar(AdvSimdScalarPairwise),
    FmaxnmpScalarEncoding(AdvSimdScalarPairwise),
    FaddpScalarEncoding(AdvSimdScalarPairwise),
    FmaxpScalarEncoding(AdvSimdScalarPairwise),
    FminnmpScalarEncoding(AdvSimdScalarPairwise),
    FminpScalarEncoding(AdvSimdScalarPairwise),

    Rev64(QSizeRnRd),
    Rev16Vec(QSizeRnRd),
    Saddlp(QSizeRnRd),
    Suqadd(QSizeRnRd),
    ClsVec(QSizeRnRd),
    Cnt(QSizeRnRd),
    Sadalp(QSizeRnRd),
    Sqabs(QSizeRnRd),
    CmgtZero(QSizeRnRd),
    CmeqZero(QSizeRnRd),
    CmltZero(QSizeRnRd),
    Abs(QSizeRnRd),
    XtnXtn2(QSizeRnRd),
    Sqxtn(QSizeRnRd),
    Fcvtn(QSizeRnRd),
    Fcvtl(QSizeRnRd),
    FrintnVec(QSizeRnRd),
    FrintmVec(QSizeRnRd),
    FcvtnsVec(QSizeRnRd),
    FcvtmsVec(QSizeRnRd),
    FcvtasVec(QSizeRnRd),
    ScvtfVecInt(QSizeRnRd),
    FcmgtZero(QSizeRnRd),
    FcmeqZero(QSizeRnRd),
    FcmltZero(QSizeRnRd),
    FabsVec(QSizeRnRd),
    FrintpVec(QSizeRnRd),
    FrintzVec(QSizeRnRd),
    FcvtpsVec(QSizeRnRd),
    FcvtzsVecInt(QSizeRnRd),
    Urecpe(QSizeRnRd),
    Frecpe(QSizeRnRd),
    Rev32Vec(QSizeRnRd),
    Uaddlp(QSizeRnRd),
    Usqadd(QSizeRnRd),
    ClzVec(QSizeRnRd),
    Uadalp(QSizeRnRd),
    Sqneg(QSizeRnRd),
    CmgeZero(QSizeRnRd),
    CmleZero(QSizeRnRd),
    NegVec(QSizeRnRd),
    Sqxtun(QSizeRnRd),
    Shll(QSizeRnRd),
    Uqxtn(QSizeRnRd),
    Fcvtxn(QSizeRnRd),
    FrintaVec(QSizeRnRd),
    FrintxVec(QSizeRnRd),
    FcvtnuVec(QSizeRnRd),
    FcvtmuVec(QSizeRnRd),
    FcvtauVec(QSizeRnRd),
    UcvtfVecInt(QSizeRnRd),
    Not(QSizeRnRd),
    RbitVec(QSizeRnRd),
    FcmgeZero(QSizeRnRd),
    FcmleZero(QSizeRnRd),
    FnegVec(QSizeRnRd),
    FrintiVec(QSizeRnRd),
    FcvtpuVec(QSizeRnRd),
    FcvtzuVecInt(QSizeRnRd),
    Ursqrte(QSizeRnRd),
    Frsqrte(QSizeRnRd),
    FsqrtVec(QSizeRnRd),

    Saddlv(QSizeRnRd),
    Smaxv(QSizeRnRd),
    Sminv(QSizeRnRd),
    Addv(QSizeRnRd),
    Uaddlv(QSizeRnRd),
    Umaxv(QSizeRnRd),
    Uminv(QSizeRnRd),
    FmaxnvmEncoding(QSizeRnRd),
    FmaxvEncoding(QSizeRnRd),
    FminnmvEncoding(QSizeRnRd),
    FminvEncoding(QSizeRnRd),

    Udf(Imm16),

    Casb(RsRnRt),
    Caslb(RsRnRt),
    Casab(RsRnRt),
    Casalb(RsRnRt),
    Cash(RsRnRt),
    Caslh(RsRnRt),
    Casah(RsRnRt),
    Casalh(RsRnRt),

    CasVar32(RsRnRt),
    CaslVar32(RsRnRt),
    CasaVar32(RsRnRt),
    CasalVar32(RsRnRt),
    CasVar64(RsRnRt),
    CaslVar64(RsRnRt),
    CasaVar64(RsRnRt),
    CasalVar64(RsRnRt),

    LdaddbVar(RsRnRt),
    LdclrbVar(RsRnRt),
    LdeorbVar(RsRnRt),
    LdsetbVar(RsRnRt),
    LdsmaxbVar(RsRnRt),
    LdsminbVar(RsRnRt),
    LdumaxbVar(RsRnRt),
    LduminbVar(RsRnRt),
    SwpbVar(RsRnRt),

    LdaddlbVar(RsRnRt),
    LdclrlbVar(RsRnRt),
    LdeorlbVar(RsRnRt),
    LdsetlbVar(RsRnRt),
    LdsmaxlbVar(RsRnRt),
    LdsminlbVar(RsRnRt),
    LdumaxlbVar(RsRnRt),
    LduminlbVar(RsRnRt),
    SwplbVar(RsRnRt),

    LdaddabVar(RsRnRt),
    LdclrabVar(RsRnRt),
    LdeorabVar(RsRnRt),
    LdsetabVar(RsRnRt),
    LdsmaxabVar(RsRnRt),
    LdsminabVar(RsRnRt),
    LdumaxabVar(RsRnRt),
    LduminabVar(RsRnRt),
    SwpabVar(RsRnRt),

    Ldaprb(RsRnRt),

    LdaddalbVar(RsRnRt),
    LdclralbVar(RsRnRt),
    LdeoralbVar(RsRnRt),
    LdsetalbVar(RsRnRt),
    LdsmaxalbVar(RsRnRt),
    LdsminalbVar(RsRnRt),
    LdumaxalbVar(RsRnRt),
    LduminalbVar(RsRnRt),
    SwpalbVar(RsRnRt),

    LdaddhVar(RsRnRt),
    LdclrhVar(RsRnRt),
    LdeorhVar(RsRnRt),
    LdsethVar(RsRnRt),
    LdsmaxhVar(RsRnRt),
    LdsminhVar(RsRnRt),
    LdumaxhVar(RsRnRt),
    LduminhVar(RsRnRt),
    SwphVar(RsRnRt),

    LdaddlhVar(RsRnRt),
    LdclrlhVar(RsRnRt),
    LdeorlhVar(RsRnRt),
    LdsetlhVar(RsRnRt),
    LdsmaxlhVar(RsRnRt),
    LdsminlhVar(RsRnRt),
    LdumaxlhVar(RsRnRt),
    LduminlhVar(RsRnRt),
    SwplhVar(RsRnRt),

    LdaddahVar(RsRnRt),
    LdclrahVar(RsRnRt),
    LdeorahVar(RsRnRt),
    LdsetahVar(RsRnRt),
    LdsmaxahVar(RsRnRt),
    LdsminahVar(RsRnRt),
    LdumaxahVar(RsRnRt),
    LduminahVar(RsRnRt),
    SwpahVar(RsRnRt),

    Ldaprh(RsRnRt),

    LdaddalhVar(RsRnRt),
    LdclralhVar(RsRnRt),
    LdeoralhVar(RsRnRt),
    LdsetalhVar(RsRnRt),
    LdsmaxalhVar(RsRnRt),
    LdsminalhVar(RsRnRt),
    LdumaxalhVar(RsRnRt),
    LduminalhVar(RsRnRt),
    SwpalhVar(RsRnRt),

    LdaddVar32(RsRnRt),
    LdclrVar32(RsRnRt),
    LdeorVar32(RsRnRt),
    LdsetVar32(RsRnRt),
    LdsmaxVar32(RsRnRt),
    LdsminVar32(RsRnRt),
    LdumaxVar32(RsRnRt),
    LduminVar32(RsRnRt),
    SwpVar32(RsRnRt),

    LdaddlVar32(RsRnRt),
    LdclrlVar32(RsRnRt),
    LdeorlVar32(RsRnRt),
    LdsetlVar32(RsRnRt),
    LdsmaxlVar32(RsRnRt),
    LdsminlVar32(RsRnRt),
    LdumaxlVar32(RsRnRt),
    LduminlVar32(RsRnRt),
    SwplVar32(RsRnRt),

    LdaddaVar32(RsRnRt),
    LdclraVar32(RsRnRt),
    LdeoraVar32(RsRnRt),
    LdsetaVar32(RsRnRt),
    LdsmaxaVar32(RsRnRt),
    LdsminaVar32(RsRnRt),
    LdumaxaVar32(RsRnRt),
    LduminaVar32(RsRnRt),
    SwpaVar32(RsRnRt),

    LdaprVar32(RsRnRt),

    LdaddalVar32(RsRnRt),
    LdclralVar32(RsRnRt),
    LdeoralVar32(RsRnRt),
    LdsetalVar32(RsRnRt),
    LdsmaxalVar32(RsRnRt),
    LdsminalVar32(RsRnRt),
    LdumaxalVar32(RsRnRt),
    LduminalVar32(RsRnRt),
    SwpalVar32(RsRnRt),

    LdaddVar64(RsRnRt),
    LdclrVar64(RsRnRt),
    LdeorVar64(RsRnRt),
    LdsetVar64(RsRnRt),
    LdsmaxVar64(RsRnRt),
    LdsminVar64(RsRnRt),
    LdumaxVar64(RsRnRt),
    LduminVar64(RsRnRt),
    SwpVar64(RsRnRt),

    St64bv0(RsRnRt),
    St64bv(RsRnRt),

    St64b(RsRnRt),
    Ld64b(RsRnRt),

    LdaddlVar64(RsRnRt),
    LdclrlVar64(RsRnRt),
    LdeorlVar64(RsRnRt),
    LdsetlVar64(RsRnRt),
    LdsmaxlVar64(RsRnRt),
    LdsminlVar64(RsRnRt),
    LdumaxlVar64(RsRnRt),
    LduminlVar64(RsRnRt),
    SwplVar64(RsRnRt),

    LdaddaVar64(RsRnRt),
    LdclraVar64(RsRnRt),
    LdeoraVar64(RsRnRt),
    LdsetaVar64(RsRnRt),
    LdsmaxaVar64(RsRnRt),
    LdsminaVar64(RsRnRt),
    LdumaxaVar64(RsRnRt),
    LduminaVar64(RsRnRt),
    SwpaVar64(RsRnRt),

    LdaprVar64(RsRnRt),

    LdaddalVar64(RsRnRt),
    LdclralVar64(RsRnRt),
    LdeoralVar64(RsRnRt),
    LdsetalVar64(RsRnRt),
    LdsmaxalVar64(RsRnRt),
    LdsminalVar64(RsRnRt),
    LdumaxalVar64(RsRnRt),
    LduminalVar64(RsRnRt),
    SwpalVar64(RsRnRt),

    Fcmp(FloatingPointCompare),
    Fcmpe(FloatingPointCompare),

    Uzp1(QSizeRmRnRd),
    Trn1(QSizeRmRnRd),
    Zip1(QSizeRmRnRd),
    Uzp2(QSizeRmRnRd),
    Trn2(QSizeRmRnRd),
    Zip2(QSizeRmRnRd),

    FmulScalarSinglePrecisionVar(RmRnRd),
    FdivScalarSinglePrecisionVar(RmRnRd),
    FaddScalarSinglePrecisionVar(RmRnRd),
    FsubScalarSinglePrecisionVar(RmRnRd),
    FmaxScalarSinglePrecisionVar(RmRnRd),
    FminScalarSinglePrecisionVar(RmRnRd),
    FmaxnmScalarSinglePrecisionVar(RmRnRd),
    FminnmScalarSinglePrecisionVar(RmRnRd),
    FnmulScalarSinglePrecisionVar(RmRnRd),

    FmulScalarDoublePrecisionVar(RmRnRd),
    FdivScalarDoublePrecisionVar(RmRnRd),
    FaddScalarDoublePrecisionVar(RmRnRd),
    FsubScalarDoublePrecisionVar(RmRnRd),
    FmaxScalarDoublePrecisionVar(RmRnRd),
    FminScalarDoublePrecisionVar(RmRnRd),
    FmaxnmScalarDoublePrecisionVar(RmRnRd),
    FminnmScalarDoublePrecisionVar(RmRnRd),
    FnmulScalarDoublePrecisionVar(RmRnRd),

    FmovScalarImmSinglePrecisionVar(FloatingPointImmediate),
    FmovScalarImmDoublePrecisionVar(FloatingPointImmediate),

    ScvtfScalarFixedPt32ToSinglePrecision(ConvBetweenFloatAndFixedPoint),
    UcvtfScalarFixedPt32ToSinglePrecision(ConvBetweenFloatAndFixedPoint),
    FcvtzsScalarFixedPtSinglePrecisionTo32(ConvBetweenFloatAndFixedPoint),
    FcvtzuScalarFixedPtSinglePrecisionTo32(ConvBetweenFloatAndFixedPoint),

    ScvtfScalarFixedPt32ToDoublePrecision(ConvBetweenFloatAndFixedPoint),
    UcvtfScalarFixedPt32ToDoublePrecision(ConvBetweenFloatAndFixedPoint),
    FcvtzsScalarFixedPtDoublePrecisionTo32(ConvBetweenFloatAndFixedPoint),
    FcvtzuScalarFixedPtDoublePrecisionTo32(ConvBetweenFloatAndFixedPoint),

    ScvtfScalarFixedPt64ToSinglePrecision(ConvBetweenFloatAndFixedPoint),
    UcvtfScalarFixedPt64ToSinglePrecision(ConvBetweenFloatAndFixedPoint),
    FcvtzsScalarFixedPtSinglePrecisionTo64(ConvBetweenFloatAndFixedPoint),
    FcvtzuScalarFixedPtSinglePrecisionTo64(ConvBetweenFloatAndFixedPoint),

    ScvtfScalarFixedPt64ToDoublePrecision(ConvBetweenFloatAndFixedPoint),
    UcvtfScalarFixedPt64ToDoublePrecision(ConvBetweenFloatAndFixedPoint),
    FcvtzsScalarFixedPtDoublePrecisionTo64(ConvBetweenFloatAndFixedPoint),
    FcvtzuScalarFixedPtDoublePrecisionTo64(ConvBetweenFloatAndFixedPoint),

    FcselSinglePrecisionVar(RmCondRnRd),
    FcselDoublePrecisionVar(RmCondRnRd),

    SmlalByElem(AdvSimdXIndexedElem),
    SqdmlalByElem(AdvSimdXIndexedElem),
    SmlslByElem(AdvSimdXIndexedElem),
    SqdmlslByElem(AdvSimdXIndexedElem),
    MulByElem(AdvSimdXIndexedElem),
    SmullByElem(AdvSimdXIndexedElem),
    SqdmullByElem(AdvSimdXIndexedElem),
    SqdmulhByElem(AdvSimdXIndexedElem),
    SqrdmulhByElem(AdvSimdXIndexedElem),

    FmlaByElemEncoding(AdvSimdXIndexedElem),
    FmlsByElemEncoding(AdvSimdXIndexedElem),
    FmulByElemEncoding(AdvSimdXIndexedElem),

    MlaByElem(AdvSimdXIndexedElem),
    UmlalByElem(AdvSimdXIndexedElem),
    MlsByElem(AdvSimdXIndexedElem),
    UmlslByElem(AdvSimdXIndexedElem),
    UmullByElem(AdvSimdXIndexedElem),
    FmulxByElemEncoding(AdvSimdXIndexedElem),
}

impl Instruction for AArch64Inst {
    fn size(&self) -> u64 {
        4
    }

    /// decode the instruction from raw bytes
    fn decode(raw_inst: &[u8]) -> Self {
        decode_aarch64_inst(raw_inst)
    }

    fn compile_to_ir(&self, basic_block: &mut BasicBlock) {
        compile_aarch64_to_ir(self, basic_block)
    }
}