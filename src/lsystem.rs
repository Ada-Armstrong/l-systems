
use std::collections::{HashMap, HashSet};
use std::io::prelude::*;
use std::fs::File;

use jzon::JsonValue;

#[derive(Debug)]
pub enum Instruction {
    Forward(f32),
    Back(f32),
    Left(f32),
    Right(f32),
    Push,
    Pop,
    Nop
}

impl Instruction {
    fn from_json(val: &jzon::object::Object) -> Self {
        if val.len() != 1 {
            panic!("Too many key value pairs");
        }
        for (k, v) in val.iter() {
            let n: f32 = match v {
                JsonValue::Number(val) => val.clone().into(),
                JsonValue::Null => 0.0,
                _ => { panic!("test"); }
            };
            let instruction = match k {
                "forward" => Instruction::Forward(n),
                "backward" => Instruction::Back(n),
                "left" => Instruction::Left(n),
                "right" => Instruction::Right(n),
                "push" => Instruction::Push,
                "pop" => Instruction::Pop,
                _ => { panic!("HELP ME"); }
            };

            return instruction;
        }

        Instruction::Nop
    }
}

#[derive(Debug)]
pub struct LSystem {
    pub iters: u64,
    pub alphabet: HashSet<String>,
    pub axiom: Vec<String>,
    pub rules: HashMap<String, Vec<String>>,
    pub interp: HashMap<String, Vec<Instruction>>
}

impl LSystem {
    #[allow(dead_code)]
    fn default() -> Self {
        Self {iters: 0, alphabet: HashSet::new(), axiom: Vec::new(), rules: HashMap::new(), interp: HashMap::new()}
    }

    pub fn step(&self, axiom: Option<Vec<String>>) -> Option<Vec<String>> {
        let mut out: Vec<String> = Vec::new();
        let start = match &axiom {
            None => &self.axiom,
            Some(val) => val
        };

        for symbol in start {
            if self.rules.contains_key(symbol) {
                let expansion = &self.rules[symbol];
                out.append(&mut expansion.clone());
            } else {
                out.push(symbol.clone());
            }
        }

        Some(out)
    }

    pub fn from_file(filepath: &str) -> Result<Self, String> {
        let mut file = File::open(filepath).expect("Unable to open the file");
        let mut contents = String::new();
        file.read_to_string(&mut contents).expect("Unable to read the file");

        let parsed = jzon::parse(&contents).unwrap();

        let json_alphabet = match &parsed["alphabet"] {
            JsonValue::Array(val) => val,
            _ => { return Err("alphabet value must be an array".to_string()) }
        };

        let json_axiom = match &parsed["axiom"] {
            JsonValue::Array(val) => val,
            _ => { return Err("axiom value must be an array".to_string()) }
        };

        let json_rules = match &parsed["rules"] {
            JsonValue::Object(val) => val,
            _ => { return Err("rules value must be an object".to_string()) }
        };

        let iters: u64 = match &parsed["iters"] {
            JsonValue::Number(val) => val.clone().as_fixed_point_u64(0).expect(""),
            _ => { return Err("iters value must be a number".to_string()); }
        };

        let mut alphabet: HashSet<String> = HashSet::new();
        
        for val in json_alphabet {
            match val {
                JsonValue::String(s) => { alphabet.insert(s.clone()); }
                JsonValue::Short(s) => { alphabet.insert(s.to_string()); }
                _ => return Err("alphabet array element must be a string".to_string())
            }
        }

        let mut axiom: Vec<String> = Vec::new();

        for val in json_axiom {
            match val {
                JsonValue::String(s) => axiom.push(s.clone()),
                JsonValue::Short(s) => axiom.push(s.to_string()),
                _ => return Err("axiom array element must be a string".to_string())
            }
        }

        let mut rules: HashMap<String, Vec<String>> = HashMap::new();

        for (input, output) in json_rules.iter() {
            match output {
                JsonValue::Array(arr) => {
                    let mut output_vec: Vec<String> = Vec::new();
                    for v in arr {
                        match v {
                            JsonValue::String(s) => output_vec.push(s.clone()),
                            JsonValue::Short(s) => output_vec.push(s.to_string()),
                            _ => return Err("production rule output element must be a string".to_string())
                        }
                    }
                    rules.insert(input.to_string(), output_vec);
                }
                _ => return Err("production rule output must be an array".to_string())
            }
        }

        let mut interp: HashMap<String, Vec<Instruction>> = HashMap::new();

        let json_interp = match &parsed["interpretation"] {
            JsonValue::Object(val) => val,
            _ => { return Err("interpretation value must be an object".to_string()) }
        };

        for (input, output) in json_interp.iter() {
            match output {
                JsonValue::Array(arr) => {
                    let mut output_vec: Vec<Instruction> = Vec::new();
                    for v in arr {
                        match v {
                            JsonValue::Object(val) => output_vec.push(Instruction::from_json(val)),
                            _ => return Err("interpretation rule output element must be an object".to_string())
                        }
                    }
                    interp.insert(input.to_string(), output_vec);
                }
                _ => return Err("interpretation rule output must be an array".to_string())
            }
        }

        Ok(Self {iters, alphabet, axiom, rules, interp})
    }

    pub fn validate(&self) -> Result<(), String> {
        let mut errors = String::new();
        for symbol in &self.axiom {
            if !self.alphabet.contains(symbol) {
                errors.push_str(&format!("Unknown symbol {symbol} in axiom;"));
            }
        }
        for (k, v) in &self.rules {
            if !self.alphabet.contains(k) {
                errors.push_str(&format!("Unknown symbol {k} as input in production rule;"));
            }
            for s in v {
                if !self.alphabet.contains(s) {
                    errors.push_str(&format!("Unknown symbol {s} as output in production rule;"));
                }
            }
        }
        for (k, _v) in &self.interp {
            if !self.alphabet.contains(k) {
                errors.push_str(&format!("Unknown symbol {k} as interpretation;"));
            }
        }
        if errors.len() == 0 {
            return Ok(());
        } else {
            return Err(errors);
        }
    }
}