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
    pub label_size: ArgumentSize,
    pub registers: &'static [&'static str],
    pub size_to_addressing_mode: fn(ArgumentSize) -> &'static str,
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

pub fn argument_size_to_byte_size(size: ArgumentSize) -> u32 {
    match size {
        ArgumentSize::Word8 => 1,
        ArgumentSize::Word16 => 2,
        ArgumentSize::Word24 => 3,
        ArgumentSize::Word32 => 4,
    }
}

pub fn number_to_argument_size(number: u32) -> ArgumentSize {
    if number > 16777215 {
        ArgumentSize::Word32
    } else if number > u16::max_value() as u32 {
        ArgumentSize::Word24
    } else if number > u8::max_value() as u32 {
        ArgumentSize::Word16
    } else {
        ArgumentSize::Word8
    }
}