use std::{
    error,
    ffi::{CStr, CString, c_char},
    result,
};

use taskwarrior_sys::{
    Context_initialize, Context_run, Context_setContext, delContext,
    newContext,
};

type Error = Box<dyn error::Error>;
type Result<T> = result::Result<T, Error>;

fn context_run() -> Result<()> {
    let cmd = CString::new("list")?;
    let mut argv: Vec<*const c_char> = vec![
        CStr::from_bytes_with_nul(b"\0")?.as_ptr(),
        cmd.as_ptr(),
        std::ptr::null(),
    ];

    unsafe {
        let ctx = newContext();
        Context_setContext(ctx);
        Context_initialize(ctx, (argv.len() - 1) as i32, argv.as_mut_ptr());
        let _status = Context_run(ctx);
        delContext(ctx);
    };
    Ok(())
}

pub fn run() -> std::result::Result<(), Box<dyn std::error::Error>> {
    context_run()
}
