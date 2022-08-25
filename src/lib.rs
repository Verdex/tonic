
mod error;
mod data;

use std::collections::HashMap;
use std::error::Error;

use crate::error::VmError;
use crate::data::*;

type Result<T> = std::result::Result<T, Box<dyn Error>>;

macro_rules! lg_get {
    ($local:ident, $global:ident, $target:ident) => {
        $local.get($target).or($global.get($target)).ok_or_else(|| Box::new(VmError::VariableNotFound($target.to_string())))?
    };
}

macro_rules! lg_remove {
    ($local:ident, $global:ident, $target:ident) => {
        $local.remove($target).or($global.get($target).map(|x| x.clone())).ok_or_else(|| Box::new(VmError::VariableNotFound($target.to_string())))?
    };
}

struct Frame<'a> {
    instr_ptr : usize,
    locals : HashMap<&'a str, Data<'a>>,
    current_function : &'a str,
    return_local : Option<&'a str>,
}

pub fn run<'a>( entry : &'a str 
          , independent_instructions : &'a Vec<IndepInstr<'a>>
          , heap : &mut HashMap<DataAddress, Data<'a>>
          , sys_calls : &Vec<fn(&mut HashMap<DataAddress, Data<'a>>, Vec<Data<'a>>) -> Result<Data<'a>>>
          ) -> Result<Option<Data<'a>>> {
    let mut globals : HashMap<&str, Data> = HashMap::new();
    let mut functions : HashMap<&str, _> = HashMap::new();

    for idep_instr in independent_instructions {
        match idep_instr {
            IndepInstr::DefineConstant { name, .. } if globals.contains_key(name) => {
                error(VmError::GlobalRedefinition(name.to_string()))?;
            },
            IndepInstr::DefineFunction { name, params, instrs } => {
                error(VmError::GlobalRedefinition(name.to_string()))?;
            },
            IndepInstr::DefineConstant { name, value } => {
                globals.insert( name, const_to_data(value) );
            },
            IndepInstr::DefineFunction { name, params, instrs } => {
                functions.insert( name, ( params, instrs ) );
            },
        }
    }

    let mut stack : Vec<Frame> = vec![]; 
    let mut final_return : Option<Data<'a>> = None;
    
    match functions.get(entry) {
        Some((_, mut instrs)) => { // TODO:  params for entry function make any sense?

            let mut instr_ptr = 0;
            let mut locals : HashMap<&str, Data> = HashMap::new();
            let mut current_function = entry;

            loop {

                if instrs.len() <= instr_ptr {
                    if stack.len() == 0 {
                        break;
                    }

                    Frame { instr_ptr, locals, current_function, .. } = stack.pop().unwrap(); 
                    let fun = functions.get(current_function).unwrap();
                    instrs = fun.1; 
                }

                match &instrs[instr_ptr] {
                    Instr::Xor { result, left, right } => {
                        let l = lg_get!(locals, globals, left);
                        let r = lg_get!(locals, globals, right);

                        let r = match (l, r) {
                            (Data::Bool(vl), Data::Bool(vr)) => vl ^ vr,
                            _ => panic!("TODO::Need error"),
                        };

                        locals.insert( result, Data::Bool(r) );
                        instr_ptr+=1;
                    },
                    Instr::Not { result, input } => {

                    },
                    Instr::Or { result, left, right } => {

                    },
                    Instr::And { result, left, right } => {

                    },
                    Instr::GreaterThan { result, left, right } => {

                    },
                    Instr::LessThan { result, left, right } => {

                    },
                    Instr::Equal { result, left, right } => {

                    },
                    Instr::Add { result, left, right } => {

                    },
                    Instr::Sub { result, left, right } => {

                    },
                    Instr::Mult { result, left, right } => {

                    },
                    Instr::Div { result, left, right } => {

                    },
                    Instr::Remainder { result, left, right } => {

                    },
                    Instr::Call { name, params } => {
                        let mut old_locals : HashMap<&str, Data> = HashMap::new();
                        std::mem::swap(&mut old_locals, &mut locals);

                        stack.push( Frame { instr_ptr, locals: old_locals, current_function, return_local: None } );
                        // TODO
                    },
                    Instr::CallWithReturn { name, params, result } => {

                    },
                    Instr::SystemCall { name, params } => {

                    },
                    Instr::SystemCallWithReturn { name, params, result } => {

                    },
                    Instr::LoadAddress { result, address } => {

                    },
                    Instr::Store { address, offset, input } => {

                    },
                    Instr::Set { result, value } => {

                    },
                    Instr::Label(name) => {

                    },
                    Instr::Jump(name) => {

                    },
                    Instr::Return(name) => {
                        let r = lg_remove!(locals, globals, name);

                        if stack.len() == 0 {
                            final_return = Some(r);
                            break;
                        }

                        let mut return_local = None;
                        Frame { instr_ptr, locals, current_function, return_local } = stack.pop().unwrap(); 
                        let fun = functions.get(current_function).unwrap();
                        instrs = fun.1; 

                        if let Some(local) = return_local {
                            locals.insert( local, r );
                        }
                    },
                    Instr::BranchOnFalse { label, input } => {

                    },
                }
            }
        },
        None => error(VmError::UndefinedGlobal(entry.to_string()))?,
    }

    Ok(final_return)

}

fn error(e : VmError) -> Result<()> {
    Err(Box::new(e))
} 

fn const_to_data(c : &Constant) -> Data {
    match c {
        Constant::Bool(v) => Data::Bool(*v),
        Constant::String(v) => Data::String(v.to_string()),
        Constant::Float(v) => Data::Float(*v),
        Constant::Int(v) => Data::Int(*v),
        Constant::Id(v) => Data::Id(*v),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
