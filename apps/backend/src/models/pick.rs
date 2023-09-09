use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../libs/shared/pick.d.ts", rename_all = "camelCase")]
#[serde( rename_all = "camelCase")]
pub struct Pick {
    #[serde( rename = "_id", skip_serializing_if = "Option::is_none")]
    #[ts(type = "{$oid: string}", optional, rename = "_id")]
    pub id: Option<ObjectId>,
    pub user_id: String,
    pub team_id: String,
    pub week_id: String,
    pub league_id: String,
}