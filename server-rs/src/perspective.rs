// server-rs\src\perspective.rs
use anyhow::Result;
use perspective::{
    client::{Table, TableData, TableInitOptions, UpdateData, UpdateOptions},
    proto::ColumnType,
    server::{LocalClient, Server},
};

use crate::ticks::{tick_to_row, Tick};

pub const TICKS_TABLE_NAME: &str = "ticks";

pub struct PerspectiveState {
    pub server: Server,
    pub client: LocalClient,
    pub table: Table,
}

impl PerspectiveState {
    pub async fn bootstrap() -> Result<Self> {
        let server = Server::new(None);
        let client = server.new_local_client();

        let schema = TableData::Schema(vec![
            ("symbol".to_string(), ColumnType::String),
            ("price".to_string(), ColumnType::Float),
            ("size".to_string(), ColumnType::Integer),
            ("ts".to_string(), ColumnType::Datetime),
        ]);

        let options = TableInitOptions {
            name: Some(TICKS_TABLE_NAME.to_string()),
            ..TableInitOptions::default()
        };

        let table = client.table(schema, options).await?;

        Ok(Self {
            server,
            client,
            table,
        })
    }
}

pub async fn apply_tick(table: &Table, tick: &Tick) -> Result<()> {
    let row = tick_to_row(tick);
    let payload = serde_json::to_string(&vec![row])?;

    table
        .update(UpdateData::JsonRows(payload), UpdateOptions::default())
        .await?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ticks::Tick;

    #[tokio::test]
    async fn bootstrap_creates_expected_schema() {
        let state = PerspectiveState::bootstrap().await.unwrap();
        let schema = state.table.schema().await.unwrap();

        assert_eq!(schema.get("symbol"), Some(&ColumnType::String));
        assert_eq!(schema.get("price"), Some(&ColumnType::Float));
        assert_eq!(schema.get("size"), Some(&ColumnType::Integer));
        assert_eq!(schema.get("ts"), Some(&ColumnType::Datetime));
    }

    #[tokio::test]
    async fn apply_tick_adds_one_row() {
        let state = PerspectiveState::bootstrap().await.unwrap();

        let tick = Tick {
            symbol: "AAPL".to_string(),
            price: 101.25,
            size: 50,
            ts: "2026-03-09T08:30:00Z".to_string(),
        };

        apply_tick(&state.table, &tick).await.unwrap();

        let size = state.table.size().await.unwrap();
        assert_eq!(size, 1);
    }
}