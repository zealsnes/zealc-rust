use zeal::lexer::*;
use zeal::system_definition::*;

pub enum LiteralExpression {
    NumberLiteralExpression(NumberLiteral),
}

pub enum Expression {
    ImpliedInstruction(&'static InstructionInfo),
    SingleArgumentInstruction(&'static InstructionInfo, LiteralExpression),
}

pub struct Parser<'a> {
    lexers: Vec<Lexer<'a>>,
    system: &'static SystemDefinition,
}

impl<'a> Parser<'a> {
    pub fn new(system: &'static SystemDefinition, lexer: Lexer<'a>) -> Self {
        let mut lexers = Vec::new();
        lexers.push(lexer);

        Parser {
            lexers: lexers,
            system: system,
        }
    }

    pub fn parse_tree(&mut self) -> Vec<Expression> {
        let mut expressions = Vec::new();

        loop {
            match self.parse() {
                Some(expression) => expressions.push(expression),
                None => break,
            }
        }

        return expressions;
    }

    fn parse(&mut self) -> Option<Expression> {
        // root = (cpuInstruction)* ;
        let token = self.get_next_token();
        match token.ttype {
            TokenType::Invalid(_) => return None,
            TokenType::EndOfFile => return None,
            TokenType::Identifier(ident) => self.parse_cpu_instruction(ident),
            _ => return None,
        }
    }

    fn parse_cpu_instruction(&mut self, ident: String) -> Option<Expression> {
        for instruction in self.system.instructions.iter() {
            if instruction.name == ident {
                return Some(Expression::ImpliedInstruction(instruction));
            }
        }

        return None;
    }

    fn get_next_token(&mut self) -> Token {
        self.lexer().unwrap().get_next_token()
    }

    fn lexer(&mut self) -> Option<&mut Lexer<'a>> {
        self.lexers.last_mut()
    }
}
