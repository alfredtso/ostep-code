use std::env;
use std::process;
use intro::spin;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("usage: cpu <string>\n");
        process::exit(1);
    }

    let myString = &args[1];

    loop {
        spin(1);
        println!("myString: {}", myString);
    }
        
}
