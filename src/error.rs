
#[derive(Debug)]
pub enum VmError {
    GlobalRedefinition(String),
    UndefinedGlobal(String),
    CannotCallNonFunction(String),
    EmptyFunction(String),
}

impl std::fmt::Display for VmError {
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            VmError::GlobalRedefinition(name) => write!(f, "attempt to redefine global: {}", name),
            VmError::UndefinedGlobal(name) => write!(f, "attempt to access undefined global: {}", name),
            VmError::CannotCallNonFunction(name) => write!(f, "attempt to call non-function:  {}", name),
            VmError::EmptyFunction(name) => write!(f, "unexpected empty function:  {}", name),
        }
    }
}

impl std::error::Error for VmError {}