use htsutils::write_bam;
use std::error::Error;

pub struct CommandBundle {
    pub command: String,
    pub input_file: String,
    pub output_file: String
}

impl CommandBundle {
    pub fn new(mut args: std::env::Args) -> Result<CommandBundle, &'static str> {
        args.next(); // skip program name
        let command = match args.next() {
            Some(arg) => arg,
            None => return Err("A command must be provided"),
        };
        let input_file = match args.next() {
            Some(arg) => arg,
            None => return Err("An input filename must be provided"),
        };
        let output_file = match args.next() {
            Some(arg) => arg,
            None => return Err("An output filename must be provided"),
        };
        Ok(CommandBundle { command, input_file, output_file })
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        println!("Command:{} Input:{} Output:{}\n", self.command, self.input_file, self.output_file);
        // match command {
        //      "write" => Error();
        // }
        write_bam(&self.input_file, &self.output_file)?;
        Result::Ok(())
    }
}
