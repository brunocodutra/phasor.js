use super::*;
use std::fmt::{Display, Error, Formatter, LowerExp, UpperExp};

impl Display for Polar {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}∠{}", self.norm(), self.angle())
    }
}

impl LowerExp for Polar {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{:e}∠{:e}", self.norm(), self.angle())
    }
}

impl UpperExp for Polar {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{:E}∠{:E}", self.norm(), self.angle())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn starts_with_the_norm(polar: Polar) {
            assert!(format!("{}", polar).starts_with(&format!("{}", polar.norm())));
            assert!(format!("{:e}", polar).starts_with(&format!("{:e}", polar.norm())));
            assert!(format!("{:E}", polar).starts_with(&format!("{:E}", polar.norm())));
        }

        #[test]
        fn ends_with_the_angle(polar: Polar) {
            assert!(format!("{}", polar).ends_with(&format!("{}", polar.angle())));
            assert!(format!("{:e}", polar).ends_with(&format!("{:e}", polar.angle())));
            assert!(format!("{:E}", polar).ends_with(&format!("{:E}", polar.angle())));
        }

        #[test]
        fn connects_norm_and_angle_by_the_unicode_angle_symbol(polar: Polar) {
            assert!(format!("{}", polar).contains("∠"));
            assert!(format!("{:e}", polar).contains("∠"));
            assert!(format!("{:E}", polar).contains("∠"));
        }
    }
}
