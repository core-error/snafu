#![cfg(feature = "std")]

mod inner {
    use snafu::{ensure, Snafu};

    #[derive(Debug, Snafu)]
    pub struct Error(InnerError);

    pub fn api() -> Result<i32, Error> {
        Ok(a()? + b()?)
    }

    pub fn not_positive(value: i32) -> Result<i32, Error> {
        ensure!(value < 1, TooBig { count: value });
        Ok(value)
    }

    pub fn boxed_inner(value: i32) -> Result<i32, Box<dyn snafu::Error>> {
        ensure!(value < 1, TooBig { count: value });
        Ok(value)
    }

    #[derive(Debug, Snafu)]
    enum InnerError {
        #[snafu(display = r#"("The value {} is too big", count)"#)]
        TooBig { count: i32 },
    }

    fn a() -> Result<i32, InnerError> {
        TooBig { count: 1 }.fail()
    }

    fn b() -> Result<i32, InnerError> {
        TooBig { count: 2 }.fail()
    }
}

#[test]
fn implements_error() {
    fn check<T: snafu::Error>() {}
    check::<inner::Error>();
    let e = inner::api().unwrap_err();
    assert!(e.to_string().contains("too big"));
}

#[test]
fn ensure_opaque() {
    assert!(inner::not_positive(-1).is_ok());

    let e = inner::not_positive(2).unwrap_err();
    assert!(e.to_string().contains("too big"));
}

#[test]
fn ensure_boxed() {
    assert!(inner::boxed_inner(-1).is_ok());

    let e = inner::boxed_inner(2).unwrap_err();
    assert!(e.to_string().contains("too big"));
}
