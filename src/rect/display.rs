use super::*;
use std::fmt::{Display, Error, Formatter, LowerExp, UpperExp};

impl Display for Rect {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        if self.imag().is_sign_negative() {
            write!(f, "{} - i{}", self.real(), -self.imag())
        } else {
            write!(f, "{} + i{}", self.real(), self.imag())
        }
    }
}

impl LowerExp for Rect {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        if self.imag().is_sign_negative() {
            write!(f, "{:e} - i{:e}", self.real(), -self.imag())
        } else {
            write!(f, "{:e} + i{:e}", self.real(), self.imag())
        }
    }
}

impl UpperExp for Rect {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        if self.imag().is_sign_negative() {
            write!(f, "{:E} - i{:E}", self.real(), -self.imag())
        } else {
            write!(f, "{:E} + i{:E}", self.real(), self.imag())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn starts_with_real_part(rect: Rect) {
            assert!(format!("{}", rect).starts_with(&format!("{}", rect.real())));
            assert!(format!("{:e}", rect).starts_with(&format!("{:e}", rect.real())));
            assert!(format!("{:E}", rect).starts_with(&format!("{:E}", rect.real())));
        }

        #[test]
        fn ends_with_unsinged_imaginary_part_preceded_by_i(rect: Rect) {
            assert!(format!("{}", rect).ends_with(&format!("i{}", rect.imag().abs())));
            assert!(format!("{:e}", rect).ends_with(&format!("i{:e}", rect.imag().abs())));
            assert!(format!("{:E}", rect).ends_with(&format!("i{:E}", rect.imag().abs())));
        }

        #[test]
        fn connects_real_and_imaginary_part_by_sign_of_the_latter(rect: Rect) {
            let sign = if rect.imag().is_sign_negative() { "-" } else { "+" };
            assert!(format!("{}", rect).contains(&format!(" {} ", sign)));
            assert!(format!("{:e}", rect).contains(&format!(" {} ", sign)));
            assert!(format!("{:E}", rect).contains(&format!(" {} ", sign)));
        }
    }
}
