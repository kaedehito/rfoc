#![deny(warnings)]

#[macro_export]
macro_rules! err {

    ( $( $s:expr ),* ) => {
        {
            let result = vec![
                $( format!("{}", $s) ),*
            ].join("");
            println!("{}: {}","\x1b[31;1mError\x1b[0m" ,result);
            std::process::exit(1);
        }
    }

}

pub trait RfocResultExtended<T, E> {
    fn rfoc_unwrap(self) -> T;
}

pub trait RfocOptionExtended<T> {
    fn rfoc_unwrap(self) -> T;
}

impl<T, E: std::fmt::Display> RfocResultExtended<T, E> for Result<T, E> {
    fn rfoc_unwrap(self) -> T {
        let s = match self {
            Err(e) => {
                err!(e);
            }
            Ok(o) => o,
        };
        s
    }
}

impl<T> RfocOptionExtended<T> for Option<T> {
    fn rfoc_unwrap(self) -> T {
        let s = match self {
            None => {
                err!("The null value");
            }
            Some(o) => o,
        };
        s
    }
}
