use core::{cell::Cell, num::NonZeroUsize, ptr::NonNull};

pub type ThreadId = NonZeroUsize;

#[inline]
pub fn get() -> ThreadId {
    thread_local! {
        static VALUE: Cell<u32> = Cell::new(4);
    }

    VALUE.with(|v| {
        let ptr = NonNull::from(v);
        let addr = ptr.as_ptr() as usize;
        unsafe { NonZeroUsize::new_unchecked(addr) }
    })
}
