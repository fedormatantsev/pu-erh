use std::env;

use pu_erh_core::Session;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args().nth(1).expect("usage: show-root <kb-storage-dir>");
    let mut session = Session::open(&path)?;
    session.save()?;
    println!("{}", session.root_id()?);
    Ok(())
}
