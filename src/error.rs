use warp::reject::{Reject, Rejection};

macro_rules! new_rejection {
    ($i:ident) => {
        #[derive(Debug)]
        pub struct $i;
        impl Reject for $i {}
    };
}

new_rejection!(InvalidKeyError);
new_rejection!(UserAlreadyExistsError);
new_rejection!(InvalidPasswordError);
new_rejection!(InternalServerError);
new_rejection!(InvalidSessionError);
new_rejection!(InvalidQueryError);
new_rejection!(InvalidFileError);
new_rejection!(UserDoesNotExistError);
new_rejection!(PostDoesNotExistError);
