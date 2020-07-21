// extern crate database_demo;
use super::*;
extern crate diesel;
// use self::database_demo::*;
use self::models::*;
use self::diesel::prelude::*;
// use std::io::{stdin,Read};

pub fn update_user_controller(isd:i32){
    use schema::users::dsl::*;
    let connection=establish_connection();
    let target=users.filter(id.eq(isd));
    diesel::update(target).set(username.eq("new username")).execute(&connection).expect("erro occured");

}


pub fn  find_user_controller(isd:i32)->Option<User>{

    use schema::users::dsl::*;
    let connection=establish_connection();
   let user_found=users.filter(id.eq(isd)).limit(1).load::<User>(&connection).expect("Erro loading post");

   let found_user=&user_found.get(0);



   match found_user {
       Some(user)=>{

        let new_user= User {
            id:user.id,
            username:user.username.clone(),
            email:user.email.clone(),
            
        };
     
    Some(new_user)

       }
       None=>{

        None
          
           
       }
   }
 

  
}
pub fn show_users(){
    use schema::users::dsl::*;
    let connection=establish_connection();
    let results=users.limit(5).load::<User>(&connection)
    .expect("Error loading users");
    use schema::person::dsl::*;
    use schema::gender::dsl::*;
    let connection=establish_connection();
    let person_1=person.find(2).get_result::<Person>(&connection).expect(
        "an error occured"
    );

    let person_2=Gender::belonging_to(&person_1).load::<Gender>(&connection).expect(
        "an error occurred"
    );
    let person_3:Gender=Gender::belonging_to(&person_1).first(&connection).expect("an error occurred");

    println!("the value of first is {:?}",person_1);

    println!("the found is {:?}",person_3);
    println!("Displaying users  {}",results.len());
    for user in results {
        println!("{}",user.username);
        println!("{}",user.email);
        println!("{}",user.id);
    }

    // results.clone(=]
}

pub fn create_users(username:&String,email:&String){
    let connection=establish_connection();
    // println!("write your username");
    // let mut username=String::new();
    // stdin().read_line(&mut username).unwrap();
    // let title = &username[..(username.len() - 1)]; 
    // println!("\nOk! Let's write {} (Press {} when finished)\n", title, EOF);
    // let mut email = String::new();
    // stdin().read_to_string(&mut email).unwrap();

    let new_user=create_user(&connection, &username, &email);


}

pub fn delete_user_controller(isd:i32){
    use schema::users::dsl::*;
    let connection=establish_connection();

    // diesel::delete(source: T)
 let results=  diesel::delete(users.filter(id.eq(isd))).execute(&connection).expect("Error occurred") ;
 println!("user with id {} deleted",results);



}
pub fn create_person_controller(){

    use schema::person::dsl::*;
    use schema::gender::dsl::*;
    let connection=establish_connection();
    let person_1=person.find(2).get_result::<Person>(&connection).expect(
        "an error occured"
    );

    let person_1=Gender::belonging_to(&person_1).execute(&connection).expect(
        "an error occurred"
    );

    println!("the value of first is {:?}",person_1);
    // let person_2=Gender::belonging_to(&person_1).first(&connection).expect("an error occurred");

    // let persons_genders=Gender::belonging_to(&person_1).first(&connection);
    

  
}

