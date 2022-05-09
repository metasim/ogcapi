use async_trait::async_trait;
use serde_json::Value;
use sqlx::types::Json;

use ogcapi_types::styles::{Style, Styles, Stylesheet};

use crate::StyleTransactions;

use super::Db;

#[async_trait]
impl StyleTransactions for Db {
    async fn list_styles(&self) -> Result<Styles, anyhow::Error> {
        let styles = sqlx::query_scalar!(
            r#"
            SELECT array_to_json(array_agg(row_to_json(t))) as "styles!: Json<Vec<Style>>"
            FROM (
                SELECT id, title, links FROM meta.styles
            ) t
            "#
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(Styles { styles: styles.0 })
    }

    async fn read_style(&self, id: &str) -> Result<Value, anyhow::Error> {
        let style = sqlx::query_scalar!(
            r#"
            SELECT row_to_json(t) as "stylesheet!: Json<Stylesheet>"
            FROM (
                SELECT id, value FROM meta.styles WHERE id = $1
            ) t
            "#,
            id
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(style.0.value)
    }
}
