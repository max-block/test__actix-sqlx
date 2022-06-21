use std::collections::HashMap;

use actix_web::HttpResponse;
use chrono::DateTime;
use tera::{Context, Error as TeraError, Tera, Value};

use crate::error::AppError;

pub type TemplateResult = Result<HttpResponse, AppError>;

fn dt_filter(value: &Value, _: &HashMap<String, Value>) -> tera::Result<Value> {
    println!("dt_filter: {}", value);
    if value.is_string() {
        let d = DateTime::parse_from_rfc3339(value.as_str().ok_or_else(|| TeraError::msg("filter_error"))?)
            .map_err(|_| TeraError::msg("filter_error"))?;
        dbg!(&d);
        return Ok(Value::String(d.format("%Y-%m-%d %H:%M:%S").to_string()))
    }
    Ok(Value::String("".to_string()))
}

pub struct Template {
    tera: Tera,
    ctx: Context,
}

impl Template {
    pub fn new() -> crate::Result<Self> {
        let mut tera = Tera::new("templates/**/*.html")?;
        let mut ctx = Context::new();
        ctx.insert("app_name", "xxx");
        ctx.insert("app_version", "123");
        tera.register_filter("dt", dt_filter);
        Ok(Template { tera, ctx })
    }

    pub fn render(&self, name: &str) -> TemplateResult {
        let s = self.tera.render(name, &self.ctx)?;
        Ok(HttpResponse::Ok().content_type("text/html").body(s))
    }

    pub fn render_with_ctx(&self, name: &str, mut ctx: Context) -> TemplateResult {
        ctx.extend(self.ctx.clone());
        let s = self.tera.render(name, &ctx)?;
        Ok(HttpResponse::Ok().content_type("text/html").body(s))
    }
}
