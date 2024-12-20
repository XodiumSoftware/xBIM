#![warn(clippy::all, rust_2018_idioms)]
#![forbid(unsafe_code)]

use crate::schemas::{ToTableSchema, User, UserPreferences};
use rocket::serde::json::serde_json::Map;
use rocket::serde::json::Value;
use tokio_postgres::types::ToSql;
use tokio_postgres::{Client, Error as PostgresError, Row};

pub struct Database {
    pub(crate) client: Client,
}

impl Database {
    pub async fn new(client: Client) -> Result<Self, PostgresError> {
        Ok(Self { client })
    }

    pub async fn create_table<T: ToTableSchema>(&self) -> Result<(), PostgresError> {
        self.client
            .execute(
                &format!(
                    "CREATE TABLE IF NOT EXISTS {} ({})",
                    T::table_name(),
                    T::columns().join(", ")
                ),
                &[],
            )
            .await?;
        Ok(())
    }

    pub async fn initialize_schema(&self) -> Result<(), PostgresError> {
        self.create_table::<User>().await?;
        self.create_table::<UserPreferences>().await?;
        Ok(())
    }

    pub async fn add_data<T: ToSql + Sync>(
        &self,
        table: &str,
        columns: &[&str],
        values: &[&T],
    ) -> Result<u64, PostgresError> {
        Ok(self
            .client
            .execute(
                &format!(
                    "INSERT INTO {} ({}) VALUES ({})",
                    table,
                    columns.join(", "),
                    (1..=values.len())
                        .map(|i| format!("${}", i))
                        .collect::<Vec<_>>()
                        .join(", ")
                ),
                &values
                    .iter()
                    .map(|v| *v as &(dyn ToSql + Sync))
                    .collect::<Vec<_>>(),
            )
            .await?)
    }

    pub async fn get_data(
        &self,
        query: &str,
        params: &[&(dyn ToSql + Sync)],
    ) -> Result<Vec<Value>, PostgresError> {
        Ok(self
            .client
            .query(query, params)
            .await?
            .into_iter()
            .map(|row| extract_row_as_json(row))
            .collect())
    }

    pub async fn delete_data<T: ToSql + Sync>(
        &self,
        table: &str,
        condition: &str,
        params: &[&T],
    ) -> Result<u64, PostgresError> {
        Ok(self
            .client
            .execute(
                &format!("DELETE FROM {} WHERE {}", table, condition),
                &params
                    .iter()
                    .map(|p| *p as &(dyn ToSql + Sync))
                    .collect::<Vec<_>>(),
            )
            .await?)
    }
}

fn extract_row_as_json(row: Row) -> Value {
    let mut json_map = Map::new();
    for (i, column) in row.columns().iter().enumerate() {
        json_map.insert(
            column.name().to_string(),
            row.try_get::<_, String>(i)
                .map(|v| Value::String(v))
                .or_else(|_| row.try_get::<_, i32>(i).map(|v| Value::Number(v.into())))
                .unwrap_or_else(|_| Value::Null),
        );
    }
    Value::Object(json_map)
}
