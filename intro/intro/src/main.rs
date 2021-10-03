use std::env;
use std::process;
use intro::spin;
use intro::mem;
use intro::mythreads;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        println!("usage: cpu <string>\n");
        process::exit(1);
    }

    let myString = &args[1];
    let mut my_int = myString.parse::<u32>().unwrap();

    mythreads(my_int);

    loop {
        spin(1);
        mem(&my_int);
        my_int += 1;
    }
        
}
