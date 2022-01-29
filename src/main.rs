use std::{env, io};

use bramtools::cli::bundle::Bundle;

fn main() -> io::Result<()> {
    let args = env::args().collect();
    let bundle = Bundle::new(&args).unwrap();
    let _res = bundle.run();
    Ok(())
}

