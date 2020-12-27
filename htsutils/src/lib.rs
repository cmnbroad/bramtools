//! Library level doc of bramtools lib.
//!

use std::{fs::File, io};

use noodles_bam::{self as bam, bai};
use noodles_sam as sam;
use noodles::Region;

/// Write the input file to the output file as a bam.
pub fn write_bam(in_file: &str, _out_file: &str) -> io::Result<()> {
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
        let in_file = String::from("../testdata/example.bam");
        let out_file = String::from("out.bam");
        write_bam(&in_file, &out_file).expect("test failed")
    }
}
