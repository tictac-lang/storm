#![allow(dead_code)] 
#![allow(unused_imports)]
use serde::{Serialize, Deserialize};
use serde_json::json;
use storm::builtins::Builtins;

type Args = Vec<serde_json::Value>;

struct Instructions(Vec<Instruction>);

struct Instruction {
    inst_name: String,
    args: Args
}

impl Instruction {
    
}

fn main() {
    let j = r#"{"instructions": {"1print": "afeto ty ferterite", "2print": "peppis burp ferterite", "3print": "i have a bad sense of humor"}}"#;

    let value: serde_json::Value = serde_json::from_str(j).unwrap();
    println!("{:?}", value);
    if let serde_json::Value::Object(i) = value.get("instructions").unwrap() {
        println!("{:?}", &i);
        for x in i {
            let (a, b) = x;
            let a = Builtins.container_repl(a.to_string());
            let a = &a;
            let returned = Builtins::return_corrisponding_info(String::from(a));
            if matches!(b, serde_json::Value::String(_)) && returned.args_amt == 1 {
                if let serde_json::Value::String(string) = b {
                    Builtins.match_func(String::from(a), vec![String::from(string)]);
                }
            }
        }
    }
}