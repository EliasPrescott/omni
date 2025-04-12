use std::rc::Rc;

use nom::AsBytes;
use sha2::Digest;

use crate::interpreter::environment::OmniEnvironment;
use crate::registry::OmniRegistry;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OmniType {
    List(Vec<OmniType>),
    Int(i32),
    Hash(String),
    Symbol(String),
    Quote(Box<OmniType>),
    QuasiQuote(Vec<OmniType>),
    UnQuote(Box<OmniType>),
    Spread(Box<OmniType>),
}

impl OmniType {
    pub fn hash(self: &Self, environment: Rc<OmniEnvironment>, registry: &dyn OmniRegistry) -> (String, String) {
        let min_form = self.resolving_format_min(environment, registry);
        let digest = sha2::Sha256::digest(&min_form);
        let mut buf = [0u8; 64];
        let str = base16ct::lower::encode_str(digest.as_bytes(), &mut buf).unwrap();
        (str.to_owned(), min_form)
    }
}
