#[macro_use]
extern crate rocket;

#[get("/")]
fn hello() -> &'static str {
    "Hello, world!"
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

#[launch]
fn rocket() -> _ {
    rocket::build()
        .mount("/", routes![hello])
        .mount("/fib", routes![fibonnaci])
}
