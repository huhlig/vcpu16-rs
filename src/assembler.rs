use std::collections::HashMap;

pub struct Instruction {
    address: usize,
    line: usize,
    opcode: ,
    u_argument: ,
    m_argument: ,
    comment: Option<String>,
}

pub struct Context {
    source: String,
    output: Vec<u16>,
    labels: HashMap<String, usize>,
}

impl Context {
    pub fn assemble(source: String) -> Vec<u16> {
        let lines: Vec<&str> = source.split('\n').collect();
        let labels: HashMap<String, usize> = HashMap::new();
        let output: Vec<u16> = Vec::new();

        for line in lines {

        }
    }
}