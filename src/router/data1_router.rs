use actix_web::{delete, get, post, web, Scope};

use crate::{
    app::App,
    db::InsertData1Params,
    util::{json_result, JsonResult},
};

#[get("")]
async fn get_data1_list(app: web::Data<App>) -> JsonResult {
    json_result(app.db.get_all_data1().await?)
}

#[post("")]
async fn create_data1(app: web::Data<App>, params: web::Json<InsertData1Params>) -> JsonResult {
    json_result(app.db.insert_data1(params.0).await?)
}

#[get("/{id}")]
async fn get_data1(app: web::Data<App>, id: web::Path<i32>) -> JsonResult {
    json_result(app.db.get_data1(*id).await?)
}

#[delete("/{id}")]
async fn delete_data1(app: web::Data<App>, id: web::Path<i32>) -> JsonResult {
    json_result(app.db.delete_data1(*id).await?)
}

#[post("/{id}/process")]
async fn process_data1(app: web::Data<App>, id: web::Path<i32>) -> JsonResult {
    json_result(app.data1_service.process(*id).await?)
}

pub fn data1_router() -> Scope {
    web::scope("/api/data1")
        .service(get_data1_list)
        .service(create_data1)
        .service(get_data1)
        .service(delete_data1)
        .service(process_data1)
}
