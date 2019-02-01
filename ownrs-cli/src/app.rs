use clap::{crate_authors, crate_version, App, Arg};

pub fn app() -> App<'static, 'static> {
    App::new("ownrs")
        .author(crate_authors!())
        .version(crate_version!())
        .arg(
            Arg::with_name("root_dir")
                .short("d")
                .long("dir")
                .value_name("DIR")
                .global(true),
        )
        .arg(
            Arg::with_name("paths")
                .value_name("PATHS")
                .multiple(true)
                .required(true),
        )
}
