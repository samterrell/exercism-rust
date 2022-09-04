use std::rc::Rc;

use crate::lexer::{Token, TokenIter};

use super::{Error, Tokenizer, Value};

#[derive(Debug)]
pub struct Routine {
    pub instructions: Vec<Instruction>,
}

#[derive(Debug, Clone)]
pub enum Instruction {
    Add,
    Subtract,
    Multiply,
    Divide,
    Duplicate,
    Drop,
    Swap,
    Over,
    Call(String),
    Push(Value),
    Define(String, Rc<Routine>),
}

impl Routine {
    pub fn new() -> Routine {
        Routine {
            instructions: vec![],
        }
    }
    pub fn parse(tokenizer: &Tokenizer) -> Result<Routine, Error> {
        let mut routine = Self::new();
        parse_forth(&mut routine, &mut tokenizer.iter(), State::Global)?;
        Ok(routine)
    }

    fn push(&mut self, instruction: Instruction) {
        self.instructions.push(instruction);
    }
}

enum State {
    Global,
    Define,
}

fn parse_forth(routine: &mut Routine, iter: &mut TokenIter, state: State) -> Result<(), Error> {
    let next = if let Some(Ok(token)) = iter.next() {
        token
    } else {
        return Err(Error::InvalidWord);
    };
    match next {
        Token::End => Ok(()),
        Token::Colon => {
            let (name, sub) = parse_subroutine(iter)?;
            routine.push(Instruction::Define(
                name.to_ascii_uppercase().to_string(),
                Rc::new(sub),
            ));
            parse_forth(routine, iter, state)
        }
        Token::Plus => {
            routine.push(Instruction::Add);
            parse_forth(routine, iter, state)
        }
        Token::Minus => {
            routine.push(Instruction::Subtract);
            parse_forth(routine, iter, state)
        }
        Token::Asterisk => {
            routine.push(Instruction::Multiply);
            parse_forth(routine, iter, state)
        }
        Token::Slash => {
            routine.push(Instruction::Divide);
            parse_forth(routine, iter, state)
        }
        Token::Dup => {
            routine.push(Instruction::Duplicate);
            parse_forth(routine, iter, state)
        }
        Token::Drop => {
            routine.push(Instruction::Drop);
            parse_forth(routine, iter, state)
        }
        Token::Swap => {
            routine.push(Instruction::Swap);
            parse_forth(routine, iter, state)
        }
        Token::Over => {
            routine.push(Instruction::Over);
            parse_forth(routine, iter, state)
        }
        Token::Word(str) => {
            routine.push(Instruction::Call(str.to_ascii_uppercase().to_string()));
            parse_forth(routine, iter, state)
        }
        Token::Value(value) => {
            routine.push(Instruction::Push(value));
            parse_forth(routine, iter, state)
        }
        Token::Semicolon => match state {
            State::Define => Ok(()),
            State::Global => Err(Error::InvalidWord),
        },
    }
}

fn parse_subroutine(iter: &mut TokenIter) -> Result<(String, Routine), Error> {
    let mut routine = Routine::new();
    let name = if let Some(Ok(Token::Word(name))) = iter.next() {
        name.to_owned()
    } else {
        return Err(Error::InvalidWord);
    };
    parse_forth(&mut routine, iter, State::Define)?;
    Ok((name, routine))
}
