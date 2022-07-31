#[derive(Eq, Debug, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct BaseResponse {
    #[serde(rename(deserialize = "Count"))]
    pub count: u32,
}
