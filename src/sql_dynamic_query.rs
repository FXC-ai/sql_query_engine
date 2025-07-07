use regex::Regex;
use sqlx::types::chrono;
use sqlx::{FromRow, PgPool};
use crate::sql_query::SqlQuery;
use crate::sql_query_param::SqlQueryParam;
use crate::sql_query_engine_error::SqlQueryEngineError;
use crate::sql_dynamic_query_data::SqlDynamicQueryData;
use crate::sql_query_param_type::SqlQueryParamType;

/// Cette structure est utilisée pour représenter une requête SQL dynamique.
/// Elle contient une requête SQL et éventuellement des paramètres associés.
/// # Fields
/// - `query`: La requête SQL à exécuter.
/// - `params`: Optionnellement, une liste de paramètres pour la requête SQL.
#[derive(Debug, Clone, FromRow)]
pub struct SqlDynamicQuery
{
    pub query : SqlQuery,
    pub params: Option<Vec<SqlQueryParam>>,
}

impl SqlDynamicQuery
{
    /// Vérifie que les paramètres fournis correspondent aux paramètres requis de la requête
    /// 
    /// # Arguments
    /// * `dynamic_query_data` - Les données contenant les paramètres à valider
    /// 
    /// # Returns
    /// * `Ok(())` - Si tous les paramètres sont valides
    /// * `Err(SqlQueryEngineError::ErrorCheckParams)` - Si la validation échoue
    /// 
    /// # Validations effectuées
    /// - Vérification que tous les paramètres requis sont présents
    /// - Validation du type de chaque paramètre
    /// - Vérification qu'aucun paramètre superflu n'est fourni
    
    pub fn check_query_params(&self, dynamic_query_data: &SqlDynamicQueryData) -> Result<(), SqlQueryEngineError>
    {

        // Si la requête n'a pas de paramètres définis
        let query_params = match &self.params {
            Some(params) => params,
            None => {
                // Si aucun paramètre n'est défini mais que des paramètres sont fournis
                if !dynamic_query_data.params.is_empty() {
                    return Err(SqlQueryEngineError::ErrorCheckParams(
                        format!("Query '{}' expects no parameters, but {} parameters were provided", 
                            self.query.item_key, dynamic_query_data.params.len())
                    ));
                }
                return Ok(());
            }
        };

        // Vérifier que tous les paramètres requis sont présents
        for query_param in query_params {
            if query_param.is_required == 1
            {
                let param_found = dynamic_query_data.params.iter()
                    .any(|(name, _)| name == &query_param.param_name);
                
                if !param_found {
                    return Err(SqlQueryEngineError::ErrorCheckParams(
                        format!("Required parameter '{}' is missing for query '{}'", 
                            query_param.param_name, self.query.item_key)
                    ));
                }
            }
        }

        // Vérifier que tous les paramètres fournis sont attendus et ont le bon type
        for (param_name, param_value) in &dynamic_query_data.params
        {
            let query_param = query_params.iter()
                .find(|p| &p.param_name == param_name);
            
            let query_param = match query_param {
                Some(param) => param,
                None => {
                    return Err(SqlQueryEngineError::ErrorCheckParams(
                        format!("Unexpected parameter '{}' provided for query '{}'", 
                            param_name, self.query.item_key)
                    ));
                }
            };

            // Valider le type du paramètre
            if let Err(validation_error) = Self::validate_param_type(&query_param.param_type, param_value) {
                return Err(SqlQueryEngineError::ErrorCheckParams(
                    format!("Parameter '{}' validation failed for query '{}': {}", 
                        param_name, self.query.item_key, validation_error)
                ));
            }
        }

        Ok(())
    }

