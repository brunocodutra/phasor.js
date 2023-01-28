use super::Phasor;
use std::fmt::{Display, Error, Formatter, LowerExp, UpperExp};

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
    use test_strategy::proptest;

    #[proptest]
    fn starts_with_the_norm(p: Phasor) {
        assert!(format!("{p}").starts_with(&format!("{}", p.norm())));
        assert!(format!("{p:e}").starts_with(&format!("{:e}", p.norm())));
        assert!(format!("{p:E}").starts_with(&format!("{:E}", p.norm())));
    }

    #[proptest]
    fn ends_with_the_angle(p: Phasor) {
        assert!(format!("{p}").ends_with(&format!("{}", p.angle())));
        assert!(format!("{p:e}").ends_with(&format!("{:e}", p.angle())));
        assert!(format!("{p:E}").ends_with(&format!("{:E}", p.angle())));
    }

    #[proptest]
    fn connects_norm_and_angle_by_the_unicode_angle_symbol(p: Phasor) {
        assert!(format!("{p}").contains('∠'));
        assert!(format!("{p:e}").contains('∠'));
        assert!(format!("{p:E}").contains('∠'));
    }
}
