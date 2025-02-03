use sqlx::{migrate::MigrateDatabase, sqlite::SqliteQueryResult, Sqlite, SqlitePool};

use crate::{MitnikaError, Project};

const TABLE_CREATEION_QUERY: &str = "PRAGMA foreign_keys = ON;

CREATE TABLE IF NOT EXISTS Project (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS File (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    project_id TEXT NOT NULL,
    creation_path TEXT NOT NULL,
    FOREIGN KEY (project_id) REFERENCES Project(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS Environment (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    file_id TEXT NOT NULL,
    FOREIGN KEY (file_id) REFERENCES File(id) ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS Version (
    id TEXT PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    environment_id TEXT NOT NULL,
    content TEXT NOT NULL,
    FOREIGN KEY (environment_id) REFERENCES Environment(id) ON DELETE CASCADE
);";

const PROJECT_CREATE_PROJECT_QUERY: &str = "INSERT INTO Project (id, name) VALUES (?, ?);";
const PROJECT_DELETE_PROJECT_QUERY: &str = "DELETE FROM Project WHERE id = ?;";
const PROJECT_UPDATE_PROJECT_QUERY: &str = "UPDATE Project SET name = ? WHERE id = ?;";
const PROJECT_FIND_PROJECT_LIKE: &str = "SELECT * FROM Project WHERE name LIKE CONCAT('%', ?, '%')";
const PROJECT_FIND_PROJECT_BY_ID: &str = "SELECT * FROM Project WHERE id = ?";
const PROJECT_FIND_PROJECT_LIKE_EXACT: &str = "SELECT * FROM Project WHERE name LIKE ?";
const PROJECT_FIND_ALL_PROJECT: &str = "SELECT * FROM Project";

const FILE_CREATE_FILE_QUERY: &str =
    "INSERT INTO File (id, name, project_id, creation_path) VALUES (?, ?, ?, ?);";
const FILE_DELETE_FILE_QUERY: &str = "DELETE FROM File WHERE id = ?;";
const FILE_UPDATE_FILE_NAME_QUERY: &str = "UPDATE File SET name = ? WHERE id = ?;";
const FILE_UPDATE_FILE_PATH_QUERY: &str = "UPDATE File SET creation_path = ? WHERE id = ?;";
const FILE_FIND_FILE_LIKE: &str = "SELECT * FROM File WHERE name LIKE CONCAT('%', ?, '%')";
const FILE_FIND_FILE_FOR_PROJECT: &str = "SELECT * FROM File WHERE name project_id = ?";

const ENV_CREATE_ENVIRONMENT_QUERY: &str =
    "INSERT INTO Environment (id, name, file_id) VALUES (?, ?, ?);";
const ENV_DELETE_ENVIRONMENT_QUERY: &str = "DELETE FROM Environment WHERE id = ?;";
const ENV_UPDATE_ENV_NAME_QUERY: &str = "UPDATE Environment SET name = ? WHERE id = ?;";
const ENV_FIND_ENV_LIKE: &str = "SELECT * FROM Environment WHERE name LIKE CONCAT('%', ?, '%')";
const ENV_FIND_ENV_FOR_FILE: &str = "SELECT * FROM File WHERE name file_id = ?";

const VERSION_CREATE_VERSION_QUERY: &str =
    "INSERT INTO Version (id, name, environment_id, content) VALUES (?, ?, ?, ?);";
const VERSION_DELETE_VERSION_QUERY: &str = "DELETE FROM Version WHERE id = ?;";
const VERSION_UPDATE_VERSION_NAME_QUERY: &str = "UPDATE Version SET name = ? WHERE id = ?;";
const VERSION_UPDATE_CONTENT_QUERY: &str = "UPDATE Version SET content = ? WHERE id = ?;";
const VERSION_FIND_VERSION_LIKE: &str = "SELECT * FROM Version WHERE name LIKE CONCAT('%', ?, '%')";
const VERSION_FIND_VERSION_FOR_ENV: &str = "SELECT * FROM Version WHERE environment_id = ?";

#[derive(Debug, Clone)]
pub(super) struct SQLiteDB {
    pool: SqlitePool,
}

impl SQLiteDB {
    pub fn new() -> Result<Self, MitnikaError> {
        let pool = tokio::runtime::Runtime::new()
            .unwrap()
            .block_on(Self::get_db_pool())
            .map_err(|err| MitnikaError::SQLiteDBError(err.to_string()))?;

        Ok(Self { pool })
    }

    pub(super) async fn get_db_pool() -> std::result::Result<SqlitePool, MitnikaError> {
        let dir_path = super::file_handler::get_dir()?;
        let db_url = format!(
            "sqlite:///{}",
            dir_path
                .to_str()
                .ok_or(MitnikaError::SQLiteFileNotCreated)?
        );
        if !Sqlite::database_exists(&db_url).await.unwrap_or(false) {
            Sqlite::create_database(&db_url).await.unwrap();
            match Self::create_schema(&db_url).await {
                Ok(_) => println!("Database created successfully."),
                Err(err) => return Err(MitnikaError::SQLiteDBError(err.to_string())),
            }
        }
        SqlitePool::connect(&db_url)
            .await
            .map_err(|err| MitnikaError::SQLiteDBError(err.to_string()))
    }

    async fn create_schema(db_url: &str) -> std::result::Result<SqliteQueryResult, MitnikaError> {
        let pool = SqlitePool::connect(db_url)
            .await
            .map_err(|err| MitnikaError::SQLiteDBError(err.to_string()))?;
        let result = sqlx::query(TABLE_CREATEION_QUERY).execute(&pool).await;
        pool.close().await;
        result.map_err(|err| MitnikaError::SQLiteDBError(err.to_string()))
    }

    pub async fn create_project(
        &self,
        name: &str,
    ) -> std::result::Result<Option<Project>, MitnikaError> {
        let id = uuid::Uuid::new_v4().to_string();
        let result = sqlx::query(PROJECT_CREATE_PROJECT_QUERY)
            .bind(&id)
            .bind(name)
            .execute(&self.pool)
            .await
            .map_err(|err| MitnikaError::SQLiteDBError(err.to_string()))?;

        println!("Added project: {result:?}");

        sqlx::query_as(PROJECT_FIND_PROJECT_BY_ID)
            .bind(&id)
            .fetch_optional(&self.pool)
            .await
            .map_err(|err| MitnikaError::SQLiteDBError(err.to_string()))
    }

    pub async fn _delete_project(
        &self,
        id: &str,
    ) -> std::result::Result<SqliteQueryResult, MitnikaError> {
        sqlx::query(PROJECT_DELETE_PROJECT_QUERY)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|err| MitnikaError::SQLiteDBError(err.to_string()))
    }

    pub async fn _update_project(
        &self,
        id: &str,
        name: &str,
    ) -> std::result::Result<SqliteQueryResult, MitnikaError> {
        sqlx::query(PROJECT_UPDATE_PROJECT_QUERY)
            .bind(name)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|err| MitnikaError::SQLiteDBError(err.to_string()))
    }

    pub async fn get_all_projects(&self) -> std::result::Result<Vec<Project>, MitnikaError> {
        sqlx::query_as(PROJECT_FIND_ALL_PROJECT)
            .fetch_all(&self.pool)
            .await
            .map_err(|err| MitnikaError::SQLiteDBError(err.to_string()))
    }

    pub async fn search_project(
        &self,
        search: &str,
        exact: bool,
    ) -> std::result::Result<Vec<Project>, MitnikaError> {
        let query = if exact {
            PROJECT_FIND_PROJECT_LIKE_EXACT
        } else {
            PROJECT_FIND_PROJECT_LIKE
        };
        sqlx::query_as(query)
            .bind(search)
            .fetch_all(&self.pool)
            .await
            .map_err(|err| MitnikaError::SQLiteDBError(err.to_string()))
    }

    pub async fn _create_file(
        &self,
        id: &str,
        name: &str,
        project_id: &str,
    ) -> std::result::Result<SqliteQueryResult, MitnikaError> {
        sqlx::query(FILE_CREATE_FILE_QUERY)
            .bind(id)
            .bind(name)
            .bind(project_id)
            .execute(&self.pool)
            .await
            .map_err(|err| MitnikaError::SQLiteDBError(err.to_string()))
    }

    pub async fn _delete_file(
        &self,
        id: &str,
    ) -> std::result::Result<SqliteQueryResult, MitnikaError> {
        sqlx::query(FILE_DELETE_FILE_QUERY)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|err| MitnikaError::SQLiteDBError(err.to_string()))
    }

    pub async fn _update_file_path(
        &self,
        id: &str,
        file_path: &str,
    ) -> std::result::Result<SqliteQueryResult, MitnikaError> {
        sqlx::query(FILE_UPDATE_FILE_PATH_QUERY)
            .bind(file_path)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|err| MitnikaError::SQLiteDBError(err.to_string()))
    }

    pub async fn _update_file_name(
        &self,
        id: &str,
        name: &str,
    ) -> std::result::Result<SqliteQueryResult, MitnikaError> {
        sqlx::query(FILE_UPDATE_FILE_NAME_QUERY)
            .bind(name)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|err| MitnikaError::SQLiteDBError(err.to_string()))
    }

    pub async fn _find_file_for_project(
        &self,
        project_id: &str,
    ) -> std::result::Result<SqliteQueryResult, MitnikaError> {
        sqlx::query(FILE_FIND_FILE_FOR_PROJECT)
            .bind(project_id)
            .execute(&self.pool)
            .await
            .map_err(|err| MitnikaError::SQLiteDBError(err.to_string()))
    }

    pub async fn _find_file_like(
        &self,
        search: &str,
    ) -> std::result::Result<SqliteQueryResult, MitnikaError> {
        sqlx::query(FILE_FIND_FILE_LIKE)
            .bind(search)
            .execute(&self.pool)
            .await
            .map_err(|err| MitnikaError::SQLiteDBError(err.to_string()))
    }

    pub async fn _create_environment(
        &self,
        id: &str,
        name: &str,
        file_id: &str,
    ) -> std::result::Result<SqliteQueryResult, MitnikaError> {
        sqlx::query(ENV_CREATE_ENVIRONMENT_QUERY)
            .bind(id)
            .bind(name)
            .bind(file_id)
            .execute(&self.pool)
            .await
            .map_err(|err| MitnikaError::SQLiteDBError(err.to_string()))
    }

    pub async fn _delete_environment(
        &self,
        id: &str,
    ) -> std::result::Result<SqliteQueryResult, MitnikaError> {
        sqlx::query(ENV_DELETE_ENVIRONMENT_QUERY)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|err| MitnikaError::SQLiteDBError(err.to_string()))
    }

    pub async fn _update_environment_name(
        &self,
        id: &str,
        name: &str,
    ) -> std::result::Result<SqliteQueryResult, MitnikaError> {
        sqlx::query(ENV_UPDATE_ENV_NAME_QUERY)
            .bind(name)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|err| MitnikaError::SQLiteDBError(err.to_string()))
    }

    pub async fn _find_env_like(
        &self,
        search: &str,
    ) -> std::result::Result<SqliteQueryResult, MitnikaError> {
        sqlx::query(ENV_FIND_ENV_LIKE)
            .bind(search)
            .execute(&self.pool)
            .await
            .map_err(|err| MitnikaError::SQLiteDBError(err.to_string()))
    }

    pub async fn _find_env_for_file(
        &self,
        file_id: &str,
    ) -> std::result::Result<SqliteQueryResult, MitnikaError> {
        sqlx::query(ENV_FIND_ENV_FOR_FILE)
            .bind(file_id)
            .execute(&self.pool)
            .await
            .map_err(|err| MitnikaError::SQLiteDBError(err.to_string()))
    }

    pub async fn _create_version(
        &self,
        id: &str,
        name: &str,
        environment_id: &str,
        content: &str,
    ) -> std::result::Result<SqliteQueryResult, MitnikaError> {
        sqlx::query(VERSION_CREATE_VERSION_QUERY)
            .bind(id)
            .bind(name)
            .bind(environment_id)
            .bind(content)
            .execute(&self.pool)
            .await
            .map_err(|err| MitnikaError::SQLiteDBError(err.to_string()))
    }

    pub async fn _delete_version(
        &self,
        id: &str,
    ) -> std::result::Result<SqliteQueryResult, MitnikaError> {
        sqlx::query(VERSION_DELETE_VERSION_QUERY)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|err| MitnikaError::SQLiteDBError(err.to_string()))
    }

    pub async fn _update_version_name(
        &self,
        id: &str,
        name: &str,
    ) -> std::result::Result<SqliteQueryResult, MitnikaError> {
        sqlx::query(VERSION_UPDATE_VERSION_NAME_QUERY)
            .bind(name)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|err| MitnikaError::SQLiteDBError(err.to_string()))
    }

    pub async fn _update_version_content(
        &self,
        id: &str,
        content: &str,
    ) -> std::result::Result<SqliteQueryResult, MitnikaError> {
        sqlx::query(VERSION_UPDATE_CONTENT_QUERY)
            .bind(content)
            .bind(id)
            .execute(&self.pool)
            .await
            .map_err(|err| MitnikaError::SQLiteDBError(err.to_string()))
    }

    pub async fn _find_version_like(
        &self,
        search: &str,
    ) -> std::result::Result<SqliteQueryResult, MitnikaError> {
        sqlx::query(VERSION_FIND_VERSION_LIKE)
            .bind(search)
            .execute(&self.pool)
            .await
            .map_err(|err| MitnikaError::SQLiteDBError(err.to_string()))
    }

    pub async fn _find_version_for_env(
        &self,
        environment_id: &str,
    ) -> std::result::Result<SqliteQueryResult, MitnikaError> {
        sqlx::query(VERSION_FIND_VERSION_FOR_ENV)
            .bind(environment_id)
            .execute(&self.pool)
            .await
            .map_err(|err| MitnikaError::SQLiteDBError(err.to_string()))
    }
}
