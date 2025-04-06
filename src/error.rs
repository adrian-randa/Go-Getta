use warp::reject::{Reject, Rejection};

#[derive(Debug)]
pub struct InvalidKeyError;
impl Reject for InvalidKeyError {}

#[derive(Debug)]
pub struct UserAlreadyExistsError;
impl Reject for UserAlreadyExistsError {}

#[derive(Debug)]
pub struct InvalidPasswordError;
impl Reject for InvalidPasswordError {}

#[derive(Debug)]
pub struct InternalServerError;
impl Reject for InternalServerError {}

#[derive(Debug)]
pub struct InvalidSessionError;
impl Reject for InvalidSessionError {}

#[derive(Debug)]
pub struct InvalidQueryError;
impl Reject for InvalidQueryError {}

#[derive(Debug)]
pub struct InvalidFileError;
impl Reject for InvalidFileError {}

#[derive(Debug)]
pub struct UserDoesNotExistError;
impl Reject for UserDoesNotExistError {}

#[derive(Debug)]
pub struct PostDoesNotExistError;
impl Reject for PostDoesNotExistError {}