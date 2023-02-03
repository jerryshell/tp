#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct User {
    pub id: u32,
    pub create_at: u32,
    pub update_at: u32,
    pub remark: String,
}
