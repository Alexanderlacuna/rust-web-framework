use crate::models::{NewPerson,NewGender};

extern crate diesel;

use crate::establish_connection;


use diesel::sqlite::SqliteConnection;


pub fn create_person(){
    use crate::schema::person::dsl::*;
    use crate::schema::gender::dsl::*;


    let connection=establish_connection();
    let f=NewPerson{

        username:"this is a test",
        email:"this is a test"

    };

    use diesel::*;

    diesel::insert_into(person).values(&f).execute(&connection).expect("error while tryinh
    to save");
    // diesel::insert_into(person).values(&f)
    // .execute(&connection).expect("Error while trying to save");
}

pub fn create_gender(){
    use crate::schema::person::dsl::*;
    use crate::schema::gender::dsl::*;
    let connection=establish_connection();

    use diesel::*;
    let r=NewGender{
        title:"thi is  title",
        person_id:1

    };

    diesel::insert_into(gender).values(&r).execute(&connection).expect("error trying to save
    ");
}



pub fn get_all_person()->Vec<database_demo::models::Person>{
    use diesel::*;
    let connection=establish_connection();
    use crate::schema::person::dsl::*;
    use crate::models::Person;
    let people=person.limit(5).load::<Person>(&connection).expect("Error loading post");
    people
}
pub fn query_person_genders(){
    use diesel::*;
    use crate::schema::person::dsl::*;
    use crate::schema::gender::dsl::*;
    let connection=establish_connection();

    use crate::models::{Gender,Person};
    // let person_1=person.find(2).get_result::<Person>(&connection).expect(
    //     "an error occured"
    // );

    println!("running the gold function");

    let person_1=person.find(1).get_result::<Person>(&connection).expect(
        "an error occured"
    );

    let person_2=Gender::belonging_to(&person_1).load::<Gender>(&connection).expect(
        "an error occurred"
    );

    println!("the person2 is {:?}",person_2);

    for p in person_2 {
        println!("{:?} {:?}",p.id,p.title);
    }

}

pub fn delete_gender(){
    // use crate::schema::person::dsl::*;
    use crate::schema::gender::dsl::*;
    use crate::schema::gender::dsl::id as re;
    // use crate::schema::gender::dsl::{self,id as e}
    use diesel::*;
    let connection=establish_connection();
    use crate::models::{Gender,Person};
    let delete_id=3;
    

    let results1=diesel::delete(gender.filter(re.eq(1))).execute(&connection).expect("failed to delete use");
    // let results=  diesel::delete(person.filter(id.eq(1))).execute(&connection).expect("Error occurred") ;
    // println!("user with id {} deleted",results);
   
   
}


pub fn update_gender(titl:String){
    use crate::schema::gender::dsl::*;
    use diesel::*;

    use crate::models::{Gender,NewGender,ChangeGender};
    let connection=establish_connection();

    diesel::update(gender.filter(id.eq(1))).set(&ChangeGender{
        person_id:None,
        title:Some(titl)
    }).execute(&connection).expect("an error occurred");

    
    

 

    }

  
   



