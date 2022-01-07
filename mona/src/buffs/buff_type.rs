use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub enum BuffType {
    ATKPercentage(f64)
}