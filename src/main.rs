use std::os;

fn main() {
    let args = os::args();
    if args.len() > 1 {
        println!( "the first argument is {}", args[1] );
    } else {
        println!("you didn't give any arguments");
    }
}

