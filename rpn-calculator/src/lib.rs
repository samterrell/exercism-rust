#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    let mut stack = vec![];
    for value in inputs {
        match value {
            &CalculatorInput::Add => {
                let l = stack.pop()?;
                let r = stack.pop()?;
                stack.push(r + l);
            }
            &CalculatorInput::Subtract => {
                let l = stack.pop()?;
                let r = stack.pop()?;
                stack.push(r - l);
            }
            &CalculatorInput::Multiply => {
                let l = stack.pop()?;
                let r = stack.pop()?;
                stack.push(r * l);
            }
            &CalculatorInput::Divide => {
                let l = stack.pop()?;
                let r = stack.pop()?;
                stack.push(r / l);
            }
            &CalculatorInput::Value(v) => stack.push(v),
        }
    }
    if stack.len() == 1 {
        stack.pop()
    } else {
        None
    }
}
