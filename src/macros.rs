macro_rules! bl_enum {
    (
        $(#[$meta:meta])*
        $vis:vis enum $name:ident {
            $(
                $(#[$meta_var:meta])*
                $variant:ident = $value:ident,
             )*
        }
        Default => $default:ident
    ) => {
        $(#[$meta])*
        #[derive(Copy, Clone, Debug, PartialEq, PartialOrd)]
        $vis enum $name {
            $(
                $(#[$meta_var])*
                $variant = $value as _,
            )*
        }
        impl From<u32> for $name {
            fn from(val: u32) -> Self {
                match val as _ {
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
