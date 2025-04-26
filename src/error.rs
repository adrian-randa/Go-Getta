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
    InvalidSessionError,
    InvalidQueryError,
    InvalidFileError,
    InvalidUsernameError,
    InvalidPublicNameError,
    InvalidBiographyError,
    InvalidPasswordError,
    UserAlreadyExistsError,
    UserDoesNotExistError,
    PostDoesNotExistError,
    RoomDoesNotExistError,
    InternalServerError,
    InsufficientPermissionsError,
    EmptyContentError,
    ContentTooLargeError,
    RoomBoundaryViolationError
);
