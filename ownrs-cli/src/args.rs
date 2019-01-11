use std::path::PathBuf;

#[derive(Debug, PartialEq)]
pub struct Args {
    pub root_dir: PathBuf,
    pub paths: Vec<PathBuf>,
}
