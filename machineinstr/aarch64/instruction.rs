use crate::aarch64::*;

use std::fmt::{Debug, Display, Formatter, Result as FmtResult};

// AArch64 instruction
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum AArch64Instr {
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

    AddShiftedReg32(RmRnRd),
    AddsShiftedReg32(RmRnRd),
    SubShiftedReg32(RmRnRd),
    SubsShiftedReg32(RmRnRd),
    AddShiftedReg64(RmRnRd),
    AddsShiftedReg64(RmRnRd),
    SubShiftedReg64(RmRnRd),
    SubsShiftedReg64(RmRnRd),

    AddExtReg32(AddSubtractExtReg),
    AddsExtReg32(AddSubtractExtReg),
    SubExtReg32(AddSubtractExtReg),
    SubsExtReg32(AddSubtractExtReg),
    AddExtReg64(AddSubtractExtReg),
    AddsExtReg64(AddSubtractExtReg),
    SubExtReg64(AddSubtractExtReg),
    SubsExtReg64(AddSubtractExtReg),

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

    StrbImm(Imm12RnRt),
    LdrbImm(Imm12RnRt),
    LdrsbImm32(Imm12RnRt),
    LdrsbImm64(Imm12RnRt),
    StrImmSimdFP8(Imm12RnRt),
    LdrImmSimdFP8(Imm12RnRt),
    StrImmSimdFP128(Imm12RnRt),
    LdrImmSimdFP128(Imm12RnRt),
    StrhImm(Imm12RnRt),
    LdrhImm(Imm12RnRt),
    LdrshImm32(Imm12RnRt),
    LdrshImm64(Imm12RnRt),
    StrImmSimdFP16(Imm12RnRt),
    LdrImmSimdFP16(Imm12RnRt),
    StrImm32(Imm12RnRt),
    LdrImm32(Imm12RnRt),
    LdrswImm(Imm12RnRt),
    StrImmSimdFP32(Imm12RnRt),
    LdrImmSimdFP32(Imm12RnRt),
    StrImm64(Imm12RnRt),
    LdrImm64(Imm12RnRt),
    PrfmImm(Imm12RnRt),
    StrImmSimdFP64(Imm12RnRt),
    LdrImmSimdFP64(Imm12RnRt),

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

    Stp32(LoadStoreRegPair),
    Ldp32(LoadStoreRegPair),
    StpSimdFP32(LoadStoreRegPair),
    LdpSimdFP32(LoadStoreRegPair),
    Stgp(LoadStoreRegPair),
    Ldpsw(LoadStoreRegPair),
    StpSimdFP64(LoadStoreRegPair),
    LdpSimdFP64(LoadStoreRegPair),
    Stp64(LoadStoreRegPair),
    Ldp64(LoadStoreRegPair),
    StpSimdFP128(LoadStoreRegPair),
    LdpSimdFP128(LoadStoreRegPair),

    Sturb(Imm12RnRt),
    Ldurb(Imm12RnRt),
    Ldursb64(Imm12RnRt),
    Ldursb32(Imm12RnRt),
    SturSimdFP8(Imm12RnRt),
    LdurSimdFP8(Imm12RnRt),
    SturSimdFP128(Imm12RnRt),
    LdurSimdFP128(Imm12RnRt),
    Sturh(Imm12RnRt),
    Ldurh(Imm12RnRt),
    Ldursh64(Imm12RnRt),
    Ldursh32(Imm12RnRt),
    SturSimdFP16(Imm12RnRt),
    LdurSimdFP16(Imm12RnRt),
    Stur32(Imm12RnRt),
    Ldur32(Imm12RnRt),
    Ldursw(Imm12RnRt),
    SturSimdFP32(Imm12RnRt),
    LdurSimdFP32(Imm12RnRt),
    Stur64(Imm12RnRt),
    Ldur64(Imm12RnRt),
    Prefum(Imm12RnRt),
    SturSimdFP64(Imm12RnRt),
    LdurSimdFP64(Imm12RnRt),

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

    Stxrb(LoadStoreRegExclusive),
    Ldxrb(LoadStoreRegExclusive),
    Stxrh(LoadStoreRegExclusive),
    Ldxrh(LoadStoreRegExclusive),
    StxrVar32(LoadStoreRegExclusive),
    LdxrVar32(LoadStoreRegExclusive),
    StxrVar64(LoadStoreRegExclusive),
    LdxrVar64(LoadStoreRegExclusive),
    Stlxrb(LoadStoreRegExclusive),
    Ldaxrb(LoadStoreRegExclusive),
    Stlxrh(LoadStoreRegExclusive),
    Ldaxrh(LoadStoreRegExclusive),
    StlxrVar32(LoadStoreRegExclusive),
    LdaxrVar32(LoadStoreRegExclusive),
    StlxrVar64(LoadStoreRegExclusive),
    LdaxrVar64(LoadStoreRegExclusive),

    Stlrb(LoadStoreOrdered),
    Ldarb(LoadStoreOrdered),
    Stlrh(LoadStoreOrdered),
    Ldarh(LoadStoreOrdered),
    StlrVar32(LoadStoreOrdered),
    LdarVar32(LoadStoreOrdered),
    StlrVar64(LoadStoreOrdered),
    LdarVar64(LoadStoreOrdered),

    BImm(Imm26),
    BlImm(Imm26),

    BCond(Imm19Cond),
    BcCond(Imm19Cond),

    Tbz(B5B40Imm14Rt),
    Tbnz(B5B40Imm14Rt),

    Cbz32(CmpAndBranchImm),
    Cbnz32(CmpAndBranchImm),
    Cbz64(CmpAndBranchImm),
    Cbnz64(CmpAndBranchImm),

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

    Movn32(Imm16Rd),
    Movz32(Imm16Rd),
    Movk32(Imm16Rd),
    Movn64(Imm16Rd),
    Movz64(Imm16Rd),
    Movk64(Imm16Rd),

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

    Nop,
    Yield,
    Wfe,
    Wfi,
    Sev,
    Sevl,

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

    Shadd(AdvancedSimd3Same),
    Sqadd(AdvancedSimd3Same),
    Srhadd(AdvancedSimd3Same),
    Shsub(AdvancedSimd3Same),
    Sqsub(AdvancedSimd3Same),
    CmgtReg(AdvancedSimd3Same),
    CmgeReg(AdvancedSimd3Same),
    Sshl(AdvancedSimd3Same),
    SqshlReg(AdvancedSimd3Same),
    Srshl(AdvancedSimd3Same),
    Sqrshl(AdvancedSimd3Same),
    Smax(AdvancedSimd3Same),
    Smin(AdvancedSimd3Same),
    Sabd(AdvancedSimd3Same),
    Saba(AdvancedSimd3Same),
    AddVec(AdvancedSimd3Same),
    Cmtst(AdvancedSimd3Same),
    MlaVec(AdvancedSimd3Same),
    MulVec(AdvancedSimd3Same),
    Smaxp(AdvancedSimd3Same),
    Sminp(AdvancedSimd3Same),
    SqdmulhVec(AdvancedSimd3Same),
    AddpVec(AdvancedSimd3Same),
    FmaxnmVec(AdvancedSimd3Same),
    FmlaVec(AdvancedSimd3Same),
    FaddVec(AdvancedSimd3Same),
    Fmulx(AdvancedSimd3Same),
    FcmeqReg(AdvancedSimd3Same),
    FmaxVec(AdvancedSimd3Same),
    Frecps(AdvancedSimd3Same),
    AndVec(AdvancedSimd3Same),
    BicVecReg(AdvancedSimd3Same),
    FminnmVec(AdvancedSimd3Same),
    FmlsVec(AdvancedSimd3Same),
    FsubVec(AdvancedSimd3Same),
    FminVec(AdvancedSimd3Same),
    Frsqrts(AdvancedSimd3Same),
    OrrVecReg(AdvancedSimd3Same),
    OrnVec(AdvancedSimd3Same),
    Uhadd(AdvancedSimd3Same),
    Uqadd(AdvancedSimd3Same),
    Urhadd(AdvancedSimd3Same),
    Uhsub(AdvancedSimd3Same),
    Uqsub(AdvancedSimd3Same),
    CmhiReg(AdvancedSimd3Same),
    CmhsReg(AdvancedSimd3Same),
    Ushl(AdvancedSimd3Same),
    UqshlReg(AdvancedSimd3Same),
    Urshl(AdvancedSimd3Same),
    Uqrshl(AdvancedSimd3Same),
    Umax(AdvancedSimd3Same),
    Umin(AdvancedSimd3Same),
    Uabd(AdvancedSimd3Same),
    Uaba(AdvancedSimd3Same),
    SubVec(AdvancedSimd3Same),
    CmeqReg(AdvancedSimd3Same),
    MlsVec(AdvancedSimd3Same),
    Pmul(AdvancedSimd3Same),
    Umaxp(AdvancedSimd3Same),
    Uminp(AdvancedSimd3Same),
    SqrdmulhVec(AdvancedSimd3Same),
    FmaxnmpVec(AdvancedSimd3Same),
    FaddpVec(AdvancedSimd3Same),
    FmulVec(AdvancedSimd3Same),
    FcmgeReg(AdvancedSimd3Same),
    Facge(AdvancedSimd3Same),
    FmaxpVec(AdvancedSimd3Same),
    FdivVec(AdvancedSimd3Same),
    EorVec(AdvancedSimd3Same),
    Bsl(AdvancedSimd3Same),
    FminnmpVec(AdvancedSimd3Same),
    Fabd(AdvancedSimd3Same),
    FcmgtReg(AdvancedSimd3Same),
    Facgt(AdvancedSimd3Same),
    FminpVec(AdvancedSimd3Same),
    Bit(AdvancedSimd3Same),
    Bif(AdvancedSimd3Same),

    Sshr(AdvSimdShiftByImm),
    Ssra(AdvSimdShiftByImm),
    Srshr(AdvSimdShiftByImm),
    Srsra(AdvSimdShiftByImm),
    Shl(AdvSimdShiftByImm),
    SqshlImm(AdvSimdShiftByImm),
    ShrnShrn2(AdvSimdShiftByImm),
    RshrnRshrn2(AdvSimdShiftByImm),
    SqshrnSqshrn2(AdvSimdShiftByImm),
    SqrshrnSqrshrn2(AdvSimdShiftByImm),
    SshllSshll2(AdvSimdShiftByImm),
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
    SqshrunSqshrun2(AdvSimdShiftByImm),
    SqrshrunSqrshrun2(AdvSimdShiftByImm),
    UqshrnUqshrn2(AdvSimdShiftByImm),
    UqrshrnUqrshrn2(AdvSimdShiftByImm),
    UshllUshll2(AdvSimdShiftByImm),
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

    Rev64(AdvSimd2RegMiscellaneous),
    Rev16Vec(AdvSimd2RegMiscellaneous),
    Saddlp(AdvSimd2RegMiscellaneous),
    Suqadd(AdvSimd2RegMiscellaneous),
    ClsVec(AdvSimd2RegMiscellaneous),
    Cnt(AdvSimd2RegMiscellaneous),
    Sadalp(AdvSimd2RegMiscellaneous),
    Sqabs(AdvSimd2RegMiscellaneous),
    CmgtZero(AdvSimd2RegMiscellaneous),
    CmeqZero(AdvSimd2RegMiscellaneous),
    CmltZero(AdvSimd2RegMiscellaneous),
    Abs(AdvSimd2RegMiscellaneous),
    XtnXtn2(AdvSimd2RegMiscellaneous),
    SqxtnSqxtn2(AdvSimd2RegMiscellaneous),
    FcvtnFcvtn2(AdvSimd2RegMiscellaneous),
    FcvtlFcvtl2(AdvSimd2RegMiscellaneous),
    FrintnVec(AdvSimd2RegMiscellaneous),
    FrintmVec(AdvSimd2RegMiscellaneous),
    FcvtnsVec(AdvSimd2RegMiscellaneous),
    FcvtmsVec(AdvSimd2RegMiscellaneous),
    FcvtasVec(AdvSimd2RegMiscellaneous),
    ScvtfVecInt(AdvSimd2RegMiscellaneous),
    FcmgtZero(AdvSimd2RegMiscellaneous),
    FcmeqZero(AdvSimd2RegMiscellaneous),
    FcmltZero(AdvSimd2RegMiscellaneous),
    FabsVec(AdvSimd2RegMiscellaneous),
    FrintpVec(AdvSimd2RegMiscellaneous),
    FrintzVec(AdvSimd2RegMiscellaneous),
    FcvtpsVec(AdvSimd2RegMiscellaneous),
    FcvtzsVecInt(AdvSimd2RegMiscellaneous),
    Urecpe(AdvSimd2RegMiscellaneous),
    Frecpe(AdvSimd2RegMiscellaneous),
    Rev32Vec(AdvSimd2RegMiscellaneous),
    Uaddlp(AdvSimd2RegMiscellaneous),
    Usqadd(AdvSimd2RegMiscellaneous),
    ClzVec(AdvSimd2RegMiscellaneous),
    Uadalp(AdvSimd2RegMiscellaneous),
    Sqneg(AdvSimd2RegMiscellaneous),
    CmgeZero(AdvSimd2RegMiscellaneous),
    CmleZero(AdvSimd2RegMiscellaneous),
    NegVec(AdvSimd2RegMiscellaneous),
    SqxtunSqxtun2(AdvSimd2RegMiscellaneous),
    ShllShll2(AdvSimd2RegMiscellaneous),
    UqxtnUqxtn2(AdvSimd2RegMiscellaneous),
    FcvtxnFcvtxn2(AdvSimd2RegMiscellaneous),
    FrintaVec(AdvSimd2RegMiscellaneous),
    FrintxVec(AdvSimd2RegMiscellaneous),
    FcvtnuVec(AdvSimd2RegMiscellaneous),
    FcvtmuVec(AdvSimd2RegMiscellaneous),
    FcvtauVec(AdvSimd2RegMiscellaneous),
    UcvtfVecInt(AdvSimd2RegMiscellaneous),
    Not(AdvSimd2RegMiscellaneous),
    RbitVec(AdvSimd2RegMiscellaneous),
    FcmgeZero(AdvSimd2RegMiscellaneous),
    FcmleZero(AdvSimd2RegMiscellaneous),
    FnegVec(AdvSimd2RegMiscellaneous),
    FrintiVec(AdvSimd2RegMiscellaneous),
    FcvtpuVec(AdvSimd2RegMiscellaneous),
    FcvtzuVecInt(AdvSimd2RegMiscellaneous),
    Ursqrte(AdvSimd2RegMiscellaneous),
    Frsqrte(AdvSimd2RegMiscellaneous),
    FsqrtVec(AdvSimd2RegMiscellaneous),

}
