use zeal::system_definition::*;

fn snes_argument_size_to_string(size: ArgumentSize) -> &'static str {
    match size {
        ArgumentSize::Word8 => "direct page",
        ArgumentSize::Word16 => "absolute",
        ArgumentSize::Word24 => "absolute long",
        ArgumentSize::Word32 => "invalid"
    }
}

pub static SNES_CPU: SystemDefinition = SystemDefinition {
    short_name: "snes-cpu",
    name: "Super Nintendo/Super Famicom Ricoh 5A22 (65816 derivate)",
    is_big_endian: false,
    registers: &[
        "x",
        "y",
    ],
    size_formatting: snes_argument_size_to_string,
    instructions: &[
        InstructionInfo { name: "clc", addressing: AddressingMode::Implied, opcode: 0x18, arguments: &[] },
        InstructionInfo { name: "cld", addressing: AddressingMode::Implied, opcode: 0xD8, arguments: &[] },
        InstructionInfo { name: "cli", addressing: AddressingMode::Implied, opcode: 0x58, arguments: &[] },
        // lda #immediate
        InstructionInfo { name: "lda", addressing: AddressingMode::Immediate, opcode: 0xA9, arguments: &[InstructionArgument::Numbers(&[ArgumentSize::Word8, ArgumentSize::Word16])] },
        // lda dp
        InstructionInfo { name: "lda", addressing: AddressingMode::SingleArgument, opcode: 0xA5, arguments: &[InstructionArgument::Number(ArgumentSize::Word8)] },
        // lda absolute
        InstructionInfo { name: "lda", addressing: AddressingMode::SingleArgument, opcode: 0xAD, arguments: &[InstructionArgument::Number(ArgumentSize::Word16)] },
        // lda long
        InstructionInfo { name: "lda", addressing: AddressingMode::SingleArgument, opcode: 0xAF, arguments: &[InstructionArgument::Number(ArgumentSize::Word24)] },
        // lda dp,x
        InstructionInfo { name: "lda", addressing: AddressingMode::Indexed, opcode: 0xB5, arguments: &[InstructionArgument::Number(ArgumentSize::Word8), InstructionArgument::Register("x")] },
        // lda absolute,x
        InstructionInfo { name: "lda", addressing: AddressingMode::Indexed, opcode: 0xBD, arguments: &[InstructionArgument::Number(ArgumentSize::Word16), InstructionArgument::Register("x")] },
        // lda long,x
        InstructionInfo { name: "lda", addressing: AddressingMode::Indexed, opcode: 0xBF, arguments: &[InstructionArgument::Number(ArgumentSize::Word24), InstructionArgument::Register("x")] },
        // lda absolute,y
        InstructionInfo { name: "lda", addressing: AddressingMode::Indexed, opcode: 0xB9, arguments: &[InstructionArgument::Number(ArgumentSize::Word16), InstructionArgument::Register("y")] },
    ],
};
