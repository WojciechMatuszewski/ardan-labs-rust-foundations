fn do_math(i: u32) -> u32 {
    let mut n = i + 1;
    for _ in 0..10 {
        n *= 2;
    }

    return n;
}

fn main() {
    println!("Hello from main thread");

    let mut thread_handles = Vec::new();
    for i in 0..10 {
        let thread_handle = std::thread::spawn(move || {
            return do_math(i);
        });
        thread_handles.push(thread_handle);
    }

    thread_handles.into_iter().for_each(|h| {
        let result = h.join().unwrap();
        println!("{result}");
    })
}
