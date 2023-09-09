use mongodb::bson::oid::ObjectId;
use serde::{Serialize, Deserialize};
use ts_rs::TS;

#[derive(Debug, Serialize, Deserialize, TS)]
#[ts(export, export_to = "../../libs/shared/response.d.ts", rename_all = "camelCase")]
pub struct Response {
    #[ts(type = "{$oid: string}")]
    pub inserted_id: Option<ObjectId>,
}