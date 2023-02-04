pub trait UnsafeInitialization {
    type Item;

    fn new_zeroed() -> Self::Item {
        unsafe { std::mem::zeroed::<Self::Item>() }
    }

    fn from_slice(s: &[u8]) -> std::result::Result<Self::Item, Box<dyn std::error::Error>> {
        if s.len() < std::mem::size_of::<Self::Item>() {
            return Err("Initialization failed due slice length being smaller than dest size".into());
        }

        let mut r: Self::Item = Self::new_zeroed();

        unsafe { std::ptr::copy(s.as_ptr() as *const _, &mut r as *mut _ as *mut _, std::mem::size_of::<Self::Item>()) }

        Ok(r)
    }

    fn size() -> usize {
        std::mem::size_of::<Self::Item>()
    }
}

#[macro_export]
macro_rules! impl_UnsafeInitialization {
    (for $($t:ty),+) => {
        $(impl UnsafeInitialization for $t {
            type Item = Self;
        })*
    }
}