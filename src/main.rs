use std::{env, io, eprintln};
use bramtools::CommandBundle;

fn main() -> io::Result<()> {
    let bundle = CommandBundle::new(env::args()).unwrap();
    let config = Config::new(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {}", err);
    });
    let _res = bundle.run();
    Ok(())
}

