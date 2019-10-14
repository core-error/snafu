#![cfg(feature = "std")]

mod default_with_lifetime {
    use snafu::{AsErrorSource, Snafu};
    use std::fmt::{Debug, Display};

    #[derive(Debug, Snafu)]
    pub struct ApiError<'a, S, T>(Error<'a, S, T>)
    where
        T: Debug + Display,
        S: snafu::Error + AsErrorSource;

    #[derive(Debug, Snafu)]
    enum Error<'a, S = std::io::Error, T = String>
    where
        T: Display,
        S: snafu::Error + AsErrorSource,
    {
        #[snafu(display("Boom: {}", value))]
        Boom {
            value: T,
            name: &'a str,
        },
        WithSource {
            source: S,
        },
        Empty,
    }

    #[test]
    fn implements_error() {
        fn check_bounds<T: snafu::Error>() {}
        check_bounds::<Error<std::io::Error>>();
        check_bounds::<ApiError<std::io::Error, i32>>();
    }
}
