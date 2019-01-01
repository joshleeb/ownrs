mod directive;
mod owner;
mod per_file;
mod statement;

fn ws_or_comment(c: char) -> bool {
    c == ' ' || c == '#' || c == '\t'
}
