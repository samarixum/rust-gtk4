use std::collections::HashMap;
use std::env;

pub fn get_env_vars() -> HashMap<String, String> {
    env::vars().collect()
}