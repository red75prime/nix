use ::libc;
#[cfg(not(target_env = "musl"))]
use crate::{Errno, Result};
use std::mem;
use crate::sys::signal::SigSet;

#[derive(Clone, Copy)]
pub struct UContext {
    context: libc::ucontext_t,
}

impl UContext {
    #[cfg(not(target_env = "musl"))]
    pub fn get() -> Result<UContext> {
        let mut context = std::mem::MaybeUninit::<libc::ucontext_t>::uninit();
        unsafe {
            let res = libc::getcontext(context.as_mut_ptr());
            Errno::result(res).map(|_| UContext { context: context.assume_init() })
        }
    }

    #[cfg(not(target_env = "musl"))]
    pub fn set(&self) -> Result<()> {
        let res = unsafe {
            libc::setcontext(&self.context as *const libc::ucontext_t)
        };
        Errno::result(res).map(drop)
    }

    pub fn sigmask_mut(&mut self) -> &mut SigSet {
        unsafe { mem::transmute(&mut self.context.uc_sigmask) }
    }

    pub fn sigmask(&self) -> &SigSet {
        unsafe { mem::transmute(&self.context.uc_sigmask) }
    }
}
