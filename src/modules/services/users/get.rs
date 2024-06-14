use crate::models::User;
use crate::error_handler::CustomError;

pub async fn get_user_by_id(id: i32) -> Result<User, CustomError> {
    let user = User::find(id)?;
    Ok(user)
}
