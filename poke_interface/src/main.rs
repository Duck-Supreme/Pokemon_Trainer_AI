mod ai_interface;

use std::io::{Read, pipe};
use std::process::Command;
use std::error::Error;

fn main() -> Result<(),Box<dyn Error>> {
    let (reader, _) = pipe()?;
    let (mut ai_output, writer) = pipe()?;
    let _trainer_ai_command = Command::new("../trainer_ai/release/poke_ai.exe").stdin(reader).stdout(writer).spawn()?;


    let mut buf = String::new();
    ai_output.read_to_string(&mut buf)?;
    println!("{}",buf);
    Ok(())

}
