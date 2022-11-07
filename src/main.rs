#[macro_use]
extern crate rocket;

use rocket::{Build, Rocket};
use rocket_dyn_templates::{context, Template};

#[get("/")]
fn index() -> Template {
	Template::render("index", context! {
		title: "Title",
	})
}

#[launch]
fn rocket() -> Rocket<Build> {
	let figment = rocket::Config::figment().merge(("address", "0.0.0.0"));

	rocket::custom(figment)
		.mount("/", routes![index])
		.attach(Template::fairing())
}
