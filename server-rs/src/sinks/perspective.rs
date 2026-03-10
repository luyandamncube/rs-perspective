// server-rs\src\sinks\perspective.rs
use anyhow::Result;
use perspective::{
    client::{Table, TableData, TableInitOptions, UpdateData, UpdateOptions},
    proto::ColumnType,
    server::{LocalClient, Server},
};

use crate::models::DemoRow;
use crate::transforms::demo_rows::demo_rows_to_value;

pub const TICKS_TABLE_NAME: &str = "ticks";

pub struct PerspectiveState {
    pub server: Server,
    pub client: LocalClient,
    pub table: Table,
}

impl PerspectiveState {
    pub async fn bootstrap(table_limit: Option<u32>) -> Result<Self> {
        let server = Server::new(None);
        let client = server.new_local_client();

        let schema = TableData::Schema(vec![
            ("name".to_string(), ColumnType::String),
            ("client".to_string(), ColumnType::String),
            ("lastUpdate".to_string(), ColumnType::Datetime),
            ("chg".to_string(), ColumnType::Float),
            ("bid".to_string(), ColumnType::Float),
            ("ask".to_string(), ColumnType::Float),
            ("vol".to_string(), ColumnType::Float),
        ]);

        let options = TableInitOptions {
            name: Some(TICKS_TABLE_NAME.to_string()),
            limit: table_limit,
            ..TableInitOptions::default()
        };

        let table = client.table(schema, options).await?;

        Ok(Self { server, client, table })
    }
}

pub async fn apply_rows(table: &Table, rows: &[DemoRow]) -> Result<()> {
    if rows.is_empty() {
        return Ok(());
    }

    let payload = serde_json::to_string(&demo_rows_to_value(rows))?;
    table
        .update(UpdateData::JsonRows(payload), UpdateOptions::default())
        .await?;

    Ok(())
}

// ------------------------------------------------
// UNIT TESTS
// ------------------------------------------------

#[cfg(test)]
mod tests {
    use super::*;
    use crate::models::DemoRow;

    #[tokio::test]
    async fn apply_rows_adds_rows() {
        let state = PerspectiveState::bootstrap(Some(5000)).await.unwrap();

        let rows = vec![
            DemoRow {
                name: "AAPL.N".to_string(),
                client: "Homer".to_string(),
                last_update: 1_741_500_000_000,
                chg: 1.25,
                bid: 100.5,
                ask: 101.0,
                vol: 105.0,
            },
            DemoRow {
                name: "AAPL.N".to_string(),
                client: "Marge".to_string(),
                last_update: 1_741_500_000_000,
                chg: 1.15,
                bid: 100.4,
                ask: 100.9,
                vol: 106.0,
            },
        ];

        apply_rows(&state.table, &rows).await.unwrap();

        let size = state.table.size().await.unwrap();
        assert_eq!(size, 2);
    }
}