#[derive(Debug, Clone)]
pub enum Opcode {
    Push(i64),
    Pop,
    Add,
    Sub,
    Mul,
    Div,
    Dup,
    Swap,
    Over,
    Rot,
    Eq,
    Lt,
    Gt,
    JmpIf(usize),
    Jmp(usize),
    Call(usize),
    Ret,
    Halt,
}

pub struct VM {
    stack: Vec<i64>,
    call_stack: Vec<usize>,
}

impl VM {
    pub fn new() -> Self {
        VM {
            stack: Vec::new(),
            call_stack: Vec::new(),
        }
    }

    pub fn execute(&mut self, program: &[Opcode]) -> Result<Vec<i64>, String> {
        self.stack.clear();
        self.call_stack.clear();
        let mut pc = 0;

        while pc < program.len() {
            match &program[pc] {
                Opcode::Push(val) => {
                    self.stack.push(*val);
                    pc += 1;
                }
                Opcode::Pop => {
                    self.pop()?;
                    pc += 1;
                }
                Opcode::Add => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.stack.push(a + b);
                    pc += 1;
                }
                Opcode::Sub => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.stack.push(a - b);
                    pc += 1;
                }
                Opcode::Mul => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.stack.push(a * b);
                    pc += 1;
                }
                Opcode::Div => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    if b == 0 {
                        return Err("Division by zero".to_string());
                    }
                    self.stack.push(a / b);
                    pc += 1;
                }
                Opcode::Dup => {
                    let a = self.peek()?;
                    self.stack.push(a);
                    pc += 1;
                }
                Opcode::Swap => {
                    let len = self.stack.len();
                    if len < 2 {
                        return Err("Stack underflow".to_string());
                    }
                    self.stack.swap(len - 1, len - 2);
                    pc += 1;
                }
                Opcode::Over => {
                    let len = self.stack.len();
                    if len < 2 {
                        return Err("Stack underflow".to_string());
                    }
                    let val = self.stack[len - 2];
                    self.stack.push(val);
                    pc += 1;
                }
                Opcode::Rot => {
                    // a b c -> b c a
                    let len = self.stack.len();
                    if len < 3 {
                        return Err("Stack underflow".to_string());
                    }
                    let a = self.stack[len - 3];
                    self.stack[len - 3] = self.stack[len - 2];
                    self.stack[len - 2] = self.stack[len - 1];
                    self.stack[len - 1] = a;
                    pc += 1;
                }
                Opcode::Eq => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.stack.push(if a == b { 1 } else { 0 });
                    pc += 1;
                }
                Opcode::Lt => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.stack.push(if a < b { 1 } else { 0 });
                    pc += 1;
                }
                Opcode::Gt => {
                    let b = self.pop()?;
                    let a = self.pop()?;
                    self.stack.push(if a > b { 1 } else { 0 });
                    pc += 1;
                }
                Opcode::JmpIf(addr) => {
                    let val = self.pop()?;
                    if val != 0 {
                        pc = *addr;
                    } else {
                        pc += 1;
                    }
                }
                Opcode::Jmp(addr) => {
                    pc = *addr;
                }
                Opcode::Call(addr) => {
                    self.call_stack.push(pc + 1);
                    pc = *addr;
                }
                Opcode::Ret => {
                    pc = self
                        .call_stack
                        .pop()
                        .ok_or_else(|| "Call stack underflow".to_string())?;
                }
                Opcode::Halt => {
                    return Ok(self.stack.clone());
                }
            }
        }

        Ok(self.stack.clone())
    }

    fn pop(&mut self) -> Result<i64, String> {
        self.stack.pop().ok_or_else(|| "Stack underflow".to_string())
    }

    fn peek(&self) -> Result<i64, String> {
        self.stack.last().copied().ok_or_else(|| "Stack underflow".to_string())
    }
}
