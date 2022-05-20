pub struct TeamQuantization {
    pub shield_coverage: f64,
}

impl Default for TeamQuantization {
    fn default() -> Self {
        TeamQuantization {
            shield_coverage: 0.0
        }
    }
}