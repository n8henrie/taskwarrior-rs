#![allow(non_camel_case_types, non_snake_case, non_upper_case_globals)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
// include!("bindings.rs");

#[cfg(test)]
mod tests {
    use super::*;
    use ::std::ffi::CString;
    use std::os::raw::c_char;

    type Result<T> = ::std::result::Result<T, Box<dyn ::std::error::Error>>;
    #[test]
    fn test_main() -> Result<()> {
        let first = CString::new("foo")?;
        let word = CString::new("list")?;
        let mut argv: Vec<*const c_char> =
            vec![first.as_ptr(), word.as_ptr(), std::ptr::null()];

        dbg!(unsafe {
            let ctx = newContext();
            Context_setContext(ctx);
            Context_initialize(
                ctx,
                (argv.len() - 1) as i32,
                argv.as_mut_ptr(),
            );
            Context_run(ctx);
            delContext(ctx);
        });
        Ok(())
    }
}
