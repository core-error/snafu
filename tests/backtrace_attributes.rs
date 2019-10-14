#![cfg(all(feature = "std", feature = "backtraces"))]

use snafu::{Backtrace, Snafu};

#[derive(Debug, Snafu)]
enum Error {
    NoArgument {
        #[snafu(backtrace)]
        thing: Backtrace,
    },

    ExplicitTrue {
        #[snafu(backtrace(true))]
        thing: Backtrace,
    },

    ExplicitFalse {
        #[snafu(backtrace(false))]
        backtrace: i32,
    },
}

fn example() -> Result<(), Error> {
    NoArgument.fail()?;
    ExplicitTrue.fail()?;
    ExplicitFalse { backtrace: 42 }.fail()?;
    Ok(())
}

#[test]
fn implements_error() {
    fn check<T: snafu::Error>() {}
    check::<Error>();
    example().unwrap_err();
}
