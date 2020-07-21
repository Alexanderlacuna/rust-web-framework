
use database_demo::*;
use actix_web::{Error,HttpRequest,web,App,HttpResponse,HttpServer,Responder,get,Result,error};
use serde::{Deserialize,Serialize};
use futures::future::{BoxFuture,FutureExt};

use  listenfd::ListenFd;
use std::sync::Mutex;
use futures::future::{ready,Ready};
mod connecting;


// use diesel::sqlite::SqliteConnection;
#[derive(Deserialize)]
struct info{
    username:String,
    email:String
}

// #[derive(Serialize)]
trait Test{

}
// #[derive(Serialize)]
pub struct Container{
     username:Vec<People>,
   
}

#[derive(Serialize)]
pub struct Item{
    pub items:Vec<People>
    

}
impl  Responder for Item{
    type Error=Error;
    type Future=Ready<Result<HttpResponse,Error>>;
    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        // let body=serde_json::to_string(&self.).unwrap();
        let body=serde_json::to_string(&self.items).unwrap();
        // create response type and set content-type
        ready(Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
    ))
        
    }
}


#[derive(Serialize)]
pub struct People{
    pub username:String,
    pub email:String,
    pub id:i32

}
impl  Responder for People{
    type Error=Error;
    type Future=Ready<Result<HttpResponse,Error>>;
    fn respond_to(self, req: &HttpRequest) -> Self::Future {
        let body=serde_json::to_string(&self).unwrap();
        // create response type and set content-type
        ready(Ok(HttpResponse::Ok()
        .content_type("application/json")
        .body(body)
    ))
        
    }
}


async fn get_all_per()->impl Responder{
    use connecting::get_all_person as call;
    let users=call();
    let mut package=Vec::new();
   
    for user in users{
      
        let new_user=People{
            username:user.username,
            email:user.email,
            id:user.id
        };
        package.push(new_user);
       

        
        };

        let item_holder=Item{
            items:package
        };
 


        item_holder

       

        // People {
        //     username:String::from("DSfs"),
        //     email:"ASdsdf".to_string(),
        //     id:4
        // }
        // package.push(new_user)
    }

    
    

async fn get_genders()->Result<String>{
    use connecting::query_person_genders;

    query_person_genders();

    println!("hello there");
    Ok(String::from("successfully found all"))
}

async fn update_user()->Result<String>{
    use users::update_user_controller;
    update_user_controller(1);
    Ok(format!("success occured"))



}

async fn create_person()->Result<String>{
    use connecting::create_person as creating;
    creating();

    Ok(String::from("it was a success"))
}

async fn create_gender()->Result<String>{
    use connecting::create_gender as gender;
    gender();
    Ok(String::from("successfully created a gender"))
}


// async fn find_user(userId:web::Query<Info2>)->Result<String>{

//     use users::find_user_controller;
//     let user_results=find_user_controller(userId.id);
//     match user_results {
//         Some(val)=>{
//             Ok(format!("user with id {} and username {} and email {}",val.id,val.username,val.email))
//         }
//         None=>{

//             // Error.as_error("dasd")
//             // error::ErrorNotFound("not found")
//             Ok(format!("no user found"))
//         }
//     }
  
// }
// async fn delete_user(userId:web::Query<Info2>)->Result<String>{

//     use users::delete_user_controller;
//     delete_user_controller(userId.id);
//     Ok(format!("deleted user"))
// }

async fn register_user(info:web::Json<info>)->Result<String>{
    use users::create_users;
    // let conns=establish_connection();
    create_users(&info.username,&info.email);
    Ok(format!("worked"))

}

async fn display_users()->Result<String>{
    use users::show_users;
    show_users();

    Ok(format!("displayed all users"))
}

async fn index()->String{
    format!("Hello there")
}

#[actix_rt::main]
async fn main()->std::io::Result<()>{
  
    let mut listenfd=ListenFd::from_env();
    let mut server=HttpServer::new(move|| {
        App::new()

          
       .route("/index", web::get().to(index))
       .route("/register",web::post().to(register_user))
       .route("/display", web::get().to(display_users))
    //    .route("/delete", web::get().to(delete_user))

    //    .route("/getUser",web::get().to(find_user))
       .route("/update", web::get().to(update_user))

       .route("/createPerson", web::get().to(create_person))
       .route("/createGender", web::get().to(create_gender))

       .route("/queryAll", web::get().to(get_genders))

       .route("/AllPerson",web::get().to(get_all_per))
            

          

    });
    server=if let Some(l)=listenfd.take_tcp_listener(0).unwrap(){
        server.listen(l)?
    }else{
        server.bind("127.0.0.1:3000")?
    };

    server.run().await
    // .bind("127.0.0.1:8081")?
    // .run()
    // .await

}