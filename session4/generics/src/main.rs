use std::collections::HashMap;
use std::hash::Hash;

#[derive(Debug)]
struct HashMapBucket<K, V> {
    map: HashMap<K, Vec<V>>,
}

impl<K, V> HashMapBucket<K, V>
where
    K: Eq + Hash,
{
    fn new() -> Self {
        return HashMapBucket {
            map: HashMap::new(),
        };
    }

    fn insert(&mut self, key: K, value: V) {
        let values = self.map.entry(key).or_insert(Vec::new());
        values.push(value);
    }
}

struct HashMapBucketIter;

impl Iterator for HashMapBucketIter {
    type Item = ();

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

fn main() {
    let mut bucket = HashMapBucket::new();
    bucket.insert("hello", 1);
    bucket.insert("hello", 3);
    bucket.insert("goodbye", 3);

    println!("{bucket:?}");
}
