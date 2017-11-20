pub enum AddressingMode {
    Implied,
    Immediate,
    Relative,
    RelativeLong,
    Direct,
    DirectIndexedX,
    DirectIndexedY,
    DirectIndirect,
    DirectIndexedIndirect,
    DirectIndirectIndexed,
    DirectIndirectLong,
    DirectIndirectIndexedLong,
    Absolute,
    AbsoluteIndexedX,
    AbsoluteIndexedY,
    AbsoluteLong,
    AbsoluteIndexedLong,
    StackRelative,
    StackRelativeIndirectIndexed,
    AbsoluteIndirect,
    AbsoluteIndirectLong,
    AbsoluteIndexedIndirect,
    BlockMove,
}

pub struct InstructionInfo {
    pub name: &'static str,
    pub addressing: AddressingMode,
    pub opcode: u8,
}

pub struct SystemDefinition {
    pub short_name: &'static str,
    pub name: &'static str,
    pub is_big_endian: bool,
    pub instructions: &'static [InstructionInfo]
}
