mod cli;
use cli::parse_args;

fn main() {
    parse_args(std::env::args().collect())
}
