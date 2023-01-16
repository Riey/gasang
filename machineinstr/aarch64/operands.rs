#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RmRnRd {
    pub rm: u8,
    pub rn: u8,
    pub rd: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RmRaRnRd {
    pub rm: u8,
    pub ra: u8,
    pub rn: u8,
    pub rd: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShImm12RnRd {
    pub sh: u8,
    pub imm12: u16,
    pub rn: u8,
    pub rd: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Imm12RnRt {
    pub imm12: u16,
    pub rn: u8,
    pub rt: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Imm26 {
    pub imm26: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Imm19Cond {
    pub imm19: u32,
    pub cond: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct RmCondRnRd {
    pub rm: u8,
    pub cond: u8,
    pub rn: u8,
    pub rd: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Imm16Rd {
    pub imm16: u16,
    pub rd: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct B5B40Imm14Rt {
    pub b5: u8,
    pub b40: u8,
    pub imm14: u16,
    pub rt: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ShiftRmImm6RnRd {
    pub shift: u8,
    pub rm: u8,
    pub imm6: u8,
    pub rn: u8,
    pub rd: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct UncondBranchReg {
    pub z: u8,
    pub op: u8,
    pub a: u8,
    pub rn: u8,
    pub rm: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PcRelAddressing {
    pub immlo: u8,
    pub immhi: u32,
    pub rd: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExceptionGen {
    pub opc: u8,
    pub imm16: u16,
    pub op2: u8,
    pub ll: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoadStoreRegRegOffset {
    pub size: u8,
    pub v: u8,
    pub opc: u8,
    pub rm: u8,
    pub option: u8,
    pub s: u8,
    pub rn: u8,
    pub rt: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AddSubtractExtReg {
    pub rm: u8,
    pub option: u8,
    pub imm3: u8,
    pub rn: u8,
    pub rd: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Bitfield {
    pub immr: u8,
    pub imms: u8,
    pub rn: u8,
    pub rd: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LogicalImm {
    pub immr: u8,
    pub imms: u8,
    pub rn: u8,
    pub rd: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct LoadStoreRegPairOffset {
    pub imm7: u8,
    pub rt2: u8,
    pub rn: u8,
    pub rt: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AddSubImmWithTags {
    pub o2: u8,
    pub uimm6: u8,
    pub op3: u8,
    pub uimm4: u8,
    pub rn: u8,
    pub rd: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ExtractImm {
    pub rm: u8,
    pub imms: u8,
    pub rn: u8,
    pub rd: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct CmpAndBranchImm {
    pub imm19: u32,
    pub rt: u8,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DataProc3Src {
    pub rm: u8,
    pub ra: u8,
    pub rn: u8,
    pub rd: u8,
}