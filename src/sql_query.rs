use sqlx::prelude::FromRow;

#[derive(Debug, Clone, FromRow)]

/// Cette structure est utilisée pour représenter une requête SQL dans le système.
/// pub id: i32,
/// pub name: String,
/// pub description: Option<String>,
/// pub sql_code: String,
/// pub item_key: String,
/// pub sign: String,
/// 

pub struct SqlQuery {
   pub id: i32,
   pub name: String,
   pub description: Option<String>,
   pub sql_code: String,
   pub item_key: String,
   pub sign: Option<String>,
}

impl SqlQuery {
    /// Crée une nouvelle instance de `SqlQuery`.
    pub fn new(
        id: i32,
        name: String,
        description: Option<String>,
        sql_code: String,
        item_key: String,
        sign: Option<String>,
    ) -> Self {
        Self {
            id,
            name,
            description,
            sql_code,
            item_key,
            sign,
        }
    }
    /// Retourne le code SQL de la requête.
    pub fn sql_code(&self) -> &str {
        &self.sql_code
    }
}