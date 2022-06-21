use actix_web::{get, web, Scope};
use tera::Context;

use crate::{
    app::App,
    template::{Template, TemplateResult},
};

#[get("/")]
async fn index_page(tpl: web::Data<Template>) -> TemplateResult {
    tpl.render("index.html")
}

#[get("/data1")]
async fn data1_page(tpl: web::Data<Template>, app: web::Data<App>) -> TemplateResult {
    let mut ctx = Context::new();
    let data1_list = app.db.get_all_data1().await?;
    ctx.insert("data1_list", &data1_list);
    tpl.render_with_ctx("data1.html", ctx)
}

pub fn ui_router() -> Scope {
    web::scope("").service(index_page).service(data1_page)
}
