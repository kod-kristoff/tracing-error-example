use std::path::Path;
use std::{fmt, fs, io};

use tracing::instrument;
use tracing_error::prelude::*;
use tracing_error::TracedError;

#[instrument(fields(p = ?p))]
pub fn tac<P: AsRef<Path> + fmt::Debug>(p: P) -> Result<(), TracedError<io::Error>> {
    let s = fs::read_to_string(p.as_ref()).in_current_span()?;
    println!("{}", s);
    Ok(())
}
