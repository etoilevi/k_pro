pub mod io;
pub mod math;

#[macro_export]
macro_rules! echo_str {
    ( $( $x:expr ),* ) => {
        {
            let mut s = vec![];
            $(
                s.push($x.to_string());
            )*
            s.join(" ")
        }
    };
}

#[macro_export]
macro_rules! echo {
    ( $( $x:expr ),* ) => {
        println!("{}", $crate::echo_str!( $( $x ),* ));
    };
}
