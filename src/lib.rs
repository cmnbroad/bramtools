use std::error::Error;

use noodles_bam::{self as bam, bai};
use noodles_sam as sam;
use noodles::Region;

use std::{fs::File, io};

pub fn write_bam(in_file: &String, _out_file: &String) -> io::Result<()> {
    let mut reader = File::open(in_file).map(bam::Reader::new)?;
    let header: sam::Header = reader.read_header()?.parse().expect("Can't read header");

    let reference_sequences = header.reference_sequences();
    let index = bai::read("sample.bam.bai")?;
    let region = Region::mapped("sq0", 17711, 28657);
    let query = reader.query(&reference_sequences, &index, &region)?;

    for result in query {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

#[cfg(test)]
mod test {

    #[test]
    fn test_write_bam() {
        use crate::write_bam;

        let in_file = String::from("testdata/example.bam");
        let out_file = String::from("testdata/out.bam");
        write_bam(&in_file, &out_file).expect("test failed")
    }

}

pub struct Bundle {
    pub command: String,
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
        Ok(Bundle { command, input_file, output_file })
    }

    pub fn run(&self) -> Result<(), Box<dyn Error>> {
        println!("Command:{} Input:{} Output:{}\n", self.command, self.input_file, self.output_file);
        write_bam(&self.input_file, &self.output_file)?;
        Result::Ok(())
    }
}

