pub trait Phasor {
    fn real(&self) -> f64;
    fn imag(&self) -> f64;
    fn norm(&self) -> f64;
    fn angle(&self) -> f64;
}
