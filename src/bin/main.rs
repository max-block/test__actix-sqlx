#[actix_web::main]
async fn main() -> std::io::Result<()> {
    test_actix_sqlx::run().await
}
