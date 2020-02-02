use actix_web::{get,post,delete,put, HttpResponse, ResponseError, Error, web};
use serde_json::json;
use crate::user::model::{Newuser, User};



#[get("/users")]
async fn find_all() -> Result<HttpResponse, Error> {
    let users = match User::find_all() {
        Ok(users) => users,
        Err(_) => {vec![]}
    };

    Ok(HttpResponse::Ok().json(users))
}

#[post("/users")]
async fn create(param: web::Json<Newuser>) -> Result<HttpResponse, Error> {
    let new_user = Newuser {
        name: param.name.clone(),
        email: param.email.clone(),
    };

    let is_created =  User::create(new_user).is_ok();
    Ok(HttpResponse::Ok().json(json!({"created": is_created})))
}

#[delete("/users/{id}")]
async fn delete(param: web::Path<(i32)>) -> Result<HttpResponse, Error> {
    User::delete(param.into_inner());
    Ok(HttpResponse::Ok().json(json!({"message": "user deleted".to_string()})))
}


pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(find_all);
    cfg.service(create);
    cfg.service(delete);
}