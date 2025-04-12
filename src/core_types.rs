use nom::AsBytes;
use sha2::Digest;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum OmniType {
    List(Vec<OmniType>),
    Int(i32),
    Hash(String),
    Symbol(String),
    Quote(Box<OmniType>),
}

impl OmniType {
    pub fn hash(self: &Self) -> String {
        let min_form = self.format_min();
        let digest = sha2::Sha256::digest(min_form);
        let mut buf = [0u8; 64];
        let str = base16ct::lower::encode_str(digest.as_bytes(), &mut buf).unwrap();
        str.to_owned()
    }
}
