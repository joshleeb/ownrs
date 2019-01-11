mod app;
mod args;

fn main() {
    let arg_matches = app::app().get_matches();
    println!("{:?}", arg_matches);
}
