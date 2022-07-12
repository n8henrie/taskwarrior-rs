#![warn(clippy::pedantic)]
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
    fn test_new() -> Result<()> {
        let cmd = CString::new("list")?;
        let mut argv: Vec<*const c_char> =
            vec![CString::new("")?.as_ptr(), cmd.as_ptr(), std::ptr::null()];

        unsafe {
            let ctx = Context_context;
            Context_setContext(ctx);
            let status = Context_initialize(
                ctx,
                i32::try_from(argv.len() - 1)?,
                argv.as_mut_ptr(),
            );
            dbg!(status);
            // let status = Context_run(Context_getContext());
            // dbg!(status);

            let mut ctx = Context::new(Context_context);
            Context::setContext(&mut ctx);
            let status = ctx
                .initialize(i32::try_from(argv.len() - 1)?, argv.as_mut_ptr());
            dbg!(status);
            let status = ctx.run();
            dbg!(status);
        };
        Ok(())
    }

    //     #[test]
    //     fn test_main() -> Result<()> {
    //         let cmd = CString::new("list")?;
    //         let mut argv: Vec<*const c_char> =
    //             vec![CString::new("")?.as_ptr(), cmd.as_ptr(), std::ptr::null()];
    //
    //         unsafe {
    //             let ctx = newContext();
    //             Context_setContext(ctx);
    //             Context_initialize(
    //                 ctx,
    //                 i32::try_from(argv.len() - 1)?,
    //                 argv.as_mut_ptr(),
    //             );
    //             Context_run(ctx);
    //         };
    //         Ok(())
    //     }
    //
    //     #[test]
    //     fn test_dispatch() -> Result<()> {
    //         let cmd = CString::new("list")?;
    //         let mut argv: Vec<*const c_char> =
    //             vec![CString::new("")?.as_ptr(), cmd.as_ptr(), std::ptr::null()];
    //
    //         let out = CString::new("")?;
    //         unsafe {
    //             let ctx = newContext();
    //             Context_setContext(ctx);
    //             Context_initialize(
    //                 ctx,
    //                 i32::try_from(argv.len() - 1)?,
    //                 argv.as_mut_ptr(),
    //             );
    //
    //             let ptr = out.into_raw();
    //             Context_dispatch(ctx, ptr);
    //             let out = CString::from_raw(ptr).into_string()?;
    //             delContext(ctx);
    //             println!("out: {}", out);
    //         };
    //         Ok(())
    //     }
}
