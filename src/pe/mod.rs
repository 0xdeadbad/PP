pub mod pe32;
pub mod pe64;
pub mod ffi;

pub trait PortableExecutable {
    fn from_slice(s: &[u8]) -> std::result::Result<Self, Box<dyn std::error::Error>> where Self: Sized;
    fn get_relocations(&mut self, s: &[u8]) -> std::result::Result<Vec<super::headers::BaseRelocation>, Box<dyn std::error::Error>>;
}