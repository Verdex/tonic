
mod error;
mod data;

use std::collections::HashMap;
use std::error::Error;

use crate::error::VmError;
use crate::data::*;

#[derive(Debug)]
enum Global<'a> { 
    Const(&'a Constant),
    Func { params : &'a Vec<String>, instrs : &'a Vec<Instr> },
}

type Result<T> = std::result::Result<T, Box<dyn Error>>;

pub fn run( entry : String
          , independent_instructions : &Vec<IndepInstr>
          , heap : &mut HashMap<DataAddress, Data>
          , sys_calls : &Vec<fn(HashMap<DataAddress, Data>, Vec<Data>) -> Result<Data>>
          ) -> Result<Data> {
    let mut globals = HashMap::new();
    for idep_instr in independent_instructions {
        match idep_instr {
            IndepInstr::DefineConstant { name, .. } if globals.contains_key(&name) => {
                return Err(Box::new(VmError::GlobalRedefinition(name.clone())));
            },
            IndepInstr::DefineFunction { name, params, instrs } => {
                return Err(Box::new(VmError::GlobalRedefinition(name.clone())));
            },
            IndepInstr::DefineConstant { name, value } => {
                globals.insert( name, Global::Const(value) );
            },
            IndepInstr::DefineFunction { name, params, instrs } => {
                globals.insert( name, Global::Func { params, instrs } );
            },
        }
    }

    let mut stack = vec![]; // hashmap<string, data> + func name? + return index // local context
    let mut intr_ptr = 0;
    let mut current_func = entry;
    let mut locals = HashMap::new(); 

    if !globals.contains_key(&entry) {
        error(VmError::UndefinedGlobal(entry))?;
    }
    
    if let Global::Func { params, instrs } = *globals.get(&entry).unwrap() {

        if instrs.len() < 1 {
            error(VmError::EmptyFunction(entry))?;
        }


    }
    else {
        error(VmError::CannotCallNonFunction(entry))?;
    }
    Ok(Data::Id(0))

}

fn error(e : VmError) -> Result<()> {
    Err(Box::new(e))
} 

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
