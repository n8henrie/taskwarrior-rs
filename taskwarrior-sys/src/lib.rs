#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
// include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
// include!("bindings.rs");

use cxx::{SharedPtr, UniquePtr};
use std::os::raw::{c_char, c_int};
use std::os::unix::ffi::OsStrExt;

#[cxx::bridge]
mod ffi {
    unsafe extern "C++" {
        include!("taskwarrior-sys/vendor/taskwarrior/src/Context.h");
        include!("taskwarrior-sys/shims.h");
        type Context;

        // Context& Context::getContext ()
        // fn getContext(&self) -> &Context;
        fn Context_newContext() -> UniquePtr<Context>;

        // static void setContext (Context*);
        fn Context_setContext(context: UniquePtr<Context>);

        // fn getContext(self: Pin<&mut Context>) -> Pin<&mut Context>;
        fn Context_getContext() -> UniquePtr<Context>;

        // int Context::initialize (int argc, const char** argv)
        unsafe fn initialize(
            self: Pin<&mut Context>,
            argc: i32,
            argv: *mut *const c_char,
        ) -> i32;
    }
}

#[cfg(test)]
mod tests {
    use super::ffi;
    // use super::Context as OldContext;
    use super::*;
    use ::std::ffi::{c_char, CStr, CString};

    type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

    #[test]
    fn test_cxx() {
        let cmd = CString::new("list").unwrap();
        let mut argv: Vec<*const c_char> = vec![
            CStr::from_bytes_with_nul(b"\0").unwrap().as_ptr(),
            cmd.as_ptr(),
            std::ptr::null(),
        ];

        let context = ffi::Context_newContext();
        unsafe {
            ffi::Context_setContext(context.into_raw());
            let mut context = ffi::Context_getContext();
            context
                .pin_mut()
                .initialize((argv.len() - 1) as i32, argv.as_mut_ptr());
        };
    }

    // #[test]
    // fn test_context_run() -> Result<()> {
    //     let cmd = CString::new("list")?;
    //     let mut argv: Vec<*const c_char> = vec![
    //         CStr::from_bytes_with_nul(b"\0")?.as_ptr(),
    //         cmd.as_ptr(),
    //         std::ptr::null(),
    //     ];

    //     unsafe {
    //         let ctx = newContext();
    //         Context_setContext(ctx);
    //         Context_initialize(
    //             ctx,
    //             (argv.len() - 1) as i32,
    //             argv.as_mut_ptr(),
    //         );
    //         let status = Context_run(ctx);
    //         assert_eq!(status, 0);
    //         delContext(ctx);
    //     };
    //     Ok(())
    // }

    // #[test]
    // fn test_context_dispatch() -> Result<()> {
    //     let cmd = CString::new("list")?;
    //     let mut argv: Vec<*const c_char> = vec![
    //         CStr::from_bytes_with_nul(b"\0")?.as_ptr(),
    //         cmd.as_ptr(),
    //         std::ptr::null(),
    //     ];

    //     unsafe {
    //         let ctx = newContext();
    //         Context_setContext(ctx);
    //         Context_initialize(
    //             ctx,
    //             (argv.len() - 1) as i32,
    //             argv.as_mut_ptr(),
    //         );
    //         let mut output = vec![0u8; 1024];
    //         let ptr = output.as_mut_ptr();
    //         let status = Context_dispatch(ctx, ptr as *mut _);
    //         assert_eq!(status, 0);
    //         let output = CStr::from_ptr(ptr as *const _).to_str().unwrap();
    //         println!("{}", output);
    //         delContext(ctx);
    //     };
    //     Ok(())
    // }
}
