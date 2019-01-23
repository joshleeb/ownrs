use clap::ArgMatches;
use std::{convert::TryFrom, env, io, path::PathBuf};

#[derive(Debug, PartialEq)]
pub struct Args {
    pub root_dir: PathBuf,
    pub paths: Vec<PathBuf>,
}

impl TryFrom<ArgMatches<'_>> for Args {
    type Error = io::Error;

    fn try_from(matches: ArgMatches) -> Result<Self, Self::Error> {
        let args = Self {
            paths: get_paths(matches.values_of("paths").unwrap())?,
            root_dir: get_root_dir(matches.value_of("root_dir"))?,
        };

        // Check that all paths are children of the root directory.
        for path in &args.paths {
            let mut dir_path = path.clone();

            // Remove the filename if it's a file.
            if dir_path.is_file() {
                dir_path = dir_path.parent().unwrap().into();
            }

            if !dir_path.starts_with(&args.root_dir) {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    format!(
                        "root path {} doesn't contain path {}",
                        args.root_dir.display(),
                        path.display()
                    ),
                ));
            }
        }

        Ok(args)
    }
}

fn get_paths<'a, I: Iterator<Item = &'a str>>(arg: I) -> io::Result<Vec<PathBuf>> {
    let paths: Vec<PathBuf> = arg.map(PathBuf::from).collect();

    if paths.is_empty() {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "no paths provided",
        ));
    }

    // Check that all paths provided exist.
    if let Some(unknown_path) = paths.iter().find(|p| !p.exists()) {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("unknown path {}", unknown_path.display()),
        ));
    }
    Ok(paths)
}

fn get_root_dir(arg: Option<&str>) -> io::Result<PathBuf> {
    let default_dir = env::current_dir()?;
    let root_dir = arg.map(PathBuf::from).unwrap_or(default_dir);

    if !root_dir.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("unknown root directory {}", root_dir.display()),
        ));
    }
    if !root_dir.is_dir() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("root directory is not a directory {}", root_dir.display()),
        ));
    }

    Ok(root_dir)
}

#[cfg(test)]
mod tests {
    use super::*;
    // use clap::Values;
    use std::fs::{canonicalize, File};

    #[test]
    fn get_paths_exists() {
        let dir = env::temp_dir();
        let arg = vec![dir.to_str().unwrap()];

        assert_eq!(vec![env::temp_dir()], get_paths(arg.into_iter()).unwrap());
    }

    #[test]
    fn get_paths_empty() {
        assert!(get_paths(vec![].into_iter()).is_err())
    }

    #[test]
    fn get_paths_not_found() {
        let file_path = env::temp_dir().join("not-found");
        let arg = vec![file_path.to_str().unwrap()];

        assert!(get_paths(arg.into_iter()).is_err())
    }

    #[test]
    fn get_root_dir_exists() {
        let dir = env::temp_dir();

        assert_eq!(dir, get_root_dir(dir.to_str()).unwrap());
    }

    #[test]
    fn get_root_dir_default() {
        let dir = env::temp_dir();
        env::set_current_dir(&dir).unwrap();

        assert_eq!(canonicalize(dir).unwrap(), get_root_dir(None).unwrap());
    }

    #[test]
    fn get_root_dir_not_found() {
        let file_path = env::temp_dir().join("not-found");

        assert!(get_root_dir(file_path.to_str()).is_err());
    }

    #[test]
    fn get_root_dir_file() {
        let file_path = env::temp_dir().join("some-file.txt");
        File::create(&file_path).unwrap();

        assert!(get_root_dir(file_path.to_str()).is_err());
    }
}
