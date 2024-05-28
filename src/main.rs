#[macro_use]
extern crate rocket;

use std::fs;
use std::env;

#[get("/")]
fn hello() -> &'static str {
    return "Hello, world!"
}

#[get("/<nth>")]
fn fibonnaci(nth: u8) -> String {
    let mut sum = 0;
    let mut last = 0;
    let mut curr = 1;
    for _i in 1..nth {
        sum = last + curr;
        last = curr;
        curr = sum;
    }

    return format!("The {}nth fibonnaci: {}", nth, sum);
}

#[get("/")]
fn write() {
    let data = "Some data!";
    fs::write("/tmp/foo", data).expect("Unable to write file");
}

#[get("/")]
fn environment() -> String {
    let user = env::var("USER").unwrap_or_default();
    let password = env::var("PASSWORD").unwrap_or_default();
    return format!("The {} user has the {} password", user, password);
}

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello])
        .mount("/fib", routes![fibonnaci])
        .mount("/write", routes![write])
        .mount("/env", routes![environment])
}
