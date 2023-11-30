use std::env;
use std::fs::File;
use turtle_graphics::{Canvas, Turtle};

mod lsystem;
extern crate jzon;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 2 {
        let program_name = &args[0];
        println!("usage: {program_name} l_system_json_file");
        return;
    }
    let filepath = &args[1];
    let system = lsystem::LSystem::from_file(filepath).expect("Encountered an error when loading L-System");

    match system.validate() {
        Err(errors) => {
            println!("[Error] Given L-System is invalid:");
            for error in errors.split(";") {
                if error.len() == 0 {
                    continue;
                }
                println!(">  {error}");
            }
            return;
        }
        _ => ()
    }

    // println!("0: {}", system.axiom[0]);
    let n = system.iters + 1;
    let mut line = None;
    for _ in 1..n {
        line = system.step(line);
        // let display = line.as_ref().expect("Missing value!").join("");
        // println!("{}: {}", n, display);
    }

    let mut t = Canvas::new();

    for symbol in line.unwrap() {
        if system.interp.contains_key(&symbol) {
            let instructions = &system.interp[&symbol];
            for instruction in instructions {
                match instruction {
                    lsystem::Instruction::Forward(amt) => t.forward(*amt),
                    lsystem::Instruction::Back(amt) => t.backward(*amt),
                    lsystem::Instruction::Left(amt) => t.left(*amt),
                    lsystem::Instruction::Right(amt) => t.right(*amt),
                    lsystem::Instruction::Push => t.push(),
                    lsystem::Instruction::Pop => t.pop(),
                    _ => ()
                }
            }
        }
    }

    t.save_svg(&mut File::create("test.svg").unwrap()).unwrap();
}
