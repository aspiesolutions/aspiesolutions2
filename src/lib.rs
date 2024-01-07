

#[cfg(feature="db")]
pub mod db;



#[cfg(feature="rocket")]
pub async fn build_rocket() -> Result<rocket::Rocket<rocket::Build>,anyhow::Error> {
    let mut rocket =rocket::Rocket::build();
    // configure the web server.

    /*  a database is required for this web app to function correctly.
        if the database is unavailable or not enabled via feature flags then
        any database related features will be unavailable
    */
    #[cfg(feature="db")] {
        println!("Put database initialization code here!")
    }


    /*
    if templates are disabled at compile time, no template based routes
    will be available!
    */ 
    #[cfg(feature="rocket_dyn_templates")] {
        use rocket_dyn_templates::Template;
        rocket = rocket.attach(Template::fairing()).mount("/", rocket::routes![render_landing_page])
    }
    #[cfg(not(feature="rocket_dyn_templates"))] {
        eprintln!("rocket_dyn_templates was disabled at compile time!. template routes will not be available")
    }
    Ok(rocket)
}
#[cfg(all(feature="rocket",feature="rocket_dyn_templates"))]
#[rocket::get("/")]
pub fn render_landing_page() -> rocket_dyn_templates::Template {
    use rocket_dyn_templates::{Template,context};
    Template::render("landing_page", context![])

}