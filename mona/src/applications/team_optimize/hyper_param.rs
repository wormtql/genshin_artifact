use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
pub struct TeamOptimizeHyperParam {
    pub mva_step: usize,
    pub work_space: usize,
    pub max_re_optimize: usize,
    pub max_search: usize,
    pub count: usize,
}

impl Default for TeamOptimizeHyperParam {
    fn default() -> Self {
        TeamOptimizeHyperParam {
            mva_step: 5,
            work_space: 1000,
            max_re_optimize: 5,
            max_search: 200000,
            count: 100,
        }
    }
}
