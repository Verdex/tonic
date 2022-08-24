
#[derive(Debug)]
pub enum VmError {
    GlobalRedefinition(String),
    UndefinedGlobal(String),
    CannotCallNonFunction(String),
    VariableNotFound(String),
}

impl std::fmt::Display for VmError {
    fn fmt(&self, f : &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            VmError::GlobalRedefinition(name) => write!(f, "attempt to redefine global: {}", name),
            VmError::UndefinedGlobal(name) => write!(f, "attempt to access undefined global: {}", name),
            VmError::CannotCallNonFunction(name) => write!(f, "attempt to call non-function:  {}", name),
            VmError::VariableNotFound(name) => write!(f, "could not find variable:  {}", name),
        }
    }
}

impl std::error::Error for VmError {}