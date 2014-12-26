#![feature(phase)]
#[phase(plugin)]
extern crate regex_macros;
extern crate regex;

use std::os; // for os::args
use std::io::File;

fn main() {
    let args = os::args();
    if args.len() <= 1 {
        panic!("you must specify a file to process");
    }

    let filename = &args[1].as_slice();
    let path = Path::new(filename);
    let display = path.display();

    let mut file = match File::open(&path) {
        Err(error) => panic!("failed to open {}: {}", display, error.desc),
        Ok(file) => file,
    };

    match file.read_to_string() {
        Err(error) => panic!("failed to read {}: {}", display, error.desc),
        Ok(string) => process(string),
    };

    // the file is closed and `file` is taken out of scope
}

fn process(string: String) {

    // an assignment can be defined as:
    // 'let' ANY_VALID_IDENTIFIER '=' ANY_MATH_EXPRESSION
    //  - where a valid identifier only includes a-z and A-Z
    //  - and where (currently, will be fixed), a valid math
    //    expression contains two operands and 1 operator (unary operators not included)
    let assignment = regex!( r"^\s*let\s+[a-zA-Z]+\s+=\s+\d+(?:\s+[\+\*-/]\s+\d+)*$" );

    for line in string.as_slice().split('\n') {
        if assignment.is_match(line) {
            println!("got assignment '{}'", line);
        }
    }
}

