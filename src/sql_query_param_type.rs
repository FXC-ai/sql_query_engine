#[derive(Debug, Clone)]
pub enum SqlQueryParamType {
    String,
    I32,
    F64,
    Bool,
    NaiveDate,
    NaiveDateTime,
}

impl From<String> for SqlQueryParamType {
    fn from(value: String) -> Self {
        match value.as_str() {
            "VARCHAR" | "Varchar" => SqlQueryParamType::String,
            "BIGINT" | "INTEGER" | "Integer" => SqlQueryParamType::I32,
            "DOUBLE PRECISION" | "DOUBLE_PRECISION" => SqlQueryParamType::F64,
            "BOOLEAN" | "Boolean" => SqlQueryParamType::Bool,
            "DATE" | "Date" => SqlQueryParamType::NaiveDate,
            "DATETIME" | "DateTime" => SqlQueryParamType::NaiveDateTime,
            
            _ => panic!("Unknown SQL query parameter type: {}", value),
        }
    }
}