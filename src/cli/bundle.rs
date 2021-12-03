use std::error::Error;
use std::str::FromStr;

//TODO: should eliminate the need for this awkward use
use crate::reads::bam;

use crate::cli::command::{Command};

#[derive(Debug)]
pub struct Bundle {
    pub command: Command,
    pub input_file: String,
    pub output_file: String
}

impl Bundle {
    pub fn new(args: &[String]) -> Result<Bundle, &'static str> {
        if args.len() < 4 {
            return Err("Not enough arguments\nUsage: program_name command input_file output_file");
        }
        let command = args[1].clone();
        let output_file = args[2].clone();
        let input_file = args[3].clone();

        let command = Command::from_str(&command).unwrap();
        Ok(Bundle { command, input_file, output_file })
    }

    // return unit, or a dynamic error
    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        println!("Command:{} Input:{} Output:{}\n", self.command, self.input_file, self.output_file);
        bam::write_bam(&self.input_file, &self.output_file)?;
        Result::Ok(())
    }
}
