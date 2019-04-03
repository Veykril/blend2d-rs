macro_rules! vcall {
    ($ctx:expr=>$func:ident( $( $arg:expr ),* )) => {
        unsafe {
            if let Some(func) = (*(*$ctx.impl_).virt).$func {
                crate::error::errcode_to_result(func($ctx.impl_, $( $arg ),*))
            } else {
                Ok(())
            }
        }
    }
}
