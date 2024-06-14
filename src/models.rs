use diesel::prelude::*;
use crate::db;
use serde::{Deserialize, Serialize};
use crate::schema::users;
use crate::error_handler::CustomError;

#[derive(Serialize, Deserialize, AsChangeset, Insertable)]
#[table_name = "users"]
pub struct UserDto {
    pub username: String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Queryable)]
pub struct User {
    pub userid: i32,
    pub username: String,
    pub password: String,
}

impl User {
    pub fn find_all() -> Result<Vec<Self>, CustomError> {
        let conn = db::connection()?;
        let users = users::table.load::<User>(&conn)?;
        Ok(users)
    }

    pub fn find(id: i32) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let value = users::table.filter(users::userid.eq(id)).first(&conn)?;
        Ok(value)
    }

    pub fn create(value: UserDto) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let value = UserDto::from(value);
        let value = diesel::insert_into(users::table)
            .values(value)
            .get_result(&conn)?;
        Ok(value)
    }

    pub fn update(id: i32, value: UserDto) -> Result<Self, CustomError> {
        let conn = db::connection()?;
        let value = diesel::update(users::table)
            .filter(users::userid.eq(id))
            .set(value)
            .get_result(&conn)?;
        Ok(value)
    }

    pub fn delete(id: i32) -> Result<usize, CustomError> {
        let conn = db::connection()?;
        let res = diesel::delete(users::table.filter(users::userid.eq(id))).execute(&conn)?;
        Ok(res)
    }
}

impl UserDto {
    fn from(user: UserDto) -> UserDto {
        UserDto {
            username: user.username,
            password: user.password,
        }
    }
}