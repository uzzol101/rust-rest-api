use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use diesel::result::Error as DieselError;
use serde_json::json;
use crate::schema::user;
use crate::db::connection;

#[derive(Serialize, Deserialize, Queryable)]
pub struct User {
    pub id: i32,
    pub name: String,
    pub email: String,
}


#[derive(Insertable, Queryable, AsChangeset, Deserialize)]
#[table_name="user"]
pub struct Newuser {
    pub name: String,
    pub email: String,
}

impl User {
    pub fn find_all() -> Result<Vec<User>, DieselError> {
        let conn = connection();
        
        let users = user::table.load::<User>(&conn)?;
        Ok(users)
    }

    pub fn find(id:i32) -> Result<User, DieselError> {
        let conn = connection();

        let user = user::table.filter(user::id.eq(id)).first(&conn)?;
        Ok(user)
    }

    pub fn create(user: Newuser) -> Result<User, DieselError> {
        let conn = connection();
        let user = diesel::insert_into(user::table).values(user).get_result::<User>(&conn)?;

        Ok(user)
    }

    pub fn update(id: i32, user: Newuser) -> Result<User, DieselError> {
        let conn = connection();

       let user = diesel::update(user::table).filter(user::id.eq(id)).set(user).get_result::<User>(&conn)?;

       Ok(user)
    }

    pub fn delete(id: i32) -> Result<usize, DieselError> {
        let conn = connection();

        let user_id = diesel::delete(user::table).filter(user::id.eq(id)).execute(&conn)?;

        Ok(user_id)
    }

}