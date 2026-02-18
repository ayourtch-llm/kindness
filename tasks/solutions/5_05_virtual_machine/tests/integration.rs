use solution::*;

#[test]
fn basic_arithmetic() {
    let mut vm = VM::new();
    let prog = vec![
        Opcode::Push(10),
        Opcode::Push(20),
        Opcode::Add,
        Opcode::Push(3),
        Opcode::Mul,
        Opcode::Halt,
    ];
    let stack = vm.execute(&prog).unwrap();
    assert_eq!(stack, vec![90]);
}

#[test]
fn stack_manipulation() {
    let mut vm = VM::new();
    let prog = vec![
        Opcode::Push(1),
        Opcode::Push(2),
        Opcode::Push(3),
        Opcode::Rot,      // 1 2 3 -> 2 3 1
        Opcode::Halt,
    ];
    let stack = vm.execute(&prog).unwrap();
    assert_eq!(stack, vec![2, 3, 1]);
}

#[test]
fn conditional_jump() {
    let mut vm = VM::new();
    // if 5 > 3, push 100, else push 200
    let prog = vec![
        Opcode::Push(5),     // 0
        Opcode::Push(3),     // 1
        Opcode::Gt,          // 2: pushes 1
        Opcode::JmpIf(6),   // 3: jump to 6 if true
        Opcode::Push(200),   // 4
        Opcode::Jmp(7),      // 5: skip over
        Opcode::Push(100),   // 6: true branch
        Opcode::Halt,        // 7
    ];
    let stack = vm.execute(&prog).unwrap();
    assert_eq!(stack, vec![100]);
}

#[test]
fn call_and_return() {
    let mut vm = VM::new();
    // Main: push 5, call double function at addr 5, halt
    // Double fn at addr 5: dup, add, ret
    let prog = vec![
        Opcode::Push(5),     // 0
        Opcode::Call(4),     // 1: call fn at 4, return to 2
        Opcode::Halt,        // 2
        Opcode::Halt,        // 3: padding
        Opcode::Dup,         // 4: fn start - dup top
        Opcode::Add,         // 5: add (doubles)
        Opcode::Ret,         // 6: return
    ];
    let stack = vm.execute(&prog).unwrap();
    assert_eq!(stack, vec![10]);
}

#[test]
fn division_by_zero() {
    let mut vm = VM::new();
    let prog = vec![
        Opcode::Push(10),
        Opcode::Push(0),
        Opcode::Div,
        Opcode::Halt,
    ];
    let result = vm.execute(&prog);
    assert!(result.is_err());
}

#[test]
fn stack_underflow_error() {
    let mut vm = VM::new();
    let prog = vec![
        Opcode::Push(1),
        Opcode::Add, // only one value on stack
        Opcode::Halt,
    ];
    let result = vm.execute(&prog);
    assert!(result.is_err());
}

#[test]
fn comparison_ops() {
    let mut vm = VM::new();
    let prog = vec![
        Opcode::Push(3),
        Opcode::Push(3),
        Opcode::Eq,       // 3 == 3 -> 1
        Opcode::Push(2),
        Opcode::Push(5),
        Opcode::Lt,       // 2 < 5 -> 1
        Opcode::Add,      // 1 + 1 = 2
        Opcode::Halt,
    ];
    let stack = vm.execute(&prog).unwrap();
    assert_eq!(stack, vec![2]);
}

#[test]
fn swap_over_pop_sub() {
    let mut vm = VM::new();
    let prog = vec![
        Opcode::Push(10),
        Opcode::Push(3),
        Opcode::Swap,       // 3 10
        Opcode::Over,       // 3 10 3
        Opcode::Sub,        // 3 (10-3=7)
        Opcode::Push(99),
        Opcode::Pop,        // discard 99: stack is [3, 7]
        Opcode::Halt,
    ];
    let stack = vm.execute(&prog).unwrap();
    assert_eq!(stack, vec![3, 7]);
}

#[test]
fn nested_calls() {
    let mut vm = VM::new();
    // main: push 2, call triple at 4, halt
    // triple (addr 4): dup, call double at 9, swap, add, ret
    // double (addr 9): dup, add, ret
    let prog = vec![
        Opcode::Push(2),     // 0
        Opcode::Call(4),     // 1: call triple
        Opcode::Halt,        // 2
        Opcode::Halt,        // 3: padding
        // triple fn at 4:
        Opcode::Dup,         // 4: [2, 2]
        Opcode::Call(9),     // 5: call double -> [2, 4], ret to 6
        Opcode::Swap,        // 6: [4, 2]
        Opcode::Add,         // 7: [6]
        Opcode::Ret,         // 8
        // double fn at 9:
        Opcode::Dup,         // 9: dup top
        Opcode::Add,         // 10: double it
        Opcode::Ret,         // 11
    ];
    let stack = vm.execute(&prog).unwrap();
    assert_eq!(stack, vec![6]);
}
