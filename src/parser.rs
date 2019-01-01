mod directive;
mod owner;
mod statement;

fn ws_or_comment(c: char) -> bool {
    c == ' ' || c == '#' || c == '\t'
}
