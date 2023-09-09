mod repository;
pub mod models;
pub mod api;

#[macro_use]
extern crate rocket;
use api::team_api::*;
use api::pick_api::*;
use repository::mongo_repo::MongoRepo;
use rocket::http::Header;
use rocket::{Request, Response};
use rocket::fairing::{Fairing, Info, Kind};

pub struct Cors;

#[rocket::async_trait]
impl Fairing for Cors {
    fn info(&self) -> Info {
        Info {
            name: "Add CORS headers to responses",
            kind: Kind::Response,
        }
    }

    async fn on_response<'r>(&self, _request: &'r Request<'_>, response: &mut Response<'r>) {
        response.set_header(Header::new("Access-Control-Allow-Origin", "*"));
        response.set_header(Header::new(
            "Access-Control-Allow-Methods",
            "POST, PATCH, PUT, DELETE, HEAD, OPTIONS, GET",
        ));
        response.set_header(Header::new("Access-Control-Allow-Headers", "*"));
        // response.set_header(Header::new("Access-Control-Allow-Credentials", "true"));
    }
}



#[launch]
fn rocket() -> _ {
    let db = MongoRepo::init();
    rocket::build().manage(db)
        .attach(Cors)
        .mount("/api", routes![post_team_preflight])
        .mount("/api", routes![create_team])
        .mount("/api", routes![get_team])
        .mount("/api", routes![get_all_teams])
        .mount("/api", routes![post_pick_preflight])
        .mount("/api", routes![create_pick])
        .mount("/api", routes![get_all_picks])

}