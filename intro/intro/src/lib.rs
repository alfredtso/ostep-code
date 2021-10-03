use std::sync::Arc;
use std::sync::Mutex;
use std::thread::sleep;
use std::time::Duration;
use std::process;
use std::thread;

/// Use sleep
pub fn spin(t: u64) {
    sleep(Duration::new(t, 0));
}

pub fn mem(t: &u32) {
    let pid = process::id();
    println!("{} value of p: {}", pid, &t);
}

pub fn mythreads(l: u32) {
    let count = Arc::new(Mutex::new(0));
    let mut threads = vec![];
    println!("Initial value: {:}", *count.lock().unwrap());

    for _ in 0..2 {
        let count = Arc::clone(&count);
        let pthread = thread::spawn(move || {
            let mut num = count.lock().unwrap();
            for _ in 0..l {
                *num += 1;
            }
        });
        threads.push(pthread);
    }

    for thread in threads {
        thread.join().unwrap();
    }
        
    println!("Final value: {}", *count.lock().unwrap());
}

