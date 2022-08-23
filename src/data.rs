
#[derive(Debug, Clone)]
pub enum Constant {
    Bool(bool),
    String(String),
    Float(f64),
    Int(i64),
    Id(u64),
}

#[derive(Debug, Clone)]
pub enum IndepInstr {
    DefineConstant { name : String, value: Constant },
    DefineFunction { name : String, params : Vec<String>, instrs : Vec<Instr> },
}

#[derive(Debug, Clone)]
pub enum Instr { // All strings here are local variables
    Xor { result : String, left : String, right : String },
    Not { result : String, input : String },
    Or { result : String, left : String, right : String },
    And { result : String, left : String, right : String },
    GreaterThan { result : String, left : String, right : String },
    LessThan { result : String, left : String, right : String },
    Equal { result : String, left : String, right : String },
    Add { result : String, left : String, right : String },
    Sub { result : String, left : String, right : String },
    Mult { result : String, left : String, right : String },
    Div { result : String, left : String, right : String },
    Remainder { result : String, left : String, right : String },
    Call { name : String, params : Vec<String> },
    CallWithReturn { name : String, params : Vec<String>, result : String },
    SystemCall { name : String, params : Vec<String> },
    SystemCallWithReturn { name : String, params : Vec<String>, result : String },
    LoadAddress { result : String, address : String },
    Store { address : String, offset : String, input : String },
    Set { result : String, value : Constant },
    Label(String),
    Jump(String),
    Return(String),
    BranchOnFalse { label : String, input : String },
}

// allocate and free can be a sys call if the heap is passed into the sys call

#[derive(Debug, Clone)]
pub struct DataAddress(usize);

#[derive(Debug, Clone)]
pub enum Data {
    Bool(bool),
    String(String),
    Float(f64),
    Int(i64),
    Id(u64),
    DataAddress(DataAddress),
    Function(String),
}