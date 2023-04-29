use std::env;

/// Returns a vector of string that holds args from env
pub fn get_args() -> Vec<String> {
    env::args().collect()
}
