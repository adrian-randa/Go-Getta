use warp::reject::Reject;

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