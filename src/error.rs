use warp::reject::{Reject, Rejection};

macro_rules! new_rejections {
    ($i0:ident $(, $i:ident)*) => {
        #[derive(Debug)]
        pub struct $i0;
        impl Reject for $i0 {}
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
    UserDoesNotExistError,
    PostDoesNotExistError,
    InsufficientPermissionsError
);
