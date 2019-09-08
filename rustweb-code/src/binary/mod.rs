pub mod bundler;
use std::collections::HashMap;

pub struct BundledModules {
    pub js_safe: Vec<u8>,
    pub base64: Vec<u8>,
    pub offsets: HashMap<u32, (bool, usize)> // (true, _) -> js_safe, otherwise base64
}