use lang::{eval, parse};

fn main() {
    let program = "and(true, or(false, and(true, or(false, false)))); or(false, true)";
    match parse::pesty_fmt(program) {
        Ok(a) => println!("{}", a),
        Err(e) => eprintln!("Error: {:?}", e),
    };

    match parse::parse_program(program) {
        Ok(a) => {
            println!("{:#?}", a);
            println!("{:#?}", eval::eval_program(a));
        }
        Err(e) => eprintln!("Error: {:?}", e),
    }
}
