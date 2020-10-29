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

