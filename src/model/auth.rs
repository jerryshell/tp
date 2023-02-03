#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RegisterForm {
    pub email: String,
    pub password: String,
}

#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Auth {
    pub id: u32,
    pub create_at: u32,
    pub update_at: u32,
    pub remark: String,
    pub user_id: u32,
    pub email: String,
    pub password: String,
}
