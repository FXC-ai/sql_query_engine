// use sqlx::{Pool, FromRow};

use sqlx::FromRow;
use crate::sql_dynamic_query::SqlDynamicQuery;
use crate::sql_query::SqlQuery;
use crate::sql_query_param::SqlQueryParam;
use crate::sql_query_engine_error::SqlQueryEngineError;


/// Cette structure est utilisée pour gérer les requêtes SQL dans la base de données.
/// La table doit obligatoirement contenir les colonnes suivantes :
/// - `id`: Identifiant unique de la requête
/// - `name`: Nom de la requête
/// - `description`: Description de la requête (optionnelle)
/// - `sql_code`: Code SQL de la requête
/// - `item_key`: Clé d'élément unique pour identifier la requête
/// - `sign` : Signature de la requête (optionnelle)

#[derive(Debug, FromRow)]
pub struct SqlQueryManager<'a>
{
    /// Pool de connexions à la base de données
    pool: &'a sqlx::Pool<sqlx::Postgres>,

    /// Nom de la table SQL contenant les requêtes
    table_query: String,

    /// Nom de la table SQL contenant les paramètres de requête
    table_query_params: String,
}

impl <'a> SqlQueryManager<'a> {

    /// Crée une nouvelle instance de `SqlQueryManager`.
    /// # Arguments
    /// * `pool`: Pool de connexions à la base de données
    /// * `table_query`: Nom de la table SQL contenant les requêtes
    /// * `table_query_params`: Nom de la table SQL contenant les paramètres de requête
    
    pub fn new(pool: &'a sqlx::Pool<sqlx::Postgres>, table_query : String, table_query_params : String) -> Self {
        Self { 
            pool,
            table_query,
            table_query_params,
        }
    }
    
   /// Récupère une requête par son item_key
   /// # Arguments
   /// * `item_key`: Clé d'élément unique pour identifier la requête
   
   pub async fn get_sql_query_by_item_key(&self, item_key: &str) -> Result<Option<SqlQuery>, SqlQueryEngineError>
   {
        let query  = format!(
            "SELECT * FROM {} WHERE item_key = $1",
            self.table_query
        );

        match sqlx::query_as::<sqlx::Postgres, SqlQuery>(query.as_str())
            .bind(item_key)
            .fetch_optional(self.pool)
            .await
        {
            Ok(query) => {
                if query.is_none() {
                    return Err(SqlQueryEngineError::ErrorNoQueryFound(format!("get_query_by_item_key : Failed to fetch query with item_key '{}': {}", item_key, "not found")));
                }
                Ok(query)
            },
            Err(e) => Err
            (
                SqlQueryEngineError::ErrorGetSqlQuery(format!("get_query_by_item_key : Failed to fetch query on table '{}' with item_key {} : {}", self.table_query, item_key, e))
            ),
        }
    }

    
    /// Récupère les paramètres d'une requête SQL par son item_key
    /// # Arguments
    /// * `item_key`: Clé d'élément unique pour identifier la requête
    pub async fn get_sql_query_params_by_item_key(&self, item_key: &str) -> Result<Option<Vec<SqlQueryParam>>, SqlQueryEngineError>
    {
        let query = format!(
            r#"
                SELECT
                    qp.id,
                    qp.param_name,
                    qp.param_type,
                    qp.param_order,
                    qp.is_required,
                    qp.default_value,
                    qp.description,
                    qp.item_key
                FROM {} qp
                INNER JOIN {} q ON qp.item_key = q.item_key
                WHERE qp.item_key = $1
            "#,
            self.table_query_params,
            self.table_query
        );

        match sqlx::query_as::<sqlx::Postgres, SqlQueryParam>(query.as_str())
            .bind(item_key)
            .fetch_all(self.pool)
            .await
        {
            Ok(mut params) =>
            {
                if params.is_empty()
                {
                    Ok(None)
                } 
                else
                {
                    params.sort_by_key(|p| p.param_order);
                    Ok(Some(params))
                }
            },
            Err(e) => Err(SqlQueryEngineError::ErrorGetSqlQueryParam(
                format!("get_sql_query_params_by_item_key : Failed to fetch query parameters on table '{}' with item_key '{}': {}", 
                    self.table_query_params, item_key, e)
            )),
        }
    }

    /// Récupère une requête dynamique complète (requête + paramètres) par son item_key
    /// # Arguments
    /// * `item_key`: Clé d'élément unique pour identifier la requête
    /// # Returns
    /// * `Ok(Some(SqlDynamicQuery))`: Si la requête est trouvée
    /// * `Ok(None)`: Si aucune requête n'est trouvée avec cette item_key
    /// * `Err(SqlQueryEngineError)`: En cas d'erreur lors de la récupération
    
    pub async fn get_sql_dynamic_query(&self, item_key: &str) -> Result<Option<SqlDynamicQuery>, SqlQueryEngineError>
    {
        // Récupérer la requête SQL
        let query = match self.get_sql_query_by_item_key(item_key).await? {
            Some(q) => q,
            None => return Ok(None), // Pas de requête trouvée
        };

        // Récupérer les paramètres (peut être None si pas de paramètres)
        let params = self.get_sql_query_params_by_item_key(item_key).await?;

        // Construire et retourner la SqlDynamicQuery
        Ok(Some(SqlDynamicQuery {
            query,
            params,
        }))
    }

}