use warp::reject::{Reject, Rejection};

macro_rules! new_rejections {
    ($i:ident) => {
        #[derive(Debug)]
        pub struct $i;
        impl Reject for $i {}
    };
    ($i0:ident $(, $i:ident)+) => {
        new_rejections!($i0);
        $(new_rejections!($i);)*
    };
}

new_rejections!(
    InvalidKeyError,
    UserAlreadyExistsError,
    InvalidPasswordError,
    InternalServerError,
    InvalidSessionError,
    InvalidQueryError,
    InvalidFileError,
    UserDoesNotExistError
);
