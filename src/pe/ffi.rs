use super::PortableExecutable;

#[no_mangle]
pub unsafe extern "C" fn from_slice(_me: *const Box<dyn PortableExecutable>) {
}