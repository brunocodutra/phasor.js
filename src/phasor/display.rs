use super::Phasor;
use core::fmt::{Display, Error, Formatter, LowerExp, UpperExp};

impl Display for Phasor {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{}∠{}", self.norm(), self.angle())
    }
}

impl LowerExp for Phasor {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{:e}∠{:e}", self.norm(), self.angle())
    }
}

impl UpperExp for Phasor {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        write!(f, "{:E}∠{:E}", self.norm(), self.angle())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::format;
    use proptest::prelude::*;

    proptest! {
        #[test]
        fn starts_with_the_norm(p: Phasor) {
            assert!(format!("{}", p).starts_with(&format!("{}", p.norm())));
            assert!(format!("{:e}", p).starts_with(&format!("{:e}", p.norm())));
            assert!(format!("{:E}", p).starts_with(&format!("{:E}", p.norm())));
        }

        #[test]
        fn ends_with_the_angle(p: Phasor) {
            assert!(format!("{}", p).ends_with(&format!("{}", p.angle())));
            assert!(format!("{:e}", p).ends_with(&format!("{:e}", p.angle())));
            assert!(format!("{:E}", p).ends_with(&format!("{:E}", p.angle())));
        }

        #[test]
        fn connects_norm_and_angle_by_the_unicode_angle_symbol(p: Phasor) {
            assert!(format!("{}", p).contains("∠"));
            assert!(format!("{:e}", p).contains("∠"));
            assert!(format!("{:E}", p).contains("∠"));
        }
    }
}
