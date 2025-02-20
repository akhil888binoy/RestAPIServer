use std::collections::HashMap;
use std::sync::{Arc, Mutex};

struct User{
    id :  String,
    name : String,
    email: String
}

struct AppState{
   users: Arc<Mutex<HashMap<String,User>>>
}

impl AppState{
    fn new(&mut self){
        self.users = Arc::new(Mutex::new(HashMap::new()));
    }
}

 fn main(){

 }