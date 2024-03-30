struct Counter {
    count: u32,

    max: u32,
}

impl Counter {
    fn new(max: u32) -> Self {
        return Self { count: 0, max };
    }
}

impl Iterator for Counter {
    type Item = u32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            return Some(self.count);
        }

        return None;
    }
}

fn main() {
    let counter = Counter::new(100);
}
