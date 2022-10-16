fn test() -> bool {
   true
}

fn main() {
    test();
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert!(test());
    }
}
