
#[derive(Debug, Clone)]
pub enum Constant {
    String(String),
    Float(f64),
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

    Call { name : String, params : Vec<String> },
    CallWithReturn { name : String, params : Vec<String>, target : String },
    SystemCall { name : String, params : Vec<String> },
    SystemCallWithReturn { name : String, params : Vec<String>, target : String },
    LoadAddress { target: String, source : String },
    // Store the target at whatever address is in address variable with some offset
    Store { address: String, offset : usize, target : String },
    Set { target : String, value : Constant },
    // TODO if
    // TODO loop
}

pub enum IfInstr {

}

pub enum LoopInstr {

}

// allocate and free can be a sys call if the heap is passed into the sys call

#[derive(Debug, Clone)]
pub struct DataAddress(usize);

#[derive(Debug, Clone)]
pub struct FunctionAddress(usize);

#[derive(Debug, Clone)]
pub enum Data {
    String(String),
    Float(f64),
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
