
pub trait Read<T> {
    fn get_name(&self) -> String;
    //fn get_pos(&self) -> i64; -> aligned
    //fn get_quals(&self) -> [u8];
}

#[cfg(test)]
mod test {

    #[test]
    fn test_seq() {
        // let in_file = String::from("testdata/print_reads.sorted.bam");
        // let out_file = String::from("testdata/out.bam");
        // bam::read_bam(&in_file, &out_file).expect("test failed")
    }

}
