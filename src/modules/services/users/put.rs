use crate::models::{User, UserDto};
use crate::error_handler::CustomError;

pub async fn update_user_by_id(id: i32, user_dto: UserDto) -> Result<User, CustomError> {
    let user = User::update(id, user_dto)?;
    Ok(user)
}
