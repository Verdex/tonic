
#[derive(Debug, Clone)]
pub enum Constant {
    Bool(bool),
    String(String),
    Float(f64),
    Int(i64),
    Id(u64),
}

#[derive(Debug, Clone)]
pub enum IndepInstr<'a> {
    DefineConstant { name : &'a str, value: Constant },
    DefineFunction { name : &'a str, params : Vec<&'a str>, instrs : Vec<Instr<'a>> },
}

#[derive(Debug, Clone)]
pub enum Instr<'a> { // All strings here are local variables
    Xor { result : &'a str, left : &'a str, right : &'a str},
    Not { result : &'a str, input : &'a str },
    Or { result : &'a str, left : &'a str, right : &'a str},
    And { result : &'a str, left : &'a str, right : &'a str},
    GreaterThan { result : &'a str, left : &'a str, right : &'a str },
    LessThan { result : &'a str, left : &'a str, right : &'a str },
    Equal { result : &'a str, left : &'a str, right : &'a str },
    Add { result : &'a str, left : &'a str, right : &'a str },
    Sub { result : &'a str, left : &'a str, right : &'a str },
    Mult { result : &'a str, left : &'a str, right : &'a str },
    Div { result : &'a str, left : &'a str, right : &'a str },
    Remainder { result : &'a str, left : &'a str, right : &'a str },
    Call { name : &'a str, params : Vec<&'a str> },
    CallWithReturn { name : &'a str, params : Vec<&'a str>, result : &'a str },
    SystemCall { name : &'a str, params : Vec<&'a str> },
    SystemCallWithReturn { name : &'a str, params : Vec<&'a str>, result : &'a str },
    LoadAddress { result : &'a str, address : &'a str },
    Store { address : &'a str, offset : &'a str, input : &'a str },
    Set { result : &'a str, value : Constant },
    Label(&'a str),
    Jump(&'a str),
    Return(&'a str),
    BranchOnFalse { label : &'a str, input : &'a str },
}

// allocate and free can be a sys call if the heap is passed into the sys call

#[derive(Debug, Clone)]
pub struct DataAddress(usize);

#[derive(Debug, Clone)]
pub enum Data<'a> {
    Bool(bool),
    String(String),
    Float(f64),
    Int(i64),
    Id(u64),
    DataAddress(DataAddress),
    Function(&'a str),
}