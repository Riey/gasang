use machineinstr::{aarch64::AArch64InstrParserRule, MachineInstrParserRule};
use proptest::prelude::*;
use utility::BitReader;

fn parse_instr(instr: u32) {
    let buf = &mut BitReader::new(instr.to_le_bytes().into_iter());
    while let Some(_inst) = AArch64InstrParserRule.parse(buf) {}
}

proptest! {
    #[test]
    fn no_reserved_byte(ty in (1u32..1 << 7).prop_filter("Reserved instr", |&ty| ty & 0b1_00_1111 != 0), content in (0u32..1 << 25)) {
        let instr = ty << 25 | content;
        println!("{instr:032b}");
        parse_instr(instr);
    }
}

#[test]
fn udf() {
    parse_instr(0);
}
