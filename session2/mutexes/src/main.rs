use std::sync::Mutex;
use std::thread;

static NUMBERS: Mutex<Vec<i32>> = Mutex::new(Vec::new());

fn main() {
    thread::scope(|s| {
        for _ in 0..10 {
            s.spawn(|| {
                let mut lock = NUMBERS.lock().unwrap();
                lock.push(1);
            });
        }
    });

    let lock = NUMBERS.lock().unwrap();
    println!("{:#?}", lock)
}
