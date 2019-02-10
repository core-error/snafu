extern crate snafu;

use snafu::Snafu;

#[derive(Debug, Snafu)]
pub enum Error {
    #[snafu_display("User ID {} is invalid", "user_id")]
    InvalidUser { user_id: i32 },
}

#[test]
fn implements_error() {
    fn check<T: std::error::Error>() {}
    check::<Error>();
}