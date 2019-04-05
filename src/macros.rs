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

#[macro_export]
macro_rules! bl_enum {
    (pub enum $name:ident { $( $variant:ident = $value:ident, )* } Default => $default:ident) => {
        #[repr(i32)]
        #[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
        pub enum $name {
            $(
                $variant = $value,
            )*
        }
        impl From<u32> for $name {
            fn from(val: u32) -> Self {
                match val as i32 {
                    $(
                        $value => $name::$variant,
                    )*
                    _ => $name::$default,
                }
            }
        }
        impl From<$name> for u32 {
            fn from(val: $name) -> u32 {
                val as u32
            }
        }
        impl Default for $name {
            fn default() -> Self {
                $name::$default
            }
        }
    };
}
