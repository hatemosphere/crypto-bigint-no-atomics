//! Type alias for Arc/Rc depending on target atomic support
#![allow(unsafe_code)]

#[cfg(target_has_atomic = "ptr")]
pub use alloc::sync::Arc as SharedPtr;

#[cfg(not(target_has_atomic = "ptr"))]
pub use self::single_thread::SharedPtr;

#[cfg(not(target_has_atomic = "ptr"))]
mod single_thread {
    use alloc::rc::Rc;
    use core::ops::Deref;
    use core::fmt;
    
    /// Wrapper around Rc that implements Send + Sync for single-threaded environments
    #[derive(Clone, Debug, Eq, PartialEq)]
    pub struct SharedPtr<T>(Rc<T>);
    
    impl<T> SharedPtr<T> {
        pub fn new(value: T) -> Self {
            SharedPtr(Rc::new(value))
        }
    }
    
    impl<T> From<T> for SharedPtr<T> {
        fn from(value: T) -> Self {
            SharedPtr::new(value)
        }
    }
    
    impl<T> Deref for SharedPtr<T> {
        type Target = T;
        
        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }
    
    impl<T: fmt::Display> fmt::Display for SharedPtr<T> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            self.0.fmt(f)
        }
    }
    
    // SAFETY: This is only safe in single-threaded environments (like RISC-V zkVM)
    unsafe impl<T> Send for SharedPtr<T> {}
    unsafe impl<T> Sync for SharedPtr<T> {}
}