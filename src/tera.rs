use rocket::response::Redirect;
use rocket::Request;

use rocket_dyn_templates::{context, tera::Tera, Template};

#[get("/")]
pub fn index() -> Redirect {
    Redirect::to(uri!("/tera", hello(name = "Your Name")))
}

#[get("/hello/<name>")]
pub fn hello(name: &str) -> Template {
    Template::render(
        "tera/index",
        context! {
            title: "Hello",
            name: Some(name),
            items: vec!["One", "Two", "Three"],
        },
    )
}

#[get("/about")]
pub fn about() -> Template {
    Template::render(
        "tera/about.html",
        context! {
            title: "About",
        },
    )
}

#[catch(404)]
pub fn not_found(req: &Request<'_>) -> Template {
    Template::render(
        "tera/error/404",
        context! {
            uri: req.uri()
        },
    )
}

#[allow(dead_code)]
pub fn customize(tera: &mut Tera) {
    tera.add_raw_template(
        "tera/about.html",
        r#"
        {% extends "tera/base" %}

        {% block content %}
            <section id="about">
              <h1>About - Here's another page!</h1>
            </section>
        {% endblock content %}
    "#,
    )
    .expect("valid Tera template");
}
