use crate::models::User;
use crate::error_handler::CustomError;

pub async fn delete_user_by_id(id: i32) -> Result<(), CustomError> {
    User::delete(id)?;
    Ok(())
}
