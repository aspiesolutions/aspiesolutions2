#[cfg(feature="rocket")]
#[rocket::main]
pub async fn main() -> Result<(),Box<dyn std::error::Error>> {

    aspiesolutions::build_rocket().await?.launch().await?;
    Ok(())
}