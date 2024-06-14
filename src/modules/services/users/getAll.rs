use crate::models::User;
use crate::error_handler::CustomError;

pub async fn get_all_users() -> Result<Vec<User>, CustomError> {
    let users = User::find_all()?;
    Ok(users)
}
