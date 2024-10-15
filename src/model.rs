use serde::Serialize;
use diesel::prelude::*;
use crate::schema::posts;


#[derive(Serialize,Queryable, Selectable)]
#[diesel(table_name = posts)]
#[diesel(check_for_backend(diesel::pg::Pg))]
pub struct Post {
 pub id: i32,
 pub firstname: String,
 pub lastname: String,
 pub email:String,
 pub phone:String,
 pub filepath:String
}




#[derive(Insertable)]
#[diesel(table_name = posts)]
pub struct NewPost{
 pub firstname: String,
 pub lastname: String,
 pub email:String,
 pub phone:String,
 pub filepath:String 
}

