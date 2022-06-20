use std::collections::HashMap;

use actix_web::{web::Data, HttpResponse};
use chrono::DateTime;
use tera::{Context, Error as TeraError, Tera, Value};

use crate::error::AppError;

pub type TemplateResult = Result<HttpResponse, AppError>;

pub fn render(tpl: Data<Tera>, name: &str, ctx: &Context) -> TemplateResult {
    let s = tpl.render(name, ctx)?;
    Ok(HttpResponse::Ok().content_type("text/html").body(s))
}

fn dt_filter(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    let d = DateTime::parse_from_rfc3339(value.as_str().ok_or_else(|| TeraError::msg("filter_error"))?)
        .map_err(|_| TeraError::msg("filter_error"))?;
    Ok(Value::String(d.format("%Y-%m-%d %H:%M:%S").to_string()))
}

pub fn init_tera() -> crate::Result<Tera> {
    let mut tera = Tera::new("templates/**/*.html")?;
    tera.register_filter("dt", dt_filter);
    Ok(tera)
}
