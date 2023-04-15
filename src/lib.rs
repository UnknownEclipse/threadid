#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "nightly", feature(thread_local))]

use cfg_if::cfg_if;

cfg_if! {
    if #[cfg(target_arch = "x86_64")] {
        #[path = "x86_64.rs"]
        mod imp;
    } else if #[cfg(feature = "nightly")] {
        #[path = "std_nightly.rs"]
        mod imp;
    } else if #[cfg(feature = "std")] {
        #[path = "std_tls.rs"]
        mod imp;
    } else {
        compile_error!("thread id support not available for current platform");
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct ThreadId(imp::ThreadId);

#[inline]
pub fn get() -> ThreadId {
    ThreadId(imp::get())
}
