use std::{env, io};
use htscommands::CommandBundle;

fn main() -> io::Result<()> {
    let command = CommandBundle::new(env::args())
        .expect("Problem parsing arguments.");
    let _res = command.run();
    Ok(())
}
