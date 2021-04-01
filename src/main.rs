extern crate ltime;

use std::io;

fn main() -> Result<(), ltime::Error> {
    ltime::filter(&mut io::stdin().lock(), &mut io::stdout(), &chrono::offset::Local)?;
    Ok(())
}
