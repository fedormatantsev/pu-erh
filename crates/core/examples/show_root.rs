use std::env;

use pu_erh_core::Session;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = env::args().nth(1).expect("usage: show-root <kb.json>");
    let mut session = Session::open(&path)?;
    println!("{}", session.root_id());
    session.save()?;
    Ok(())
}
