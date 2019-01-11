use clap::{crate_authors, crate_version, App, Arg};

pub fn app() -> App<'static, 'static> {
    let mut app = App::new("ownrs")
        .author(crate_authors!())
        .version(crate_version!());

    for arg in args_and_flags() {
        app = app.arg(arg);
    }
    app
}

fn args_and_flags<'a>() -> Vec<Arg<'static, 'static>> {
    vec![arg_root(), arg_paths()]
}

fn arg_root() -> Arg<'static, 'static> {
    Arg::with_name("root_dir")
        .short("d")
        .long("dir")
        .value_name("DIR")
        .global(true)
}

fn arg_paths() -> Arg<'static, 'static> {
    Arg::with_name("paths")
        .value_name("PATHS")
        .multiple(true)
        .required(true)
}
