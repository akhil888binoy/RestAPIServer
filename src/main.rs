use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use actix_web::web::route;
use serde::{Deserialize, Serialize};
use actix_web::{web, App, HttpResponse, HttpServer};


#[derive(Debug, Serialize, Deserialize, Clone)]
struct User{
    id :  String,
    name : String,
    email: String
}


#[derive(Clone)]
struct AppState{
   users: Arc<Mutex<HashMap<String,User>>>
}

impl AppState{
    fn new()->Self{
       return AppState{
            users: Arc::new(Mutex::new(HashMap::new())),
        };
    }
}

async fn getallusers(state:web::Data<AppState>)->HttpResponse{
    let users = state.users.lock().unwrap();
    let allusers : Vec<User>= users.values().cloned().collect();
    HttpResponse::Ok().json(allusers)
}

async fn getauser(state: web::Data<AppState> ,userid : web::Path<String>)->HttpResponse{
    let users = state.users.lock().unwrap();
    if let Some(user) = users.get(&userid.into_inner()){
        HttpResponse::Ok().json(user)
    }else{
        HttpResponse::NotFound().body("User not found")
    }
}

async fn createuser(state : web::Data<AppState> , newuser : web::Json<User>)->HttpResponse{
    let mut users = state.users.lock().unwrap();
    let user = newuser.into_inner();
    users.insert(user.id.clone() , user.clone());
    HttpResponse::Created().json(user)
}

async fn updateuser(state:web::Data<AppState> , userid:web::Path<String> , userinfo:web::Json<User>)->HttpResponse{
    let mut users = state.users.lock().unwrap();
    let id = userid.into_inner();
    if users.contains_key(&id){
        let user = userinfo.into_inner();
        users.insert(id.clone(), user.clone());
        HttpResponse::Ok().json(user)
    }else{
        HttpResponse::NotFound().json("User not found")
    }
}


async fn deleteuser(state : web::Data<AppState> , id:web::Path<String>)->HttpResponse{
    let mut users = state.users.lock().unwrap();
    let id = id.into_inner();
    if users.remove(&id).is_some(){
        HttpResponse::Ok().body("User deleted")
    }else{
        HttpResponse::NotFound().body("User not found")
    }
}


#[actix_web::main]
 async fn main()->std::io::Result<()> {
    let state = AppState::new();
    HttpServer::new(move || {
        App::new()
                .app_data(web::Data::new(state.clone()))
                .route("/users" , web::get().to(getallusers))
                .route("/users/{id}" , web::get().to(getauser))
                .route("/users", web::post().to(createuser))
                .route("/users/{id}" , web::put().to(updateuser))
                .route("/users/{id}" , web::delete().to(deleteuser))        
    })
    .bind("127.0.0.1:8000")?
    .run()
    .await
 }