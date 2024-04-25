pub mod records;

use super::Cloudflare;

pub struct Dns {
    cloudflare: Cloudflare,
}

impl Cloudflare {
    pub fn dns(&self) -> Dns {
        Dns {
            cloudflare: self.clone(),
        }
    }
}