    /// Valide qu'une valeur correspond au type attendu
    /// 
    /// # Arguments
    /// * `expected_type` - Le type attendu (String, Integer, Float, Boolean, DateTime)
    /// * `value` - La valeur à valider
    /// 
    /// # Returns
    /// * `Ok(())` - Si la valeur correspond au type
    /// * `Err(String)` - Message d'erreur si la validation échoue
    fn validate_param_type(expected_type: &str, value: &str) -> Result<(), String> {
        // Convertir le type string en enum pour une validation plus robuste
        let param_type = match SqlQueryParamType::from(expected_type.to_string())
        {
            param_type => param_type,
        };

        match param_type {
            SqlQueryParamType::String => {
                // Toute chaîne est valide pour le type String
                Ok(())
            },
            SqlQueryParamType::I32 => {
                value.parse::<i32>()
                    .map(|_| ())
                    .map_err(|_| format!("'{}' is not a valid integer", value))
            },
            SqlQueryParamType::F64 => {
                value.parse::<f64>()
                    .map(|_| ())
                    .map_err(|_| format!("'{}' is not a valid float", value))
            },
            SqlQueryParamType::Bool => {
                match value.to_lowercase().as_str() {
                    "true" | "false" | "1" | "0" | "yes" | "no" | "on" | "off" => Ok(()),
                    _ => Err(format!("'{}' is not a valid boolean (expected: true/false, 1/0, yes/no, on/off)", value))
                }
            },
            SqlQueryParamType::NaiveDateTime => {
                // Validation basique pour les formats de date/heure courants
                // On peut utiliser chrono pour une validation plus robuste si nécessaire
                if value.is_empty() {
                    return Err("DateTime cannot be empty".to_string());
                }
                
                // Patterns basiques pour ISO 8601, formats SQL standards
                // SQL standard: 2023-12-25 10:30:00
                let datetime_pattern = r"^\d{4}-\d{2}-\d{2} \d{2}:\d{2}:\d{2}$";

                // ISO 8601: 2023-12-25T10:30:00Z
                // r"^\d{4}-\d{2}-\d{2}T\d{2}:\d{2}:\d{2}Z?$",
                
                // Date seule: 2023-12-25
                // r"^\d{4}-\d{2}-\d{2}$",
                
                let is_valid = regex::Regex::new(datetime_pattern)
                        .map(|re| re.is_match(value))
                        .unwrap_or(false);
                
                if is_valid {Ok(())}
                else {Err(format!("'{}' is not a valid datetime format (expected: YYYY-MM-DD, YYYY-MM-DD HH:MM:SS, or ISO 8601)", value))}
            },
            SqlQueryParamType::NaiveDate => {
                // Validation basique pour les dates
                // On peut utiliser chrono pour une validation plus robuste si nécessaire
                if value.is_empty() {
                    return Err("Date cannot be empty".to_string());
                }
                
                // Pattern basique pour le format SQL standard: 2023-12-25
                let date_pattern = r"^\d{4}-\d{2}-\d{2}$";
                
                let is_valid = regex::Regex::new(date_pattern)
                        .map(|re| re.is_match(value))
                        .unwrap_or(false);
                
                if is_valid {Ok(())}
                else {Err(format!("'{}' is not a valid date format (expected: YYYY-MM-DD)", value))}
            },
        }
    }

    pub async fn execute<T>
    (
        &self,
        pool: &PgPool,
        dynamic_query_data: SqlDynamicQueryData,
    ) -> Result<Vec<T>, SqlQueryEngineError>
    where
        T: for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
    {
        // Étape 1 : Vérification des paramètres
        self.check_query_params(&dynamic_query_data)?;

        // Étape 2 : Construction de la requête SQL dynamique
        let mut query = sqlx::query_as::<sqlx::Postgres, T>(self.query.sql_code());

        
        if let Some(params) = &self.params
        {
            // params.sort_by_key(|p| p.param_order);
            for param in params
            {
                let value = match dynamic_query_data.get_param(&param.param_name)
                {
                    Some(v) => v,
                    None => {
                        if let Some(default) = &param.default_value
                        {
                            default
                        }
                        else
                        {
                            return Err(SqlQueryEngineError::ErrorExecutionQuery(format!(
                                "Parameter '{}' is missing and has no default value",
                                param.param_name
                            )));
                        }
                    }
                };

                
                let param_type = SqlQueryParamType::from(param.param_type.clone());


                query = match param_type
                {
                    SqlQueryParamType::String => query.bind(value.to_string()),

                    SqlQueryParamType::I32 => match value.parse::<i32>()
                    {
                        Ok(v) => query.bind(v),
                        Err(_) => return Err(SqlQueryEngineError::ErrorExecutionQuery(format!(
                            "Invalid integer value for '{}'",
                            param.param_name
                        ))),
                    },

                    SqlQueryParamType::F64 => match value.parse::<f64>()
                    {
                        Ok(v) => query.bind(v),
                        Err(_) => return Err(SqlQueryEngineError::ErrorExecutionQuery(format!(
                            "Invalid float value for '{}'",
                            param.param_name
                        ))),
                    },

                    SqlQueryParamType::Bool => match value.to_lowercase().as_str()
                    {
                        "true" | "1" | "yes" | "on" => query.bind(true),
                        "false" | "0" | "no" | "off" => query.bind(false),
                        _ => return Err(SqlQueryEngineError::ErrorExecutionQuery(format!(
                            "Invalid boolean value for '{}'",
                            param.param_name
                        ))),
                    },

                    SqlQueryParamType::NaiveDate => match chrono::NaiveDate::parse_from_str(value, "%Y-%m-%d")
                    {
                        Ok(dt) => query.bind(dt),
                        Err(e) => return Err(SqlQueryEngineError::ErrorExecutionQuery(format!(
                            "{} || Invalid datetime format for '{}' : '{}'. Expected format: 'YYYY-MM-DD'",
                            e,
                            param.param_name,
                            value
                        ))),
                    },
                    SqlQueryParamType::NaiveDateTime => match chrono::NaiveDateTime::parse_from_str(value, "%Y-%m-%d %H:%M:%S")
                    {
                        Ok(dt) => query.bind(dt),
                        Err(e) => return Err(SqlQueryEngineError::ErrorExecutionQuery(format!(
                            "{} || Invalid datetime format for '{}' : '{}'. Expected format: 'YYYY-MM-DD HH:MM:SS'",
                            e,
                            param.param_name,
                            value
                        ))),
                    },
                };
            }
        }

        // Étape 3 : Exécution
        match query.fetch_all(pool).await {
            Ok(result) => Ok(result),
            Err(e) => Err(SqlQueryEngineError::ErrorExecutionQuery(format!(
                "Error executing query '{}': {}",
                self.query.item_key, e
            ))),
        }
    }
}
    
    
