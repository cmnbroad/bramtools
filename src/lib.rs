mod tools;

#[cfg(test)]

#[test]
fn test_write_bam() {
    //assert_eq!(2 + 2, 4);
    let in_file = String::from("testdata/example.bam");
    let out_file = String::from("testdata/out.bam");
    tools::write_bam(&in_file, &out_file).expect("test failed")
}