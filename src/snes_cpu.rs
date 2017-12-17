use zeal::system_definition::*;

fn snes_argument_size_to_addressing_mode(size: ArgumentSize) -> &'static str {
    match size {
        ArgumentSize::Word8 => "direct page",
        ArgumentSize::Word16 => "absolute",
        ArgumentSize::Word24 => "absolute long",
        ArgumentSize::Word32 => "invalid",
    }
}

pub static SNES_CPU: SystemDefinition = SystemDefinition {
    short_name: "snes-cpu",
    name: "Super Nintendo/Super Famicom Ricoh 5A22 (65816 derivate)",
    is_big_endian: false,
    label_size: ArgumentSize::Word16,
    registers: &["x", "y", "s"],
    size_to_addressing_mode: snes_argument_size_to_addressing_mode,
    instructions: &[
        // adc (dp,x)
        InstructionInfo {
            name: "adc",
            addressing: AddressingMode::IndexedIndirect,
            opcode: 0x61,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("x"),
            ],
        },
        // adc byte,s
        InstructionInfo {
            name: "adc",
            addressing: AddressingMode::Indexed,
            opcode: 0x63,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("s"),
            ],
        },
        // adc dp
        InstructionInfo {
            name: "adc",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x65,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // adc [dp]
        InstructionInfo {
            name: "adc",
            addressing: AddressingMode::IndirectLong,
            opcode: 0x67,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // adc #number
        InstructionInfo {
            name: "adc",
            addressing: AddressingMode::Immediate,
            opcode: 0x69,
            arguments: &[
                InstructionArgument::Numbers(&[ArgumentSize::Word8, ArgumentSize::Word16]),
            ],
        },
        // adc absolute
        InstructionInfo {
            name: "adc",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x6D,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word16)],
        },
        // adc long
        InstructionInfo {
            name: "adc",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x6F,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word24)],
        },
        // adc (dp),y
        InstructionInfo {
            name: "adc",
            addressing: AddressingMode::IndirectIndexed,
            opcode: 0x71,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("y"),
            ],
        },
        // adc (dp)
        InstructionInfo {
            name: "adc",
            addressing: AddressingMode::Indirect,
            opcode: 0x72,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // adc (sr,s),y
        InstructionInfo {
            name: "adc",
            addressing: AddressingMode::StackRelativeIndirectIndexed,
            opcode: 0x73,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("s"),
                InstructionArgument::Register("y"),
            ],
        },
        // adc dp,x
        InstructionInfo {
            name: "adc",
            addressing: AddressingMode::Indexed,
            opcode: 0x75,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("x"),
            ],
        },
        // adc [dp],y
        InstructionInfo {
            name: "adc",
            addressing: AddressingMode::IndirectIndexedLong,
            opcode: 0x77,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("y"),
            ],
        },
        // adc absolute,y
        InstructionInfo {
            name: "adc",
            addressing: AddressingMode::Indexed,
            opcode: 0x79,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word16),
                InstructionArgument::Register("y"),
            ],
        },
        // adc absolute,x
        InstructionInfo {
            name: "adc",
            addressing: AddressingMode::Indexed,
            opcode: 0x7D,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word16),
                InstructionArgument::Register("x"),
            ],
        },
        // adc long,x
        InstructionInfo {
            name: "adc",
            addressing: AddressingMode::Indexed,
            opcode: 0x7F,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word24),
                InstructionArgument::Register("x"),
            ],
        },
        // and (dp,x)
        InstructionInfo {
            name: "and",
            addressing: AddressingMode::IndexedIndirect,
            opcode: 0x21,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("x"),
            ],
        },
        // and sr,s
        InstructionInfo {
            name: "and",
            addressing: AddressingMode::Indexed,
            opcode: 0x23,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("s"),
            ],
        },
        // and dp
        InstructionInfo {
            name: "and",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x25,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // and [dp]
        InstructionInfo {
            name: "and",
            addressing: AddressingMode::IndirectLong,
            opcode: 0x27,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // and #immediate
        InstructionInfo {
            name: "and",
            addressing: AddressingMode::Immediate,
            opcode: 0x29,
            arguments: &[
                InstructionArgument::Numbers(&[ArgumentSize::Word8, ArgumentSize::Word16]),
            ],
        },
        // and absolute
        InstructionInfo {
            name: "and",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x2D,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word16)],
        },
        // and long
        InstructionInfo {
            name: "and",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x2F,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word24)],
        },
        // and (dp),y
        InstructionInfo {
            name: "and",
            addressing: AddressingMode::IndirectIndexed,
            opcode: 0x31,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("y"),
            ],
        },
        // and (dp)
        InstructionInfo {
            name: "and",
            addressing: AddressingMode::Indirect,
            opcode: 0x32,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // and (sr,s),y
        InstructionInfo {
            name: "and",
            addressing: AddressingMode::StackRelativeIndirectIndexed,
            opcode: 0x33,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("s"),
                InstructionArgument::Register("y"),
            ],
        },
        // and dp,x
        InstructionInfo {
            name: "and",
            addressing: AddressingMode::Indexed,
            opcode: 0x35,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("x"),
            ],
        },
        // and [dp],y
        InstructionInfo {
            name: "and",
            addressing: AddressingMode::IndirectIndexedLong,
            opcode: 0x37,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("y"),
            ],
        },
        // and absolute,y
        InstructionInfo {
            name: "and",
            addressing: AddressingMode::Indexed,
            opcode: 0x39,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word16),
                InstructionArgument::Register("y"),
            ],
        },
        // and absolute,x
        InstructionInfo {
            name: "and",
            addressing: AddressingMode::Indexed,
            opcode: 0x3D,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word16),
                InstructionArgument::Register("x"),
            ],
        },
        // and long,x
        InstructionInfo {
            name: "and",
            addressing: AddressingMode::Indexed,
            opcode: 0x3F,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word24),
                InstructionArgument::Register("x"),
            ],
        },
        // asl dp
        InstructionInfo {
            name: "asl",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x06,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // asl
        InstructionInfo {
            name: "asl",
            addressing: AddressingMode::Implied,
            opcode: 0x0A,
            arguments: &[],
        },
        // asl absolute
        InstructionInfo {
            name: "asl",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x0E,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word16)],
        },
        // asl dp,x
        InstructionInfo {
            name: "asl",
            addressing: AddressingMode::Indexed,
            opcode: 0x16,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("x"),
            ],
        },
        // asl absolute,x
        InstructionInfo {
            name: "asl",
            addressing: AddressingMode::Indexed,
            opcode: 0x1E,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word16),
                InstructionArgument::Register("x"),
            ],
        },
        // bcc label
        InstructionInfo {
            name: "bcc",
            addressing: AddressingMode::Relative,
            opcode: 0x90,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // bcs label
        InstructionInfo {
            name: "bcs",
            addressing: AddressingMode::Relative,
            opcode: 0xB0,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // beq label
        InstructionInfo {
            name: "beq",
            addressing: AddressingMode::Relative,
            opcode: 0xF0,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // bit dp
        InstructionInfo {
            name: "bit",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x24,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // bit absolute
        InstructionInfo {
            name: "bit",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x2C,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word16)],
        },
        // bit dp,x
        InstructionInfo {
            name: "bit",
            addressing: AddressingMode::Indexed,
            opcode: 0x34,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("x"),
            ],
        },
        // bit absolute,x
        InstructionInfo {
            name: "bit",
            addressing: AddressingMode::Indexed,
            opcode: 0x3C,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word16),
                InstructionArgument::Register("x"),
            ],
        },
        // bit #immediate
        InstructionInfo {
            name: "bit",
            addressing: AddressingMode::Immediate,
            opcode: 0x89,
            arguments: &[
                InstructionArgument::Numbers(&[ArgumentSize::Word8, ArgumentSize::Word16]),
            ],
        },
        // bmi label
        InstructionInfo {
            name: "bmi",
            addressing: AddressingMode::Relative,
            opcode: 0x30,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // bne label
        InstructionInfo {
            name: "bne",
            addressing: AddressingMode::Relative,
            opcode: 0xD0,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // bpl label
        InstructionInfo {
            name: "bpl",
            addressing: AddressingMode::Relative,
            opcode: 0x10,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // bra label
        InstructionInfo {
            name: "bra",
            addressing: AddressingMode::Relative,
            opcode: 0x80,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // brk
        InstructionInfo {
            name: "brk",
            addressing: AddressingMode::Implied,
            opcode: 0x00,
            arguments: &[],
        },
        // brl label
        InstructionInfo {
            name: "brl",
            addressing: AddressingMode::Relative,
            opcode: 0x82,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word16)],
        },
        // bvc label
        InstructionInfo {
            name: "bvc",
            addressing: AddressingMode::Relative,
            opcode: 0x50,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // bvs label
        InstructionInfo {
            name: "bvs",
            addressing: AddressingMode::Relative,
            opcode: 0x70,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // clc
        InstructionInfo {
            name: "clc",
            addressing: AddressingMode::Implied,
            opcode: 0x18,
            arguments: &[],
        },
        // cld
        InstructionInfo {
            name: "cld",
            addressing: AddressingMode::Implied,
            opcode: 0xD8,
            arguments: &[],
        },
        // cli
        InstructionInfo {
            name: "cli",
            addressing: AddressingMode::Implied,
            opcode: 0x58,
            arguments: &[],
        },
        // clv
        InstructionInfo {
            name: "clv",
            addressing: AddressingMode::Implied,
            opcode: 0xB8,
            arguments: &[],
        },
        // cmp (dp,x)
        InstructionInfo {
            name: "cmp",
            addressing: AddressingMode::IndexedIndirect,
            opcode: 0xC1,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("x"),
            ],
        },
        // cmp byte,s
        InstructionInfo {
            name: "cmp",
            addressing: AddressingMode::Indexed,
            opcode: 0xC3,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("s"),
            ],
        },
        // cmp dp
        InstructionInfo {
            name: "cmp",
            addressing: AddressingMode::SingleArgument,
            opcode: 0xC5,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // cmp [dp]
        InstructionInfo {
            name: "cmp",
            addressing: AddressingMode::IndirectLong,
            opcode: 0xC7,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // cmp #number
        InstructionInfo {
            name: "cmp",
            addressing: AddressingMode::Immediate,
            opcode: 0xC9,
            arguments: &[
                InstructionArgument::Numbers(&[ArgumentSize::Word8, ArgumentSize::Word16]),
            ],
        },
        // cmp absolute
        InstructionInfo {
            name: "cmp",
            addressing: AddressingMode::SingleArgument,
            opcode: 0xCD,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word16)],
        },
        // cmp long
        InstructionInfo {
            name: "cmp",
            addressing: AddressingMode::SingleArgument,
            opcode: 0xCF,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word24)],
        },
        // cmp (dp),y
        InstructionInfo {
            name: "cmp",
            addressing: AddressingMode::IndirectIndexed,
            opcode: 0xD1,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("y"),
            ],
        },
        // cmp (dp)
        InstructionInfo {
            name: "cmp",
            addressing: AddressingMode::Indirect,
            opcode: 0xD2,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // cmp (sr,s),y
        InstructionInfo {
            name: "cmp",
            addressing: AddressingMode::StackRelativeIndirectIndexed,
            opcode: 0xD3,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("s"),
                InstructionArgument::Register("y"),
            ],
        },
        // cmp dp,x
        InstructionInfo {
            name: "cmp",
            addressing: AddressingMode::Indexed,
            opcode: 0xD5,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("x"),
            ],
        },
        // cmp [dp],y
        InstructionInfo {
            name: "cmp",
            addressing: AddressingMode::IndirectIndexedLong,
            opcode: 0xD7,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("y"),
            ],
        },
        // cmp absolute,y
        InstructionInfo {
            name: "cmp",
            addressing: AddressingMode::Indexed,
            opcode: 0xD9,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word16),
                InstructionArgument::Register("y"),
            ],
        },
        // cmp absolute,x
        InstructionInfo {
            name: "cmp",
            addressing: AddressingMode::Indexed,
            opcode: 0xDD,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word16),
                InstructionArgument::Register("x"),
            ],
        },
        // cmp long,x
        InstructionInfo {
            name: "cmp",
            addressing: AddressingMode::Indexed,
            opcode: 0xDF,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word24),
                InstructionArgument::Register("x"),
            ],
        },
        // cop const
        InstructionInfo {
            name: "cop",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x02,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // cpx #immediate
        InstructionInfo {
            name: "cpx",
            addressing: AddressingMode::Immediate,
            opcode: 0xE0,
            arguments: &[
                InstructionArgument::Numbers(&[ArgumentSize::Word8, ArgumentSize::Word16]),
            ],
        },
        // cpx dp
        InstructionInfo {
            name: "cpx",
            addressing: AddressingMode::SingleArgument,
            opcode: 0xE4,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // cpx absolute
        InstructionInfo {
            name: "cpx",
            addressing: AddressingMode::SingleArgument,
            opcode: 0xEC,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word16)],
        },
        // cpy #immediate
        InstructionInfo {
            name: "cpy",
            addressing: AddressingMode::Immediate,
            opcode: 0xC0,
            arguments: &[
                InstructionArgument::Numbers(&[ArgumentSize::Word8, ArgumentSize::Word16]),
            ],
        },
        // cpy dp
        InstructionInfo {
            name: "cpy",
            addressing: AddressingMode::SingleArgument,
            opcode: 0xC4,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // cpx absolute
        InstructionInfo {
            name: "cpy",
            addressing: AddressingMode::SingleArgument,
            opcode: 0xCC,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word16)],
        },
        // dec
        InstructionInfo {
            name: "dec",
            addressing: AddressingMode::Implied,
            opcode: 0x3A,
            arguments: &[],
        },
        // dec dp
        InstructionInfo {
            name: "dec",
            addressing: AddressingMode::SingleArgument,
            opcode: 0xC6,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // dec absolute
        InstructionInfo {
            name: "dec",
            addressing: AddressingMode::SingleArgument,
            opcode: 0xCE,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word16)],
        },
        // dec dp,x
        InstructionInfo {
            name: "dec",
            addressing: AddressingMode::Indexed,
            opcode: 0xD6,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("x"),
            ],
        },
        // dec absolute,x
        InstructionInfo {
            name: "dec",
            addressing: AddressingMode::Indexed,
            opcode: 0xDE,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word16),
                InstructionArgument::Register("x"),
            ],
        },
        // dex
        InstructionInfo {
            name: "dex",
            addressing: AddressingMode::Implied,
            opcode: 0xCA,
            arguments: &[],
        },
        // dey
        InstructionInfo {
            name: "dey",
            addressing: AddressingMode::Implied,
            opcode: 0x88,
            arguments: &[],
        },
        // eor (dp,x)
        InstructionInfo {
            name: "eor",
            addressing: AddressingMode::IndexedIndirect,
            opcode: 0x41,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("x"),
            ],
        },
        // eor sr,s
        InstructionInfo {
            name: "eor",
            addressing: AddressingMode::Indexed,
            opcode: 0x43,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("s"),
            ],
        },
        // eor dp
        InstructionInfo {
            name: "eor",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x45,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // eor [dp]
        InstructionInfo {
            name: "eor",
            addressing: AddressingMode::IndirectLong,
            opcode: 0x47,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // eor #immediate
        InstructionInfo {
            name: "eor",
            addressing: AddressingMode::Immediate,
            opcode: 0x49,
            arguments: &[
                InstructionArgument::Numbers(&[ArgumentSize::Word8, ArgumentSize::Word16]),
            ],
        },
        // eor absolute
        InstructionInfo {
            name: "eor",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x4D,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word16)],
        },
        // eor long
        InstructionInfo {
            name: "eor",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x4F,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word24)],
        },
        // eor (dp),y
        InstructionInfo {
            name: "eor",
            addressing: AddressingMode::IndirectIndexed,
            opcode: 0x51,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("y"),
            ],
        },
        // eor (dp)
        InstructionInfo {
            name: "eor",
            addressing: AddressingMode::Indirect,
            opcode: 0x52,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // eor (sr,s),y
        InstructionInfo {
            name: "eor",
            addressing: AddressingMode::StackRelativeIndirectIndexed,
            opcode: 0x53,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("s"),
                InstructionArgument::Register("y"),
            ],
        },
        // eor dp,x
        InstructionInfo {
            name: "eor",
            addressing: AddressingMode::Indexed,
            opcode: 0x55,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("x"),
            ],
        },
        // eor [dp],y
        InstructionInfo {
            name: "eor",
            addressing: AddressingMode::IndirectIndexedLong,
            opcode: 0x57,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("y"),
            ],
        },
        // eor absolute,y
        InstructionInfo {
            name: "eor",
            addressing: AddressingMode::Indexed,
            opcode: 0x59,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word16),
                InstructionArgument::Register("y"),
            ],
        },
        // eor absolute,x
        InstructionInfo {
            name: "eor",
            addressing: AddressingMode::Indexed,
            opcode: 0x5D,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word16),
                InstructionArgument::Register("x"),
            ],
        },
        // eor long,x
        InstructionInfo {
            name: "eor",
            addressing: AddressingMode::Indexed,
            opcode: 0x5F,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word24),
                InstructionArgument::Register("x"),
            ],
        },
        // inc
        InstructionInfo {
            name: "inc",
            addressing: AddressingMode::Implied,
            opcode: 0x1A,
            arguments: &[],
        },
        // inc dp
        InstructionInfo {
            name: "inc",
            addressing: AddressingMode::SingleArgument,
            opcode: 0xE6,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // inc absolute
        InstructionInfo {
            name: "inc",
            addressing: AddressingMode::SingleArgument,
            opcode: 0xEE,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word16)],
        },
        // inc dp,x
        InstructionInfo {
            name: "inc",
            addressing: AddressingMode::Indexed,
            opcode: 0xF6,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("x"),
            ],
        },
        // inc absolute,x
        InstructionInfo {
            name: "inc",
            addressing: AddressingMode::Indexed,
            opcode: 0xFE,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word16),
                InstructionArgument::Register("x"),
            ],
        },
        // inx
        InstructionInfo {
            name: "inx",
            addressing: AddressingMode::Implied,
            opcode: 0xE8,
            arguments: &[],
        },
        // iny
        InstructionInfo {
            name: "iny",
            addressing: AddressingMode::Implied,
            opcode: 0xC8,
            arguments: &[],
        },
        // jmp absolute
        InstructionInfo {
            name: "jmp",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x4C,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word16)],
        },
        // jml long
        InstructionInfo {
            name: "jml",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x5C,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word24)],
        },
        // jmp (absolute)
        InstructionInfo {
            name: "jmp",
            addressing: AddressingMode::Indirect,
            opcode: 0x6C,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word16)],
        },
        // jmp (absolute,x)
        InstructionInfo {
            name: "jmp",
            addressing: AddressingMode::IndexedIndirect,
            opcode: 0x7C,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word16),
                InstructionArgument::Register("x"),
            ],
        },
        // jmp [absolute]
        InstructionInfo {
            name: "jmp",
            addressing: AddressingMode::IndirectLong,
            opcode: 0xDC,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word16)],
        },
        // jsr absolute
        InstructionInfo {
            name: "jsr",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x20,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word16)],
        },
        // jsl long
        InstructionInfo {
            name: "jsl",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x22,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word24)],
        },
        // jsr (absolute,x)
        InstructionInfo {
            name: "jsr",
            addressing: AddressingMode::IndexedIndirect,
            opcode: 0xFC,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word16),
                InstructionArgument::Register("x"),
            ],
        },
        // lda (dp,x)
        InstructionInfo {
            name: "lda",
            addressing: AddressingMode::IndexedIndirect,
            opcode: 0xA1,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("x"),
            ],
        },
        // lda sr,s
        InstructionInfo {
            name: "lda",
            addressing: AddressingMode::Indexed,
            opcode: 0xA3,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("s"),
            ],
        },
        // lda dp
        InstructionInfo {
            name: "lda",
            addressing: AddressingMode::SingleArgument,
            opcode: 0xA5,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // lda [dp]
        InstructionInfo {
            name: "lda",
            addressing: AddressingMode::IndirectLong,
            opcode: 0xA7,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // lda #immediate
        InstructionInfo {
            name: "lda",
            addressing: AddressingMode::Immediate,
            opcode: 0xA9,
            arguments: &[
                InstructionArgument::Numbers(&[ArgumentSize::Word8, ArgumentSize::Word16]),
            ],
        },
        // lda absolute
        InstructionInfo {
            name: "lda",
            addressing: AddressingMode::SingleArgument,
            opcode: 0xAD,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word16)],
        },
        // lda long
        InstructionInfo {
            name: "lda",
            addressing: AddressingMode::SingleArgument,
            opcode: 0xAF,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word24)],
        },
        // lda (dp),y
        InstructionInfo {
            name: "lda",
            addressing: AddressingMode::IndirectIndexed,
            opcode: 0xB1,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("y"),
            ],
        },
        // lda (dp)
        InstructionInfo {
            name: "lda",
            addressing: AddressingMode::Indirect,
            opcode: 0xB2,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // lda (byte,s),y
        InstructionInfo {
            name: "lda",
            addressing: AddressingMode::StackRelativeIndirectIndexed,
            opcode: 0xB3,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("s"),
                InstructionArgument::Register("y"),
            ],
        },
        // lda dp,x
        InstructionInfo {
            name: "lda",
            addressing: AddressingMode::Indexed,
            opcode: 0xB5,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("x"),
            ],
        },
        // lda [dp],y
        InstructionInfo {
            name: "lda",
            addressing: AddressingMode::IndirectIndexedLong,
            opcode: 0xB7,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("y"),
            ],
        },
        // lda absolute,y
        InstructionInfo {
            name: "lda",
            addressing: AddressingMode::Indexed,
            opcode: 0xB9,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word16),
                InstructionArgument::Register("y"),
            ],
        },
        // lda absolute,x
        InstructionInfo {
            name: "lda",
            addressing: AddressingMode::Indexed,
            opcode: 0xBD,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word16),
                InstructionArgument::Register("x"),
            ],
        },
        // lda long,x
        InstructionInfo {
            name: "lda",
            addressing: AddressingMode::Indexed,
            opcode: 0xBF,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word24),
                InstructionArgument::Register("x"),
            ],
        },
        // ldx #immediate
        InstructionInfo {
            name: "ldx",
            addressing: AddressingMode::Immediate,
            opcode: 0xA2,
            arguments: &[
                InstructionArgument::Numbers(&[ArgumentSize::Word8, ArgumentSize::Word16]),
            ],
        },
        // ldx dp
        InstructionInfo {
            name: "ldx",
            addressing: AddressingMode::SingleArgument,
            opcode: 0xA6,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // ldx absolute
        InstructionInfo {
            name: "ldx",
            addressing: AddressingMode::SingleArgument,
            opcode: 0xAE,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word16)],
        },
        // ldx dp,y
        InstructionInfo {
            name: "ldx",
            addressing: AddressingMode::Indexed,
            opcode: 0xB6,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("y"),
            ],
        },
        // ldx absolute,y
        InstructionInfo {
            name: "ldx",
            addressing: AddressingMode::Indexed,
            opcode: 0xBE,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word16),
                InstructionArgument::Register("y"),
            ],
        },
        // ldy #immediate
        InstructionInfo {
            name: "ldy",
            addressing: AddressingMode::Immediate,
            opcode: 0xA0,
            arguments: &[
                InstructionArgument::Numbers(&[ArgumentSize::Word8, ArgumentSize::Word16]),
            ],
        },
        // ldy dp
        InstructionInfo {
            name: "ldy",
            addressing: AddressingMode::SingleArgument,
            opcode: 0xA4,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // ldy absolute
        InstructionInfo {
            name: "ldy",
            addressing: AddressingMode::SingleArgument,
            opcode: 0xAC,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word16)],
        },
        // ldy dp,x
        InstructionInfo {
            name: "ldy",
            addressing: AddressingMode::Indexed,
            opcode: 0xB4,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("x"),
            ],
        },
        // ldy absolute,x
        InstructionInfo {
            name: "ldy",
            addressing: AddressingMode::Indexed,
            opcode: 0xBC,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word16),
                InstructionArgument::Register("x"),
            ],
        },
        // lsr dp
        InstructionInfo {
            name: "lsr",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x46,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // lsr
        InstructionInfo {
            name: "lsr",
            addressing: AddressingMode::Implied,
            opcode: 0x4A,
            arguments: &[],
        },
        // lsr absolute
        InstructionInfo {
            name: "lsr",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x4E,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word16)],
        },
        // lsr dp,x
        InstructionInfo {
            name: "lsr",
            addressing: AddressingMode::Indexed,
            opcode: 0x56,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("x"),
            ],
        },
        // lsr absolute,x
        InstructionInfo {
            name: "lsr",
            addressing: AddressingMode::Indexed,
            opcode: 0x5E,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word16),
                InstructionArgument::Register("x"),
            ],
        },
        // mvn byte,byte
        InstructionInfo {
            name: "mvn",
            addressing: AddressingMode::BlockMove,
            opcode: 0x54,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Number(ArgumentSize::Word8),
            ],
        },
        // mvp byte,byte
        InstructionInfo {
            name: "mvp",
            addressing: AddressingMode::BlockMove,
            opcode: 0x44,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Number(ArgumentSize::Word8),
            ],
        },
        // nop
        InstructionInfo {
            name: "nop",
            addressing: AddressingMode::Implied,
            opcode: 0xEA,
            arguments: &[],
        },
        // ora (dp,x)
        InstructionInfo {
            name: "ora",
            addressing: AddressingMode::IndexedIndirect,
            opcode: 0x01,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("x"),
            ],
        },
        // ora sr,s
        InstructionInfo {
            name: "ora",
            addressing: AddressingMode::Indexed,
            opcode: 0x03,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("s"),
            ],
        },
        // ora dp
        InstructionInfo {
            name: "ora",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x05,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // ora [dp]
        InstructionInfo {
            name: "ora",
            addressing: AddressingMode::IndirectLong,
            opcode: 0x07,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // ora #immediate
        InstructionInfo {
            name: "ora",
            addressing: AddressingMode::Immediate,
            opcode: 0x09,
            arguments: &[
                InstructionArgument::Numbers(&[ArgumentSize::Word8, ArgumentSize::Word16]),
            ],
        },
        // ora absolute
        InstructionInfo {
            name: "ora",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x0D,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word16)],
        },
        // ora long
        InstructionInfo {
            name: "ora",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x0F,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word24)],
        },
        // ora (dp),y
        InstructionInfo {
            name: "ora",
            addressing: AddressingMode::IndirectIndexed,
            opcode: 0x11,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("y"),
            ],
        },
        // ora (dp)
        InstructionInfo {
            name: "ora",
            addressing: AddressingMode::Indirect,
            opcode: 0x12,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // ora (sr,s),y
        InstructionInfo {
            name: "ora",
            addressing: AddressingMode::StackRelativeIndirectIndexed,
            opcode: 0x13,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("s"),
                InstructionArgument::Register("y"),
            ],
        },
        // ora dp,x
        InstructionInfo {
            name: "ora",
            addressing: AddressingMode::Indexed,
            opcode: 0x15,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("x"),
            ],
        },
        // ora [dp],y
        InstructionInfo {
            name: "ora",
            addressing: AddressingMode::IndirectIndexedLong,
            opcode: 0x17,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("y"),
            ],
        },
        // ora absolute,y
        InstructionInfo {
            name: "ora",
            addressing: AddressingMode::Indexed,
            opcode: 0x19,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word16),
                InstructionArgument::Register("y"),
            ],
        },
        // ora absolute,x
        InstructionInfo {
            name: "ora",
            addressing: AddressingMode::Indexed,
            opcode: 0x1D,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word16),
                InstructionArgument::Register("x"),
            ],
        },
        // ora long,x
        InstructionInfo {
            name: "ora",
            addressing: AddressingMode::Indexed,
            opcode: 0x1F,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word24),
                InstructionArgument::Register("x"),
            ],
        },
        // pea absolute
        InstructionInfo {
            name: "pea",
            addressing: AddressingMode::SingleArgument,
            opcode: 0xF4,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word16)],
        },
        // pei (dp)
        InstructionInfo {
            name: "pei",
            addressing: AddressingMode::Indirect,
            opcode: 0xD4,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // per absolute
        InstructionInfo {
            name: "per",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x62,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word16)],
        },
        // pha
        InstructionInfo {
            name: "pha",
            addressing: AddressingMode::Implied,
            opcode: 0x48,
            arguments: &[],
        },
        // phb
        InstructionInfo {
            name: "phb",
            addressing: AddressingMode::Implied,
            opcode: 0x8B,
            arguments: &[],
        },
        // phd
        InstructionInfo {
            name: "phd",
            addressing: AddressingMode::Implied,
            opcode: 0x0B,
            arguments: &[],
        },
        // phk
        InstructionInfo {
            name: "phk",
            addressing: AddressingMode::Implied,
            opcode: 0x4B,
            arguments: &[],
        },
        // php
        InstructionInfo {
            name: "php",
            addressing: AddressingMode::Implied,
            opcode: 0x08,
            arguments: &[],
        },
        // phx
        InstructionInfo {
            name: "phx",
            addressing: AddressingMode::Implied,
            opcode: 0xDA,
            arguments: &[],
        },
        // phy
        InstructionInfo {
            name: "pha",
            addressing: AddressingMode::Implied,
            opcode: 0x5A,
            arguments: &[],
        },
        // pla
        InstructionInfo {
            name: "pla",
            addressing: AddressingMode::Implied,
            opcode: 0x68,
            arguments: &[],
        },
        // plb
        InstructionInfo {
            name: "plb",
            addressing: AddressingMode::Implied,
            opcode: 0xAB,
            arguments: &[],
        },
        // pld
        InstructionInfo {
            name: "pld",
            addressing: AddressingMode::Implied,
            opcode: 0x2B,
            arguments: &[],
        },
        // plp
        InstructionInfo {
            name: "plp",
            addressing: AddressingMode::Implied,
            opcode: 0x28,
            arguments: &[],
        },
        // plx
        InstructionInfo {
            name: "plx",
            addressing: AddressingMode::Implied,
            opcode: 0xFA,
            arguments: &[],
        },
        // ply
        InstructionInfo {
            name: "ply",
            addressing: AddressingMode::Implied,
            opcode: 0x7A,
            arguments: &[],
        },
        // rep #immediate
        InstructionInfo {
            name: "rep",
            addressing: AddressingMode::Immediate,
            opcode: 0xC2,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // rol dp
        InstructionInfo {
            name: "rol",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x26,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // rol
        InstructionInfo {
            name: "rol",
            addressing: AddressingMode::Implied,
            opcode: 0x2A,
            arguments: &[],
        },
        // rol absolute
        InstructionInfo {
            name: "lsr",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x2E,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word16)],
        },
        // rol dp,x
        InstructionInfo {
            name: "rol",
            addressing: AddressingMode::Indexed,
            opcode: 0x36,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("x"),
            ],
        },
        // rol absolute,x
        InstructionInfo {
            name: "rol",
            addressing: AddressingMode::Indexed,
            opcode: 0x3E,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word16),
                InstructionArgument::Register("x"),
            ],
        },
        // ror dp
        InstructionInfo {
            name: "ror",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x66,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // ror
        InstructionInfo {
            name: "ror",
            addressing: AddressingMode::Implied,
            opcode: 0x6A,
            arguments: &[],
        },
        // ror absolute
        InstructionInfo {
            name: "ror",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x6E,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word16)],
        },
        // ror dp,x
        InstructionInfo {
            name: "ror",
            addressing: AddressingMode::Indexed,
            opcode: 0x76,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("x"),
            ],
        },
        // ror absolute,x
        InstructionInfo {
            name: "ror",
            addressing: AddressingMode::Indexed,
            opcode: 0x7E,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word16),
                InstructionArgument::Register("x"),
            ],
        },
        // rti
        InstructionInfo {
            name: "rti",
            addressing: AddressingMode::Implied,
            opcode: 0x40,
            arguments: &[],
        },
        // rtl
        InstructionInfo {
            name: "rtl",
            addressing: AddressingMode::Implied,
            opcode: 0x6B,
            arguments: &[],
        },
        // rts
        InstructionInfo {
            name: "rts",
            addressing: AddressingMode::Implied,
            opcode: 0x60,
            arguments: &[],
        },
        // sbc (dp,x)
        InstructionInfo {
            name: "sbc",
            addressing: AddressingMode::IndexedIndirect,
            opcode: 0xE1,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("x"),
            ],
        },
        // sbc byte,s
        InstructionInfo {
            name: "sbc",
            addressing: AddressingMode::Indexed,
            opcode: 0xE3,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("s"),
            ],
        },
        // sbc dp
        InstructionInfo {
            name: "sbc",
            addressing: AddressingMode::SingleArgument,
            opcode: 0xE5,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // sbc [dp]
        InstructionInfo {
            name: "sbc",
            addressing: AddressingMode::IndirectLong,
            opcode: 0xE7,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // sbc #number
        InstructionInfo {
            name: "sbc",
            addressing: AddressingMode::Immediate,
            opcode: 0xE9,
            arguments: &[
                InstructionArgument::Numbers(&[ArgumentSize::Word8, ArgumentSize::Word16]),
            ],
        },
        // sbc absolute
        InstructionInfo {
            name: "sbc",
            addressing: AddressingMode::SingleArgument,
            opcode: 0xED,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word16)],
        },
        // sbc long
        InstructionInfo {
            name: "sbc",
            addressing: AddressingMode::SingleArgument,
            opcode: 0xEF,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word24)],
        },
        // sbc (dp),y
        InstructionInfo {
            name: "sbc",
            addressing: AddressingMode::IndirectIndexed,
            opcode: 0xF1,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("y"),
            ],
        },
        // sbc (dp)
        InstructionInfo {
            name: "sbc",
            addressing: AddressingMode::Indirect,
            opcode: 0xF2,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // sbc (sr,s),y
        InstructionInfo {
            name: "sbc",
            addressing: AddressingMode::StackRelativeIndirectIndexed,
            opcode: 0xF3,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("s"),
                InstructionArgument::Register("y"),
            ],
        },
        // sbc dp,x
        InstructionInfo {
            name: "sbc",
            addressing: AddressingMode::Indexed,
            opcode: 0xF5,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("x"),
            ],
        },
        // sbc [dp],y
        InstructionInfo {
            name: "sbc",
            addressing: AddressingMode::IndirectIndexedLong,
            opcode: 0xF7,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("y"),
            ],
        },
        // sbc absolute,y
        InstructionInfo {
            name: "sbc",
            addressing: AddressingMode::Indexed,
            opcode: 0xF9,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word16),
                InstructionArgument::Register("y"),
            ],
        },
        // sbc absolute,x
        InstructionInfo {
            name: "sbc",
            addressing: AddressingMode::Indexed,
            opcode: 0xFD,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word16),
                InstructionArgument::Register("x"),
            ],
        },
        // sbc long,x
        InstructionInfo {
            name: "sbc",
            addressing: AddressingMode::Indexed,
            opcode: 0xFF,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word24),
                InstructionArgument::Register("x"),
            ],
        },
        // sec
        InstructionInfo {
            name: "sec",
            addressing: AddressingMode::Implied,
            opcode: 0x38,
            arguments: &[],
        },
        // sed
        InstructionInfo {
            name: "sed",
            addressing: AddressingMode::Implied,
            opcode: 0xF8,
            arguments: &[],
        },
        // sei
        InstructionInfo {
            name: "sei",
            addressing: AddressingMode::Implied,
            opcode: 0x78,
            arguments: &[],
        },
        // sep #immediate
        InstructionInfo {
            name: "sep",
            addressing: AddressingMode::Immediate,
            opcode: 0xE2,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // sta (dp,x)
        InstructionInfo {
            name: "sta",
            addressing: AddressingMode::IndexedIndirect,
            opcode: 0x81,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("x"),
            ],
        },
        // sta sr,s
        InstructionInfo {
            name: "sta",
            addressing: AddressingMode::Indexed,
            opcode: 0x83,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("s"),
            ],
        },
        // sta dp
        InstructionInfo {
            name: "sta",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x85,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // sta [dp]
        InstructionInfo {
            name: "sta",
            addressing: AddressingMode::IndirectLong,
            opcode: 0x87,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // sta absolute
        InstructionInfo {
            name: "sta",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x8D,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word16)],
        },
        // sta long
        InstructionInfo {
            name: "sta",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x8F,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word24)],
        },
        // sta (dp),y
        InstructionInfo {
            name: "sta",
            addressing: AddressingMode::IndirectIndexed,
            opcode: 0x91,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("y"),
            ],
        },
        // sta (dp)
        InstructionInfo {
            name: "sta",
            addressing: AddressingMode::Indirect,
            opcode: 0x92,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // sta (byte,s),y
        InstructionInfo {
            name: "sta",
            addressing: AddressingMode::StackRelativeIndirectIndexed,
            opcode: 0x93,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("s"),
                InstructionArgument::Register("y"),
            ],
        },
        // sta dp,x
        InstructionInfo {
            name: "sta",
            addressing: AddressingMode::Indexed,
            opcode: 0x95,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("x"),
            ],
        },
        // sta [dp],y
        InstructionInfo {
            name: "sta",
            addressing: AddressingMode::IndirectIndexedLong,
            opcode: 0x97,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("y"),
            ],
        },
        // sta absolute,y
        InstructionInfo {
            name: "sta",
            addressing: AddressingMode::Indexed,
            opcode: 0x99,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word16),
                InstructionArgument::Register("y"),
            ],
        },
        // sta absolute,x
        InstructionInfo {
            name: "sta",
            addressing: AddressingMode::Indexed,
            opcode: 0x9D,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word16),
                InstructionArgument::Register("x"),
            ],
        },
        // sta long,x
        InstructionInfo {
            name: "sta",
            addressing: AddressingMode::Indexed,
            opcode: 0x9F,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word24),
                InstructionArgument::Register("x"),
            ],
        },
        // stp
        InstructionInfo {
            name: "stp",
            addressing: AddressingMode::Implied,
            opcode: 0xDB,
            arguments: &[],
        },
        // stx dp
        InstructionInfo {
            name: "stx",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x86,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // stx absolute
        InstructionInfo {
            name: "stx",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x8E,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word16)],
        },
        // stx dp,y
        InstructionInfo {
            name: "stx",
            addressing: AddressingMode::Indexed,
            opcode: 0x96,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("y"),
            ],
        },
        // sty dp
        InstructionInfo {
            name: "sty",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x84,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // sty absolute
        InstructionInfo {
            name: "sty",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x8C,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word16)],
        },
        // sty dp,x
        InstructionInfo {
            name: "sty",
            addressing: AddressingMode::Indexed,
            opcode: 0x94,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("x"),
            ],
        },
        // stz dp
        InstructionInfo {
            name: "stz",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x64,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // sty dp,x
        InstructionInfo {
            name: "stz",
            addressing: AddressingMode::Indexed,
            opcode: 0x74,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word8),
                InstructionArgument::Register("x"),
            ],
        },
        // stz absolute
        InstructionInfo {
            name: "stz",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x9C,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word16)],
        },
        // stz absolute,x
        InstructionInfo {
            name: "stz",
            addressing: AddressingMode::Indexed,
            opcode: 0x9E,
            arguments: &[
                InstructionArgument::Number(ArgumentSize::Word16),
                InstructionArgument::Register("x"),
            ],
        },
        // tax
        InstructionInfo {
            name: "tax",
            addressing: AddressingMode::Implied,
            opcode: 0xAA,
            arguments: &[],
        },
        // tay
        InstructionInfo {
            name: "tay",
            addressing: AddressingMode::Implied,
            opcode: 0xA8,
            arguments: &[],
        },
        // tcd
        InstructionInfo {
            name: "tcd",
            addressing: AddressingMode::Implied,
            opcode: 0x5B,
            arguments: &[],
        },
        // tcs
        InstructionInfo {
            name: "tcs",
            addressing: AddressingMode::Implied,
            opcode: 0x1B,
            arguments: &[],
        },
        // tdc
        InstructionInfo {
            name: "tdc",
            addressing: AddressingMode::Implied,
            opcode: 0x7B,
            arguments: &[],
        },
        // trb dp
        InstructionInfo {
            name: "trb",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x14,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // trb absolute
        InstructionInfo {
            name: "trb",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x1C,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word16)],
        },
        // tsb dp
        InstructionInfo {
            name: "tsb",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x04,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word8)],
        },
        // tsb absolute
        InstructionInfo {
            name: "tsb",
            addressing: AddressingMode::SingleArgument,
            opcode: 0x0C,
            arguments: &[InstructionArgument::Number(ArgumentSize::Word16)],
        },
        // tsc
        InstructionInfo {
            name: "tsc",
            addressing: AddressingMode::Implied,
            opcode: 0x3B,
            arguments: &[],
        },
        // tsx
        InstructionInfo {
            name: "tsx",
            addressing: AddressingMode::Implied,
            opcode: 0xBA,
            arguments: &[],
        },
        // txa
        InstructionInfo {
            name: "txa",
            addressing: AddressingMode::Implied,
            opcode: 0x8A,
            arguments: &[],
        },
        // txs
        InstructionInfo {
            name: "txs",
            addressing: AddressingMode::Implied,
            opcode: 0x9A,
            arguments: &[],
        },
        // txy
        InstructionInfo {
            name: "txa",
            addressing: AddressingMode::Implied,
            opcode: 0x9B,
            arguments: &[],
        },
        // tya
        InstructionInfo {
            name: "tya",
            addressing: AddressingMode::Implied,
            opcode: 0x98,
            arguments: &[],
        },
        // tyx
        InstructionInfo {
            name: "tyx",
            addressing: AddressingMode::Implied,
            opcode: 0xBB,
            arguments: &[],
        },
        // wai
        InstructionInfo {
            name: "wai",
            addressing: AddressingMode::Implied,
            opcode: 0xCB,
            arguments: &[],
        },
        // wdm
        InstructionInfo {
            name: "wdm",
            addressing: AddressingMode::Implied,
            opcode: 0x42,
            arguments: &[],
        },
        // xba
        InstructionInfo {
            name: "xba",
            addressing: AddressingMode::Implied,
            opcode: 0xEB,
            arguments: &[],
        },
        // xce
        InstructionInfo {
            name: "xce",
            addressing: AddressingMode::Implied,
            opcode: 0xFB,
            arguments: &[],
        },
    ],
};
