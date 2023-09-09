// use std::env;
// extern crate dotenv;
// use dotenv::dotenv;
use mongodb::{
    bson::{extjson::de::Error, oid::ObjectId, doc},
    results::InsertOneResult,
    sync::{Client, Collection},
};
use crate::models::team::*;
use crate::models::pick::*;

pub struct MongoRepo {
    teams_col: Collection<Team>,
    picks_col: Collection<Pick>,
}

impl MongoRepo {
    pub fn init() -> Self {
        // dotenv().ok();
        // let uri = match env::var("MONGOURI") {
        //     Ok(v) => v.to_string(),
        //     Err(_) => format!("Error loading env variable"),
        // };
        let uri = String::from("mongodb://localhost:27017");
        let client = Client::with_uri_str(uri).unwrap();
        let db = client.database("kingofthehill");
        let teams_col: Collection<Team> = db.collection("teams");
        let picks_col: Collection<Pick> = db.collection("picks");
        MongoRepo { teams_col, picks_col }
    }

    pub fn create_team(&self, new_team: Team) -> Result<InsertOneResult, Error> {
        let new_doc = Team {
            id: None,
            name: new_team.name,
            city: new_team.city,
        };
        let team = self
            .teams_col
            .insert_one(new_doc, None)
            .ok()
            .expect("Error creating team");
        Ok(team)
    }

    pub fn get_team(&self, id: &String) -> Result<Team, Error> {
        let obj_id = ObjectId::parse_str(id).unwrap();
        let filter = doc! {"_id": obj_id};
        let team_detail = self
            .teams_col
            .find_one(filter, None)
            .ok()
            .expect("Error getting user's detail");
        Ok(team_detail.unwrap())
    }

    pub fn get_all_teams(&self) -> Result<Vec<Team>, Error> {
        let cursors = self
            .teams_col
            .find(None, None)
            .ok()
            .expect("Error getting list of teams");
        let teams = cursors
            .map(|doc| doc.unwrap())
            .collect();
        Ok(teams)
    }

    pub fn create_pick(&self, data: Pick) -> Result<InsertOneResult, Error> {
        let new_doc = Pick {
            ..data
        };
        // println!("{:?}", new_doc);
        // let serialized_data = bson::to_bson(&new_doc).unwrap();
        // println!("{:?}", serialized_data);
        // let document = serialized_data.as_document().unwrap();
        let pick = self
            .picks_col
            .insert_one(&new_doc, None)
            .ok()
            .expect("Error creating pick");
        Ok(pick)
    }

    pub fn get_all_picks(&self) -> Result<Vec<Pick>, Error> {
        let cursors = self
            .picks_col
            .find(None, None)
            .ok()
            .expect("Error getting list of picks");
        let picks = cursors
            .map(|doc| doc.unwrap())
            .collect();
        Ok(picks)
    }
}