use auth_service;

#[rocket::main]
async fn main() -> Result<(), rocket::Error> {
    auth_service::rocket().launch().await;
    Ok(())
}
