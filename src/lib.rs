
#[derive(Debug, Clone)]
pub enum Constant {
    Bool(bool),
    String(String),
    Float(f64),
    Usize(usize),
    Int(i64),
    Id(u64),
    List(Vec<Constant>),
}

#[derive(Debug, Clone)]
pub enum IndepInstr {
    DefineConstant { name : String, value: Constant },
    DefineFunction { name : String, params : Vec<String>, instrs : Vec<Instr> },
}

#[derive(Debug, Clone)]
pub enum Instr { // All strings here are local variables

    // BoolEqual, BoolAnd, BoolOr, BoolXor
    // StringEqual
    // FloatEqual (float compare?)
    // IdEqual
    // ListLen 
    // IntEqual, IntLessThan, IntGreaterThan
    // UsizeEqual, USizeLessThan, USizeGreaterThan
    // FloatAdd, mult, div, sub
    // IntAdd, mult, div, sub 

    Call { name : String, params : Vec<String> },
    CallWithReturn { name : String, params : Vec<String>, target : String },
    SystemCall { name : String, params : Vec<String> },
    SystemCallWithReturn { name : String, params : Vec<String>, target : String },
    LoadAddress { target: String, source : String },
    // Store the target at whatever address is in address variable with some offset
    Store { address: String, offset : usize, target : String },
    Set { target : String, value : Constant },
    Loop(Vec<Insr>),
    If { test : String, true_case : Vec<Instr>, false_case : Vec<Instr> },
    Break,
}

// allocate and free can be a sys call if the heap is passed into the sys call

#[derive(Debug, Clone)]
pub struct DataAddress(usize);

#[derive(Debug, Clone)]
pub struct FunctionAddress(usize);

#[derive(Debug, Clone)]
pub enum Data {
    Bool(bool),
    String(String),
    Float(f64),
    Usize(usize),
    Int(i64),
    Id(u64),
    DataAddress(DataAddress),
    List(Vec<Data>),
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
