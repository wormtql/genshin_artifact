pub struct Reaction {}

impl Reaction {
    pub fn amp(em: f64) -> f64 {
        em * 25.0 / (9.0 * (em + 1400.0))
    }

    pub fn transformative(em: f64) -> f64 {
        em * 16.0 / (em + 2000.0)
    }

    pub fn catalyze(em: f64) -> f64 {
        em * 5.0 / (em + 1200.0)
    }
}