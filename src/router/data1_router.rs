use actix_web::{get, post, web, Scope};

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

pub fn data1_router() -> Scope {
    web::scope("/api/data1").service(get_data1_list).service(create_data1)
}
