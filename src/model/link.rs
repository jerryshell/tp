#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateForm {
    pub id: String,
    pub target_link: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTargetLinkForm {
    pub id: String,
    pub target_link: String,
}

#[derive(serde::Serialize, serde::Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateIdForm {
    pub id: String,
    pub new_id: String,
}

#[derive(serde::Serialize, serde::Deserialize, sqlx::FromRow)]
#[serde(rename_all = "camelCase")]
pub struct Link {
    pub id: String,
    pub create_at: u32,
    pub update_at: u32,
    pub remark: String,
    pub user_id: u32,
    pub target_link: String,
    pub visits_count: u32,
}
