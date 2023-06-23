// FIXME: can't derive Eq, although we can compare numbers for equality - ?
#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy, PartialOrd, Ord)]
pub struct Satoshi(pub u64);

impl Satoshi {
    pub fn eq(&self, other_amount: &Satoshi) -> bool {
        self.0 == other_amount.0
    }
    pub fn gt(&self, other_amount: &Satoshi) -> bool {
        self.0 > other_amount.0
    }
    pub fn lt(&self, other_amount: &Satoshi) -> bool {
        other_amount.gt(&self)
    }
    pub fn ge(&self, other_amount: &Satoshi) -> bool {
        self.gt(other_amount) || self.eq(other_amount)
    }
    pub fn le(&self, other_amount: &Satoshi) -> bool {
        self.lt(other_amount) || self.eq(other_amount)
    }
    pub fn add(&self, other_amount: &Satoshi) -> Satoshi {
        Satoshi(self.0 + other_amount.0)
    }
    pub fn sub(&self, other_amount: &Satoshi) -> Satoshi {
        assert!(self.ge(&other_amount));
        Satoshi(self.0 - other_amount.0)
    }
    pub fn mul(&self, coeff: f64) -> Satoshi {
        // TODO: think of a proper way to round monetary amounts
        Satoshi((self.0 as f64 * coeff) as u64)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn satoshi_equality() {
        let amount1 = Satoshi(2);
        let amount2 = Satoshi(2);
        assert_eq!(amount1, amount2);
        assert!(amount1.ge(&amount2));
        assert!(amount1.le(&amount2));
    }

    #[test]
    fn satoshi_inequality() {
        let higher_amount = Satoshi(3);
        let lower_amount = Satoshi(2);
        assert!(higher_amount.gt(&lower_amount));
        assert!(higher_amount.ge(&lower_amount));
        assert!(!higher_amount.le(&lower_amount));
        assert!(lower_amount.lt(&higher_amount));
        assert!(lower_amount.le(&higher_amount));
        assert!(!lower_amount.ge(&higher_amount));
    }

    #[test]
    fn satoshi_addition() {
        let amount1 = Satoshi(2);
        let amount2 = Satoshi(3);
        let total = amount1.add(&amount2);
        assert_eq!(&total, &Satoshi(5));
    }

    #[test]
    fn satoshi_subtraction() {
        let amount1 = Satoshi(3);
        let amount2 = Satoshi(2);
        let total = amount1.sub(&amount2);
        assert_eq!(&total, &Satoshi(1));
    }

    #[test]
    #[should_panic]
    fn satoshi_subtraction_panic() {
        let amount1 = Satoshi(3);
        let amount2 = Satoshi(4);
        amount1.sub(&amount2);
    }

    #[test]
    fn satoshi_multiplication() {
        let amount = Satoshi(2);
        assert_eq!(&amount.mul(3.0), &Satoshi(6));
        assert_eq!(&amount.mul(3.7), &Satoshi(7));
    }

}