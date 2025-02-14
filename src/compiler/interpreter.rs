use crate::compiler::parser::OpCode;

// Executes the IR opcodes
pub fn exec(ir: Vec<OpCode>) {
    for opcode in ir {
        if let OpCode::Echo(temp) = opcode {
            println!("{}", &temp[1..temp.len() - 1]);
        }
    }
}
