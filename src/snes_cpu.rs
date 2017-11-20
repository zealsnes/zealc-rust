use zeal::system_definition::*;

pub static SNES_CPU: SystemDefinition = SystemDefinition {
    short_name: "snes-cpu",
    name: "Super Nintendo/Super Famicom Ricoh 5A22 (65816 derivate)",
    is_big_endian: false,
    instructions: &[
        InstructionInfo { name: "clc", addressing: AddressingMode::Implied, opcode: 0x18 },
        InstructionInfo { name: "cld", addressing: AddressingMode::Implied, opcode: 0xD8 },
        InstructionInfo { name: "cli", addressing: AddressingMode::Implied, opcode: 0x58 },
    ],
};
