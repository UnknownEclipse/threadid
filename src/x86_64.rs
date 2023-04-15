use core::{arch::asm, cell::Cell, num::NonZeroUsize, ptr::NonNull};

pub type ThreadId = NonZeroUsize;

#[inline]
pub fn get() -> ThreadId {
    let value;
    unsafe {
        asm!("mov {}, fs", out(reg) value, options(nomem, nostack, preserves_flags));
        NonZeroUsize::new_unchecked(value)
    }
}
