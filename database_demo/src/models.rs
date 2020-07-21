
use super::schema::users;
use super::schema::contents;

#[derive(Identifiable,Queryable,Associations,PartialEq,Debug)]
#[table_name="users"]
pub struct User {
    pub id:i32,
    pub username:String,
    pub email:String
}

// #[derive(Identifiable,Queryable,Associations,PartialEq,Debug)]

// #[belongs_to(User)]
// #[table_name="contents"]
// #[belongs_to(User)]
// pub struct Content{
//     postId:i32,
//     user_id:i32,
//     title:String,
//     content:String
// }

#[derive(Insertable)]
#[table_name="contents"]
pub struct NewContent <'a>{
    pub title:&'a str,
    pub content:&'a str
}

#[derive(Insertable)]
#[table_name="users"]
pub struct NewUser <'a>{
    pub username:&'a str,
    pub email:&'a str
}

use super::schema::{person,gender};
#[derive(Identifiable,Queryable,PartialEq,Debug)]
#[table_name="person"]
pub struct Person{
    pub id:i32,
    pub username:String,
    pub email:String

}

#[derive(Insertable)]
#[table_name="person"]
pub struct NewPerson <'a>{
    pub username:&'a str,
    pub email:&'a str
}


#[derive(Identifiable,Queryable,Associations,PartialEq,Debug)]

#[belongs_to(Person)]
#[table_name="gender"]
pub struct Gender {
    pub id:i32,
    pub title:String,
    pub person_id:Option<i32>

}

#[derive(Insertable)]
#[table_name="gender"]
pub struct NewGender <'a>{
    pub title:&'a str,
 
    pub person_id:i32
}
#[derive(AsChangeset)]
#[table_name="gender"]
pub struct ChangeGender{
    pub title:Option<String>,
    pub person_id:Option<i32>
}
