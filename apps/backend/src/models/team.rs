use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../libs/shared/team.d.ts")]
pub struct Team {
    #[serde(rename = "_id", skip_serializing_if = "Option::is_none")]
    #[ts(type = "{$oid: string}", optional, rename = "_id")]
    pub id: Option<ObjectId>,
    pub name: String,
    pub city: String,
}