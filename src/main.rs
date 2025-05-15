fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    // Lataa kirjastoja yksikkötestien ulkopuolelta.
    //use super::*;

    #[test]
    fn test_lisaa() {
        assert_eq!(1 + 2, 3);
    }

    #[test]
    fn test_random_true() {
        let x = 4;
        let y = 2 + 2;
        assert_eq!(x, y);
    }
}