fn main() {
    println!("Hello, world!");
}

async fn double(n: u32) -> u32 {
    return n * 2;
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn simple_test() {
        assert_eq!(2, 2);
    }

    #[tokio::test]
    async fn will_not_compile() {
        assert_eq!(double(2).await, 4);
    }
}