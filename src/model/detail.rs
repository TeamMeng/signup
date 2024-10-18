use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct ProfileDetails {
    pub name: String,
    pub email: String,
}
