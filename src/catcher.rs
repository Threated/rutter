use rocket::{Catcher, http::Cookie, Request};


#[catch(401)]
fn unauthorized(req: &Request) -> () {
    req.cookies().add(Cookie::build("isAuthenticated", "0").finish())
}

pub fn catchers() -> Vec<Catcher> {
    catchers![unauthorized]
}
