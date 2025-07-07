use sqlx::prelude::FromRow;

#[derive(Debug, Clone, FromRow)]
pub struct SqlQueryParam {
   pub id: i32,
   pub param_name: String,
   pub param_type: String,
   pub param_order: i32,
   pub is_required: i32,
   pub default_value: Option<String>,
   pub description: Option<String>,
   pub item_key: String,
}