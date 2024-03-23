use std::sync::Mutex;

static MY_SHARED: Mutex<i32> = Mutex::new(3);

fn posioner() {
    let mut lock = MY_SHARED.lock().unwrap();
    *lock += 1;
    panic!("Strike")
}

fn main() {
    let handle = std::thread::spawn(posioner);
    println!("Trying to return from the thread");
    println!("{:?}", handle.join());

    let lock = MY_SHARED.lock();
    println!("{lock:?}");
}
