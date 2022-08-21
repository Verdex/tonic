
mod error;
mod data;

use std::collections::HashMap;
use std::error::Error;

use crate::error::VmError;
use crate::data::*;



pub fn run( entry : String
          , top_level : &Vec<IndepInstr>
          , heap : &mut HashMap<DataAddress, Data>
          , sys_calls : &Vec<fn(HashMap<DataAddress, Data>, Vec<Data>) -> Result<Data, Box<dyn Error>>>
          ) -> Result<(), Box<dyn Error>> {

    Ok(())
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
