#[macro_use] extern crate rocket;
use rocket::{tokio::fs, response::content::RawHtml};

#[get("/")]
fn index() -> &'static str {
    "Hello World"
}

#[get("/hello")]
async fn say_hello() -> RawHtml<String> {
    let file = fs::read_to_string("./src/templates/index.liquid").await.unwrap();
    let template = liquid::ParserBuilder::with_stdlib().build().unwrap().parse(&file).unwrap();
    let vals = liquid::object!({"name": "leo"});
    let output =template.render(&vals).unwrap();
    RawHtml(output)
}

#[launch]
async fn rocket() -> _ {
    rocket::build().mount("/", routes![index, say_hello])
}
