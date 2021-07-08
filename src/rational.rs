use std::ops::{Add, Sub};
use std::fmt;
use std::error::Error;


// macros
// --------------------------------------------------------
#[macro_export]
macro_rules! rat {
    ($num:literal / $denom:literal) => {
        RationalNumber::new($num, $denom)
    }
}


// Error types
// --------------------------------------------------------
#[derive(Debug)]
pub struct RationalNumberError {
    details: String
}

impl fmt::Display for RationalNumberError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("RationalNumberError")
         .field("details", &self.details)
         .finish()
    }
}


impl Error for RationalNumberError {
    fn description(&self) -> &str {
        &self.details
    }
}


// Core types
// --------------------------------------------------------
#[derive(PartialEq)]
pub struct RationalNumber {
    num: i32,
    denom: i32
}


impl RationalNumber {
    pub fn new(num: i32, denom: i32) -> Result<RationalNumber, RationalNumberError> {
        if denom == 0 {
            let msg = String::from("`denom` cannot be zero");
            let err = RationalNumberError { details: msg };
            Err(err)
        } else if num == 0 {
            let res = RationalNumber { num: 0, denom: 1 };
            Ok(res)
        } else {
            let mut res = RationalNumber { num, denom };
            res.reduce();
            Ok(res)
        }
    }

    fn reduce(&mut self) {
        let g = gcd(self.num, self.denom);
        self.num = self.num / g;
        self.denom = self.denom / g;
    }
}


impl fmt::Debug for RationalNumber {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}/{}", self.num, self.denom)
    }
}


impl Add for RationalNumber {
    type Output = RationalNumber;

    fn add(self, other: RationalNumber) -> RationalNumber {
        let numerator = (self.num * other.denom) + (other.num * self.denom);
        let denominator = self.denom * other.denom;

        let mut output = RationalNumber {
            num: numerator,
            denom: denominator
        };
        output.reduce();
        output
    }
}


impl Sub for RationalNumber {
    type Output = RationalNumber;

    fn sub(self, other: RationalNumber) -> RationalNumber {
        let numerator = (self.num * other.denom) - (other.num * self.denom);
        let denominator = self.denom * other.denom;

        let mut output = RationalNumber {
            num: numerator,
            denom: denominator
        };
        output.reduce();
        output
    }
}


// GCD via simple Euclid's algorithm
// --------------------------------------------------------
fn gcd(num1: i32, num2: i32) -> i32 {
    let mut max = num1;
    let mut min = num2;

    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}
