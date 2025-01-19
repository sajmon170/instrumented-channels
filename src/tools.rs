use base64::prelude::*;

pub fn base64_text(hash: &[u8]) -> String {
    BASE64_STANDARD.encode(hash)
}
