#[derive(PartialEq, Copy, Clone)]
pub enum ArgumentSize {
    Word8,
    Word16,
    Word24,
    Word32,
}

#[derive(PartialEq)]
pub enum InstructionArgument {
    Number(ArgumentSize),
    Numbers(&'static [ArgumentSize]),
    Register(&'static str),
    NotStaticRegister(String),
}

#[derive(PartialEq)]
pub enum AddressingMode {
    Implied,
    Immediate,
    Relative,
    SingleArgument,
    Indexed,
    Indirect,
    IndirectLong,
    IndexedIndirect,
    IndirectIndexed,
    IndirectIndexedLong,
    BlockMove,
    StackRelativeIndirectIndexed,
}

pub struct InstructionInfo {
    pub name: &'static str,
    pub addressing: AddressingMode,
    pub opcode: u8,
    pub arguments: &'static [InstructionArgument],
}

pub struct SystemDefinition {
    pub short_name: &'static str,
    pub name: &'static str,
    pub is_big_endian: bool,
    pub registers: &'static [&'static str],
    pub size_formatting: fn(ArgumentSize) -> &'static str,
    pub instructions: &'static [InstructionInfo],
}

pub fn argument_size_to_bit_size(size: ArgumentSize) -> i32 {
    match size {
        ArgumentSize::Word8 => 8,
        ArgumentSize::Word16 => 16,
        ArgumentSize::Word24 => 24,
        ArgumentSize::Word32 => 32,
    }
}
