use std::fmt;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BigInt {
    digits: Vec<u8>, // Least significant digit first
}

impl BigInt {
    /// Creates a new BigInt from a string representation.
    pub fn new(value: &str) -> BigInt {
        let digits = value
            .chars()
            .rev()
            .map(|c| c.to_digit(10).unwrap() as u8)
            .collect();
        BigInt { digits }
    }

    /// Returns a string representation of the BigInt.
    pub fn to_string(&self) -> String {
        self.digits.iter().rev().map(|d| d.to_string()).collect()
    }

    /// Adds another BigInt to this BigInt and returns the result.
    pub fn add(&self, other: &BigInt) -> BigInt {
        let mut result = Vec::new();
        let mut carry = 0;

        let max_len = self.digits.len().max(other.digits.len());
        for i in 0..max_len {
            let sum = self.digits.get(i).unwrap_or(&0) + other.digits.get(i).unwrap_or(&0) + carry;
            result.push(sum % 10);
            carry = sum / 10;
        }

        if carry > 0 {
            result.push(carry);
        }

        BigInt { digits: result }
    }
}

// Display trait implementation to use BigInt in print statements.
impl fmt::Display for BigInt {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.to_string())
    }
}
