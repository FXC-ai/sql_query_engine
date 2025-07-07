#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SqlQueryEngineError
{
   ErrorGetSqlQuery(String),
   ErrorNoQueryFound(String),
   ErrorGetSqlQueryParam(String),
   ErrorGetDynamicQuery(String),
   ErrorExecutionQuery(String),
   ErrorCheckParams(String),
}