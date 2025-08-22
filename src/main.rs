use std::fs::File;
use turtle_graphics::{Canvas, Turtle};
use clap::Parser;

use crate::lsystem::LSystem;

mod lsystem;
extern crate jzon;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    filepath: String,
    #[arg(short = 'i', long = "iters")]
    iters: Option<u64>,
    #[arg(short = 'o', long = "out")]
    outpath: Option<String>,
    #[arg(short = 'a', long = "all")]
    all: bool,
}

fn create_img(system: &LSystem, line: &Vec<String>, outfile: &str) {
    let mut t = Canvas::new();

    for symbol in line {
        if system.interp.contains_key(symbol) {
            let instructions = &system.interp[symbol];
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

    t.save_svg(&mut File::create(outfile).unwrap()).unwrap();
}

fn main() {
    let args = Args::parse();
    let mut system = lsystem::LSystem::from_file(&args.filepath).expect("Encountered an error when loading L-System");
    match args.iters {
        Some(n) => system.iters = n,
        None => ()
    }

    match system.validate() {
        Err(errors) => {
            eprintln!("[Error] Given L-System is invalid:");
            for error in errors.split(";") {
                if error.len() == 0 {
                    continue;
                }
                eprintln!(">  {error}");
            }
            return;
        }
        _ => ()
    }

    let n = system.iters + 1;
    let mut line = None;
    let mut lines: Vec<Vec<String>> = Vec::new();
    for _ in 1..n {
        line = system.step(line);
        if let Some(ref l) = line {
            lines.push(l.clone());
        }
    }

    let outfile = match args.outpath {
        Some(p) => p,
        None => "test.svg".to_string(),
    };

    if args.all {
        for (i, line) in lines.iter().enumerate() {
            let outfile_i = if let Some(dot) = outfile.rfind('.') {
                format!("{}_{:03}.{}", &outfile[..dot], i, &outfile[dot+1..])
            } else {
                format!("{}_{}", outfile, i)
            };
            create_img(&system, &line, &outfile_i);
        }
    } else {
        create_img(&system, &lines[lines.len()-1], &outfile);
    }
}
