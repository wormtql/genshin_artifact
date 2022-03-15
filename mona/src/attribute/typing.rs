// pub type EdgeFunction = Box<dyn Fn(f64) -> (String, f64)>;
pub type EdgeFunctionFwd = Box<dyn Fn(f64, f64) -> f64>;

// grad, x1, x2 -> grad_x1, grad_x2
pub type EdgeFunctionBwd = Box<dyn Fn(f64, f64, f64) -> (f64, f64)>;