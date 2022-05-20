pub mod artifact;

pub fn set_panic_hook() {
    #[cfg(feature = "console_error_panic_hook")]
    console_error_panic_hook::set_once();
}

#[cfg(not(target_family = "wasm"))]
pub fn my_log(s: &str) {
    println!("{}", s);
}

#[cfg(target_family = "wasm")]
pub fn my_log(s: &str) {
    web_sys::console::log_1(&s.into());
}

#[macro_export]
macro_rules! log {
    ( $( $t:tt )* ) => {
        crate::utils::my_log(&format!( $( $t )* ));
    }
}

// macro_rules! log {
//     ( $( $t:tt )* ) => {
//         web_sys::console::log_1(&format!( $( $t )* ).into());
//     }
// }

pub(crate) use log;
