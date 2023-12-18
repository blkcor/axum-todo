use deadpool_postgres::Client;

use crate::model::Admin;
use crate::Result;

pub async fn find(client: &Client, email: &str) -> Result<Admin> {
    super::query_row(
        client,
        "SELECT * FROM admins WHERE email = $1 AND is_del=false",
        &[&email],
    )
    .await
}
