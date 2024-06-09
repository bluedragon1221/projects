mod cli;
use cli::parse_args;

fn main() {
    // arg0 is the program (avoid)
    // arg1 is the filename
    // arg2.. are the tags

    parse_args(std::env::args().collect())
}
