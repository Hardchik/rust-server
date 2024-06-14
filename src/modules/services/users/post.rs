use crate::models::{User, UserDto};
use crate::error_handler::CustomError;

pub async fn create_user(new_user: UserDto) -> Result<User, CustomError> {
    let value = User::create(new_user)?;
    Ok(value)
}
