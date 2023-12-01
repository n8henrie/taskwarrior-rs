#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
// include!("bindings.rs");

#[cfg(test)]
mod tests {
    use super::*;
    use ::std::ffi::{c_char, CStr, CString};

    type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;

    #[test]
    fn test_context_run() -> Result<()> {
        let cmd = CString::new("list")?;
        let mut argv: Vec<*const c_char> = vec![
            CStr::from_bytes_with_nul(b"\0")?.as_ptr(),
            cmd.as_ptr(),
            std::ptr::null(),
        ];

        unsafe {
            let ctx = newContext();
            Context_setContext(ctx);
            Context_initialize(
                ctx,
                (argv.len() - 1) as i32,
                argv.as_mut_ptr(),
            );
            let status = Context_run(ctx);
            assert_eq!(status, 0);
            delContext(ctx);
        };
        Ok(())
    }

    #[test]
    fn test_context_dispatch() -> Result<()> {
        let cmd = CString::new("list")?;
        let mut argv: Vec<*const c_char> = vec![
            CStr::from_bytes_with_nul(b"\0")?.as_ptr(),
            cmd.as_ptr(),
            std::ptr::null(),
        ];

        unsafe {
            let ctx = newContext();
            Context_setContext(ctx);
            Context_initialize(
                ctx,
                (argv.len() - 1) as i32,
                argv.as_mut_ptr(),
            );
            let mut output = vec![0u8; 1024];
            let ptr = output.as_mut_ptr();
            let status = Context_dispatch(ctx, ptr as *mut _);
            assert_eq!(status, 0);
            let output = CStr::from_ptr(ptr as *const _).to_str().unwrap();
            println!("{}", output);
            delContext(ctx);
        };
        Ok(())
    }
}
