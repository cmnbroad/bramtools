
use noodles_sam as sam;
use noodles_bam::{self as bam};
//use noodles::Region;

use std::{
    ffi::CStr,
    fs::File,
    io::{self, BufReader}};

use std::io::Read;

// Issues:
// testdata/example.bam is rejected sue to the presence of lower case read group names

pub fn read_sam(in_file: &str, _out_file: &str)-> io::Result<()> {
    let mut reader = File::open(in_file)
        .map(BufReader::new)
        .map(sam::Reader::new)?;

    let _header: sam::Header = reader.read_header()?.parse().expect("Can't parse header");

    for result in reader.records() {
        let record = result?;
        println!("{:?}", record);
    }
    Ok(())
}

pub fn read_bam(in_file: &str, _out_file: &str) -> io::Result<()> {
    // let mut reader = File::open(in_file).map(bam::Reader::new)?;
    // let header: sam::Header = reader.read_header()?.parse().expect("Can't read header");
    //
    // let _reference_sequences = header.reference_sequences();
    // // let index = bai::read("sample.bam.bai")?;
    // // let region = Region::mapped("sq0", 17711, 28657);
    // // let query = reader.query(&reference_sequences, &index, &region)?;

    let mut reader = File::open(in_file).map(bam::Reader::new)?;
    reader.read_header()?;
    reader.read_reference_sequences()?;

    for result in reader.records() {
        let record = result?;
        let read_name = CStr::from_bytes_with_nul(record.read_name());
        println!("{:?}", record);
    }
    // for result in reader.records() {
    // let record = result.unwrap();
    //     // let _seq_id = record.reference_sequence_id().unwrap();
    //     // let _position = record.position().expect("can't read record position");
    //     // let _read_name = record.read_name();
    //     // //println!("{:?}", record.read_name());
    // }

    Ok(())
}

#[cfg(test)]
mod test {

    #[test]
    fn test_read_sam() {
        use crate::reads::bam;

        let in_file = String::from("testdata/print_reads.sorted.sam");
        let out_file = String::from("testdata/out.bam");
        bam::read_sam(&in_file,&out_file).expect("test failed")
    }

    #[test]
    fn test_read_bam() {
        use crate::reads::bam;

        let in_file = String::from("testdata/print_reads.sorted.bam");
        let out_file = String::from("testdata/out.bam");
        bam::read_bam(&in_file, &out_file).expect("test failed")
    }

}
