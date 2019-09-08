use crate::hyp;
use super::BundledModules;
use std::collections::HashMap;

pub struct Bundler;

fn safe_for_js(b: u8) -> bool {
    b == 9 || (b >= 32 && b <= 33) || (b >= 35 && b <= 91) || (b >= 93 && b <= 126)
}

impl Bundler {
    pub fn bundle(module_infos: &mut Vec<hyp::ModuleInfo>) -> BundledModules {

        let mut res = BundledModules {
            js_safe: vec![],
            base64: vec![],
            offsets: HashMap::new()
        };

        // TODO: Sort to maximize compressibility

        for (i, mi) in module_infos.iter().enumerate() {
            if mi.language == hyp::Language::Binary {
                if mi.src.iter().cloned().all(safe_for_js) {
                    let offset = res.js_safe.len();
                    res.js_safe.extend_from_slice(&mi.src[..]);
                    res.offsets.insert(i as u32, (true, offset));
                } else {
                    let offset = res.base64.len();
                    res.base64.extend_from_slice(&mi.src[..]);
                    res.offsets.insert(i as u32, (false, offset));
                }
            }
        }

        res
    }
}