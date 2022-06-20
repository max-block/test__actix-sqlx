use actix_web::{get, web, Scope};
use tera::{Context, Tera};

use crate::{template::{render, TemplateResult}, app::App};

#[get("/")]
async fn index_page(tpl: web::Data<Tera>) -> TemplateResult {
    render(tpl, "index.html", &Context::new())
}

#[get("/data1")]
async fn data1_page(tpl: web::Data<Tera>, app: web::Data<App>) -> TemplateResult {
    let mut ctx = Context::new();
    let data1_list = app.db.get_all_data1().await?;
    ctx.insert("data1_list", &data1_list);
    render(tpl, "data1.html", &ctx)
}


pub fn ui_router() -> Scope {
    web::scope("").service(index_page).service(data1_page)
}
