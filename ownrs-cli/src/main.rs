#![feature(try_from)]

use args::Args;
use std::{convert::TryFrom, io};

mod app;
mod args;

fn main() -> io::Result<()> {
    let matches = app::app().get_matches();
    let args = Args::try_from(matches)?;

    println!("{:?}", args);
    Ok(())
}
