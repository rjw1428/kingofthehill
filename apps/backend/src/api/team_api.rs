use crate::{models::team::Team, repository::mongo_repo::MongoRepo};
use mongodb::results::InsertOneResult;
use rocket::{http::Status, serde::json::Json, State, response::status::NoContent};

#[options("/v1/team")]
pub fn post_team_preflight() -> NoContent { 
     NoContent
}

#[post("/v1/team", data = "<new_team>")]
pub fn create_team(
    db: &State<MongoRepo>,
    new_team: Json<Team>,
) -> Result<Json<InsertOneResult>, Status> {
    let data = Team {
        id: None,
        name: new_team.name.to_owned(),
        city: new_team.city.to_owned(),
    };
    let team_details = db.create_team(data);
    match team_details {
        Ok(team) => Ok(Json(team)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/v1/teams")]
pub fn get_all_teams(db: &State<MongoRepo>) -> Result<Json<Vec<Team>>, Status> {
    let teams_detail = db.get_all_teams();
    match teams_detail {
        Ok(teams) => Ok(Json(teams)),
        Err(_) => Err(Status::InternalServerError),
    }
}

#[get("/v1/team/<path>")]
pub fn get_team(db: &State<MongoRepo>, path: String) -> Result<Json<Team>, Status> {
    let id = path;
    if id.is_empty() {
        return Err(Status::BadRequest);
    };
    let team_detail = db.get_team(&id);
    match team_detail {
        Ok(team) => Ok(Json(team)),
        Err(_) => Err(Status::InternalServerError),
    }
}

