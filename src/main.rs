#[macro_use] extern crate rocket;

#[get("/")]
fn index() -> &'static str {
    "Hello World"
}

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![index])
}
