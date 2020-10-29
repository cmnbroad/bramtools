use std::{env, io};

mod tools;

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 3 {
        let in_file = &args[1];
        let out_file = &args[2];
        println!("Infile: {} Outfile: {}", in_file, out_file);
        tools::write_bam(in_file, out_file)
    } else {
        println!("Input and output files must be provided");
        Ok(())
    }
}

