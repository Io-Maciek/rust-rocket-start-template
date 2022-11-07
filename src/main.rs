#[macro_use]
extern crate rocket;

#[macro_use]
extern crate rocket_include_static_resources;

use rocket::{Build, Rocket, serde::Serialize};
use rocket_dyn_templates::{context, Template};


#[derive(Serialize)]
struct User{
	username: &'static str,
}

#[get("/")]
fn index() -> Template {
	Template::render("index", context! {
		title: "Title",
		user: "User",
	})
}

#[launch]
fn rocket() -> Rocket<Build> {
	let figment = rocket::Config::figment().merge(("address", "0.0.0.0"));

	rocket::custom(figment)
		.attach(static_resources_initializer!(
			"favicon" => "img/favicon.png",
		))
		.mount("/", routes![favicon, index])
		.attach(Template::fairing())
}

static_response_handler! {
    "/favicon.png" => favicon => "favicon",
}