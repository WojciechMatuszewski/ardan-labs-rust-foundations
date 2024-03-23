use std::sync::atomic::AtomicU32;
use std::sync::atomic::Ordering::Relaxed;

static COUNTER: AtomicU32 = AtomicU32::new(0);


fn main() {
    let mut handles = Vec::new();

    for _ in 0..1000 {
        let handle = std::thread::spawn(|| {
            for _ in 0..1_100 {
                COUNTER.fetch_add(1, Relaxed);
            }
        });

        handles.push(handle);
    }

    handles.into_iter().for_each(|h| {
        h.join().unwrap()
    });

    println!("{}", COUNTER.load(Relaxed));
}

