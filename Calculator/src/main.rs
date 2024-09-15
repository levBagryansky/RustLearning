struct Calc {
    r0: Result<i32, String>,
}

impl Calc {
    fn add(mut self, rhs: i32) -> Self {
        self.r0 = match self.r0 {
            Err(err) => Err(err),
            Ok(val) => val.checked_add(rhs).ok_or("Overflow".to_string())
        };
        self
    }

    fn sub(self, rhs: i32) -> Self {
        self.add(-rhs)
    }

    fn mul(mut self, rhs: i32) -> Self {
        self.r0 = match self.r0 {
            Err(err) => Err(err),
            Ok(val) => val.checked_mul(rhs).ok_or("Overflow".to_string())
        };
        self
    }

    fn div(mut self, rhs: i32) -> Self {
        self.r0 = match self.r0 {
            Err(err) => Err(err),
            Ok(val) => val.checked_div(rhs).ok_or("The divisor is zero".to_string())
        };
        self
    }
}

impl Default for Calc {
    fn default() -> Self {
        Self {
            r0: Ok(0)
        }
    }
}

impl std::fmt::Display for Calc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Cals: {}",
            self.r0.clone()
                .map_or_else(|err| err, |val| val.to_string())
        )
    }
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod test {
    use crate::Calc;

    #[test]
    fn simple_addition() {
        let res = Calc::default().add(2).add(2);
        assert_eq!("Cals: 4", res.to_string());
    }

    #[test]
    fn overflow() {
        let res = Calc::default().add(i32::MAX).add(1);
        assert_eq!("Cals: Overflow", res.to_string());
    }

    #[test]
    fn division_by_zero() {
        let res = Calc::default().div(0);
        assert_eq!("Cals: The divisor is zero", res.to_string());
    }
}