fn main() {
    const N_THREADS: usize = 8;

    let to_add: Vec<u32> = (0..5000).collect();
    let mut thread_handles = Vec::new();

    let chunks = to_add.chunks(N_THREADS);

    for chunk in chunks {
        let my_chunk = chunk.to_owned();

        let handle = std::thread::spawn(move || {
            return my_chunk.iter().sum::<u32>();
        });
        thread_handles.push(handle);
    }

    let mut sum = 0;
    for handle in thread_handles {
        sum += handle.join().unwrap();
    }

    println!("{sum}");
}
