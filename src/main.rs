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
        Ok(string) => print!("{}", string),
    };

    // the file is closed and `file` is taken out of scope
}

