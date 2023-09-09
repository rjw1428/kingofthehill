use crate::{models::pick::*, repository::mongo_repo::MongoRepo};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State, response::status::NoContent};

#[options("/v1/pick")]
pub fn post_pick_preflight() -> NoContent { 
     NoContent
}

#[post("/v1/pick", data = "<new_pick>")]
pub fn create_pick(
    db: &State<MongoRepo>,
    new_pick: Json<Pick>,
) -> Result<Json<InsertOneResult>, Status> {
    println!("{:?}", new_pick);
    let data = Pick {
        id: None,
        user_id: new_pick.user_id.to_owned(),
        team_id: new_pick.team_id.to_owned(),
        league_id: new_pick.league_id.to_owned(),
        week_id: new_pick.week_id.to_owned()
    };
    let team_details = db.create_pick(data);
    match team_details {
        Ok(team) => Ok(Json(team)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/v1/picks")]
pub fn get_all_picks(db: &State<MongoRepo>) -> Result<Json<Vec<Pick>>, Status> {
    let pick_details = db.get_all_picks();
    match pick_details {
        Ok(picks) => Ok(Json(picks)),
        Err(_) => Err(Status::InternalServerError),
    }
}