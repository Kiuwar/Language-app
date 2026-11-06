use crate::db::DbConnection;

pub struct AppState {
    pub db: DbConnection,
}
