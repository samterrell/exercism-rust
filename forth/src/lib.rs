mod lexer;
mod parser;
use std::borrow::Borrow;
use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;

use crate::lexer::Tokenizer;
use crate::parser::{Instruction, Routine};

pub type Value = i32;
pub type Result = std::result::Result<(), Error>;

#[derive(Debug)]
pub struct Forth {
    stack: Stack,
    routines: FnTable,
}

#[derive(Debug)]
struct FnTable {
    inner: RefCell<HashMap<String, Rc<Routine>>>,
}

#[derive(Debug)]
struct Stack {
    inner: Vec<Value>,
}

struct ProgramCounter {
    routine: Rc<Routine>,
    offset: usize,
}

impl ProgramCounter {
    fn new(routine: Rc<Routine>) -> Self {
        Self {
            routine: Rc::clone(&routine),
            offset: 0,
        }
    }
}

impl Iterator for ProgramCounter {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        let routine: &Routine = self.routine.borrow();
        if self.offset >= routine.instructions.len() {
            None
        } else {
            let res = routine.instructions[self.offset].clone();
            self.offset += 1;
            Some(res)
        }
    }
}

// impl Program {
//     fn new(main: Routine) -> Self {
//         let mut segments = HashMap::new();
//         segments.insert(Segment::Main, Rc::new(main));
//         Program {
//             segments: RefCell::new(segments),
//         }
//     }

//     fn iter(&self) -> ProgramIter {
//         ProgramIter::new(&self)
//     }
// }

struct ProgramIter {
    pc: Vec<ProgramCounter>,
}

impl ProgramIter {
    fn new() -> Self {
        Self { pc: vec![] }
    }

    fn call(&mut self, routine: Rc<Routine>) {
        self.pc.push(ProgramCounter::new(routine));
    }
}

impl Iterator for ProgramIter {
    type Item = Instruction;

    fn next(&mut self) -> Option<Self::Item> {
        let mut pc = self.pc.pop()?;
        if let Some(instruction) = pc.next() {
            self.pc.push(pc);
            Some(instruction)
        } else {
            self.next()
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    DivisionByZero,
    StackUnderflow,
    UnknownWord,
    InvalidWord,
}

impl Stack {
    fn new() -> Self {
        Self { inner: vec![] }
    }

    fn stack(&self) -> &[Value] {
        &self.inner[..]
    }

    fn pop(&mut self) -> std::result::Result<Value, Error> {
        match self.inner.pop() {
            Some(value) => Ok(value),
            None => Err(Error::StackUnderflow),
        }
    }

    fn push(&mut self, value: Value) {
        self.inner.push(value);
    }

    fn peek(&self) -> std::result::Result<Value, Error> {
        let len = self.inner.len();
        if len > 0 {
            Ok(self.inner[len - 1])
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn drop(&mut self) -> std::result::Result<(), Error> {
        self.pop()?;
        Ok(())
    }

    fn swap(&mut self) -> std::result::Result<(), Error> {
        let len = self.inner.len();
        if len >= 2 {
            (self.inner[len - 2], self.inner[len - 1]) = (self.inner[len - 1], self.inner[len - 2]);
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }

    fn over(&mut self) -> std::result::Result<(), Error> {
        let len = self.inner.len();
        if len >= 2 {
            self.inner.push(self.inner[len - 2]);
            Ok(())
        } else {
            Err(Error::StackUnderflow)
        }
    }
}

impl Forth {
    pub fn new() -> Self {
        Forth {
            stack: Stack::new(),
            routines: FnTable::new(),
        }
    }

    pub fn stack(&self) -> &[Value] {
        self.stack.stack()
    }

    pub fn eval(&mut self, input: &str) -> Result {
        println!("{}", input);
        let tokenizer = Tokenizer::new(input);
        let mut program = {
            let program = Routine::parse(&tokenizer)?;
            // let pc = ProgramCounter::new(Rc::new(program));
            let mut iter = ProgramIter::new();
            iter.call(Rc::new(program));
            iter
        };
        // println!("{:?}", program);
        // Ok(())
        interpret(&mut self.stack, &self.routines, &mut program)
    }
}

impl FnTable {
    fn new() -> Self {
        Self {
            inner: RefCell::new(HashMap::new()),
        }
    }

    fn define(&self, word: String, routine: &Rc<Routine>) {
        self.inner.borrow_mut().insert(word, Rc::clone(routine));
    }

    fn get(&self, word: &String) -> std::result::Result<Rc<Routine>, Error> {
        let segments = self.inner.borrow();
        if let Some(segment) = segments.get(word) {
            Ok(Rc::clone(segment))
        } else {
            Err(Error::UnknownWord)
        }
    }
}

fn interpret(stack: &mut Stack, routines: &FnTable, program: &mut ProgramIter) -> Result {
    println!("Interpret start");
    while let Some(instruction) = program.next() {
        println!("Execute: {:?}", instruction);
        match instruction {
            Instruction::Add => {
                let r = stack.pop()?;
                let l = stack.pop()?;
                stack.push(l + r);
            }
            Instruction::Subtract => {
                let r = stack.pop()?;
                let l = stack.pop()?;
                stack.push(l - r);
            }
            Instruction::Multiply => {
                let r = stack.pop()?;
                let l = stack.pop()?;
                stack.push(l * r);
            }
            Instruction::Divide => {
                let r = stack.pop()?;
                let l = stack.pop()?;
                if r == 0 {
                    return Err(Error::DivisionByZero);
                } else {
                    stack.push(l / r);
                }
            }
            Instruction::Duplicate => {
                let l = stack.peek()?;
                stack.push(l);
            }
            Instruction::Drop => stack.drop()?,
            Instruction::Swap => stack.swap()?,
            Instruction::Over => stack.over()?,
            Instruction::Call(word) => program.call(routines.get(&word)?),
            Instruction::Push(v) => stack.push(v),
            Instruction::Define(word, routine) => routines.define(word.to_string(), &routine),
        }
    }
    Ok(())
}
