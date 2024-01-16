fn main() {
    println!("{}", (-27i64).rem_euclid(13));
    println!("{}", 7u32.rem_euclid(3));
}

#[derive(Debug, PartialEq, Eq)]
struct FieldElement {
    num: u32,
    prime: u32,
}

impl FieldElement {
    pub fn new(num: u32, prime: u32) -> Self {
        Self { num, prime }
    }

    pub fn is_within_order(&self) -> Result<&Self, String> {
        // By using a Rust `u32` we always ensure the number cannot be less than 0
        // and we avoid checking `num >= prime || num < 0`
        if self.num >= self.prime {
            let error = String::new()
                + "Num `"
                + &self.num.to_string()
                + "` not in field range `0 to "
                + &self.prime.to_string()
                + "`";
            return Err(error);
        } else {
            Ok(self)
        }
    }
}
