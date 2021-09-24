use std::convert::TryInto;
use std::sync::Mutex;

use crate::types::Int;

lazy_static::lazy_static!{
static ref CorrispondingInfo: Mutex<CrsInfo> = Mutex::new(CrsInfo(vec![FunctionInfo{name: String::from("print"), args_amt: 1}]));
}

pub struct CrsInfo(Vec<FunctionInfo>);

impl CrsInfo {
    pub fn search(&mut self, query: String) -> Int {
        let CrsInfo(vec) = self;
        let index = vec.iter().position(|r| r.name == query).unwrap();
        return index.try_into().unwrap();
    }
}

pub struct Builtins;

#[derive(Clone)]
pub struct FunctionInfo {
    pub name: String,
    pub args_amt: Int
}

impl Builtins {
    pub fn return_corrisponding_info(query: String) -> FunctionInfo {
        let index = (*CorrispondingInfo.lock().unwrap()).search(String::from(&query));
        let CrsInfo(raw) = &*CorrispondingInfo.lock().unwrap();
        FunctionInfo {
            name: query,
            args_amt: raw[index as usize].args_amt
        }
    }

    pub fn match_func(&mut self, func_name: String, variables: Vec<String>) {
        match func_name.as_str() {
            i if i.to_string().contains("print") => self.print(String::from(&variables[0])),
            _ => panic!("oh no functon no maetch")
        }
    }

    pub fn container_repl(&mut self, func_name: String) -> String {
        match func_name.as_str() {
            i if i.to_string().contains("print") => String::from("print"),
            _ => panic!("oh no functon no maetch")
        }
    }

    pub fn print(&mut self, string: String) {
        println!("{}", string);
    }
}