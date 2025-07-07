
#[cfg(test)]
mod tests
{
    use sqlx::{postgres::PgPoolOptions, types::chrono};
    use crate::{sql_dynamic_query_data::SqlDynamicQueryData, sql_query_manager::SqlQueryManager};

    async fn setup() -> sqlx::Pool<sqlx::Postgres> {
        let pool = PgPoolOptions::new()
            .max_connections(1)
            .connect("postgres://postgres:a2020@pg:5432/aeveil")
            .await
            .expect("Failed to connect to the database");
        pool
    }
    
    #[tokio::test]
    async fn test_get_query_by_item_key_0 ()
    {
        let pool = setup().await;

        let sql_query_manager = SqlQueryManager::new(&pool, String::from("data_analyst.queries"), String::from("data_analyst.parameters"));

        let sql_query = sql_query_manager
            .get_sql_query_by_item_key("select.atelier")
            .await;

        assert!(sql_query.is_ok());
    }

    #[tokio::test]
    async fn test_get_query_by_item_key_1 ()
    {
        let pool = setup().await;

        let sql_query_manager = SqlQueryManager::new(&pool, String::from("data_analyst.queries"), String::from("data_analyst.parameters"));

        let sql_query = sql_query_manager
            .get_sql_query_by_item_key("item_qui_n_existe_pas")
            .await;

        assert!(sql_query.is_err());
    }

    #[tokio::test]
    async fn test_get_sql_query_params_by_item_key_0() {
       
        let pool = setup().await;

        let manager = SqlQueryManager::new(
            &pool,
            "data_analyst.queries".to_string(),
            "data_analyst.parameters".to_string(),
        );

        let result = manager.get_sql_query_params_by_item_key("select.atelier").await;

        assert!(result.is_ok(), "La requête devrait réussir");
        
        let params_option = result.unwrap();
        assert!(params_option.is_some(), "Des paramètres devraient être trouvés");
        
        let params = params_option.unwrap();
        assert_eq!(params.len(), 1, "Il devrait y avoir exactement 1 paramètre");
        
        let param = &params[0];
        assert_eq!(param.param_name, "id");
        assert_eq!(param.param_type, "BIGINT");
        assert_eq!(param.param_order, 1);
        assert_eq!(param.is_required, 1);
        assert_eq!(param.item_key, "select.atelier");
        assert_eq!(param.description, Some("Identifiant de l'atelier".to_string()));
    }

    #[tokio::test]
    async fn test_get_sql_query_params_by_item_key_1()
    {
        let pool = setup().await;

        let manager = SqlQueryManager::new(
            &pool,
            "data_analyst.queries".to_string(),
            "data_analyst.parameters".to_string(),
        );

        let result = manager.get_sql_query_params_by_item_key("item_key_inconnu").await;
        assert_eq!(result.is_ok(), true, "La requête renvoie un résultat vide");
    }

    #[tokio::test]
    async fn test_get_dynamic_query_success()
    {
        let pool = setup().await;
        let manager = SqlQueryManager::new(
            &pool,
            "data_analyst.queries".to_string(),
            "data_analyst.parameters".to_string(),
        );

        let result = manager.get_sql_dynamic_query("select.atelier").await;
        
        assert!(result.is_ok(), "La requête devrait réussir");
        
        let dynamic_query_option = result.unwrap();
        assert!(dynamic_query_option.is_some(), "Une requête dynamique devrait être trouvée");
        
        let dynamic_query = dynamic_query_option.unwrap();
        
        // Vérifier que la requête est présente
        assert_eq!(dynamic_query.query.item_key, "select.atelier");
        
        // Vérifier que les paramètres sont présents
        assert!(dynamic_query.params.is_some(), "Des paramètres devraient être présents");
        let params = dynamic_query.params.unwrap();
        assert_eq!(params.len(), 1, "Il devrait y avoir exactement 1 paramètre");
        
        let param = &params[0];
        assert_eq!(param.param_name, "id");
        assert_eq!(param.param_type, "BIGINT");
        assert_eq!(param.param_order, 1);
        assert_eq!(param.is_required, 1);
        assert_eq!(param.item_key, "select.atelier");
        assert_eq!(param.description, Some("Identifiant de l'atelier".to_string()));
    }

    #[tokio::test]
    async fn test_get_dynamic_query_not_found()
    {
        let pool = setup().await;
        let manager = SqlQueryManager::new(
            &pool,
            "data_analyst.queries".to_string(),
            "data_analyst.parameters".to_string(),
        );

        let result = manager.get_sql_dynamic_query("item_key_inexistant").await;
        assert!(result.is_err(), "La requête n'existe pas, donc une erreur devrait être renvoyée");
    }

    #[tokio::test]
    async fn test_check_params()
    {
        let pool = setup().await;
        let manager = SqlQueryManager::new(
            &pool,
            "data_analyst.queries".to_string(),
            "data_analyst.parameters".to_string(),
        );

        let datas_from_front = SqlDynamicQueryData::new(
            "participant.recherche".to_string(),
            vec![
                ("nom_participant".to_string(), "Dupont".to_string()),
                // ("prenom_participant".to_string(), "Jean".to_string()),
            ].into_iter().collect()
        );

        let result = manager.get_sql_dynamic_query(&datas_from_front.item_key).await;
        
        assert!(result.is_ok(), "La requête devrait réussir");
        
        let dynamic_query_option = result.unwrap();
        assert!(dynamic_query_option.is_some(), "Une requête dynamique devrait être trouvée");
        
        let dynamic_query = dynamic_query_option.unwrap();
        
        // Vérifier que la requête est présente
        assert_eq!(dynamic_query.query.item_key, datas_from_front.item_key);

        dynamic_query.check_query_params(&datas_from_front).expect("Les paramètres devraient être valides");
    }

    #[tokio::test]
    async fn test_check_params_1()
    {
        let pool = setup().await;
        let manager = SqlQueryManager::new(
            &pool,
            "data_analyst.queries".to_string(),
            "data_analyst.parameters".to_string(),
        );

        let datas_from_front = SqlDynamicQueryData::new(
            "participant.recherche".to_string(),
            vec![
                ("nom_participant".to_string(), "Dupont".to_string()),
            ].into_iter().collect()
        );

        let result = manager.get_sql_dynamic_query(&datas_from_front.item_key).await;
        
        assert!(result.is_ok(), "La requête devrait réussir");
        
        let dynamic_query_option = result.unwrap();
        assert!(dynamic_query_option.is_some(), "Une requête dynamique devrait être trouvée");
        
        let dynamic_query = dynamic_query_option.unwrap();
        
        // Vérifier que la requête est présente
        assert_eq!(dynamic_query.query.item_key, datas_from_front.item_key);

        dynamic_query.check_query_params(&datas_from_front).expect("Les paramètres devraient être valides");
    }

    #[tokio::test]
    async fn test_check_params_2()
    {
        let pool = setup().await;
        let manager = SqlQueryManager::new(
            &pool,
            "data_analyst.queries".to_string(),
            "data_analyst.parameters".to_string(),
        );

        let datas_from_front = SqlDynamicQueryData::new(
            "test.activite_pro".to_string(),
            vec![
                ("participant_id".to_string(), "1021".to_string()),
                ("item_date_start".to_string(), "2000-01-01".to_string()),
                ("item_date_end".to_string(), "2055-01-01".to_string()),
            ].into_iter().collect()
        );

        let result = manager.get_sql_dynamic_query(&datas_from_front.item_key).await;
        assert!(result.is_ok(), "La requête aurait dû réussir");
        
        let dynamic_query_option = result.unwrap();
        assert!(dynamic_query_option.is_some(), "Une requête dynamique devrait être trouvée");
        
        let dynamic_query = dynamic_query_option.unwrap();
        assert_eq!(dynamic_query.query.item_key, datas_from_front.item_key);

        dynamic_query.check_query_params(&datas_from_front).expect("Les paramètres devraient être valides");
    }

    #[tokio::test]
    async fn test_check_params_3()
    {
        let pool = setup().await;
        let manager = SqlQueryManager::new(
            &pool,
            "data_analyst.queries".to_string(),
            "data_analyst.parameters".to_string(),
        );

        let datas_from_front = SqlDynamicQueryData::new(
            "test.activite_pro".to_string(),
            vec![
                ("participant_id".to_string(), "1021".to_string()),
                ("item_date_start".to_string(), "2000-01-01".to_string()),
                ("item_date_ende".to_string(), "2055-01-01".to_string()),
            ].into_iter().collect()
        );

        let result = manager.get_sql_dynamic_query(&datas_from_front.item_key).await;
        
        assert!(result.is_ok(), "La requête devrait réussir");
        
        let dynamic_query_option = result.unwrap();
        assert!(dynamic_query_option.is_some(), "Une requête dynamique devrait être trouvée");
        
        let dynamic_query = dynamic_query_option.unwrap();
        
        // Vérifier que la requête est présente
        assert_eq!(dynamic_query.query.item_key, datas_from_front.item_key);

        dynamic_query.check_query_params(&datas_from_front).expect_err("Les paramètres devraient être invalides");
    }

    #[tokio::test]
    async fn test_check_params_4()
    {
        let pool = setup().await;
        let manager = SqlQueryManager::new(
            &pool,
            "data_analyst.queries".to_string(),
            "data_analyst.parameters".to_string(),
        );

        let datas_from_front = SqlDynamicQueryData::new(
            "test.activite_pro".to_string(),
            vec![].into_iter().collect()
        );

        let result = manager.get_sql_dynamic_query(&datas_from_front.item_key).await;
        
        assert!(result.is_ok(), "La requête devrait réussir");
        
        let dynamic_query_option = result.unwrap();
        assert!(dynamic_query_option.is_some(), "Une requête dynamique devrait être trouvée");
        
        let dynamic_query = dynamic_query_option.unwrap();
        
        // Vérifier que la requête est présente
        assert_eq!(dynamic_query.query.item_key, datas_from_front.item_key);

        dynamic_query.check_query_params(&datas_from_front).expect_err("Les paramètres devraient être invalides");
    }

    #[derive(Debug, sqlx::FromRow)]
    struct Answer {
        id : i32,
        participant_id : i32,
        activite_id : i32,
        item_date : chrono::NaiveDate,
    }

    #[tokio::test]
    async fn test_execute()
    {

        let pool = setup().await;
        let manager = SqlQueryManager::new(
            &pool,
            "data_analyst.queries".to_string(),
            "data_analyst.parameters".to_string(),
        );

        let datas_from_front = SqlDynamicQueryData::new(
            "test.activite_pro".to_string(),
            vec![
                ("participant_id".to_string(), "1021".to_string()),
                ("item_date_start".to_string(), "2000-01-01".to_string()),
                ("item_date_end".to_string(), "2055-01-01".to_string()),
            ].into_iter().collect()
        );

        let result = manager.get_sql_dynamic_query(&datas_from_front.item_key).await;
        
        assert!(result.is_ok(), "La requête devrait réussir");
        
        let dynamic_query_option = result.unwrap();
        
        assert!(dynamic_query_option.is_some(), "Une requête dynamique devrait être trouvée");
        
        let dynamic_query = dynamic_query_option.unwrap();
        
        assert_eq!(dynamic_query.query.item_key, datas_from_front.item_key);

        dynamic_query.execute::<Answer>(&pool,datas_from_front)
            .await
            .expect("L'exécution de la requête devrait réussir");
    }

    #[derive(Debug, sqlx::FromRow)]
    struct Atelier {
        id: i32,
        item_name: String,
    }

    #[tokio::test]
    async fn test_execute2()
    {

        let pool = setup().await;
        let manager = SqlQueryManager::new(
            &pool,
            "data_analyst.queries".to_string(),
            "data_analyst.parameters".to_string(),
        );

        let datas_from_front = SqlDynamicQueryData::new(
            "select.atelier".to_string(),
            vec![
                ("id".to_string(), "123".to_string()),
            ].into_iter().collect()
        );

        let result = manager.get_sql_dynamic_query(&datas_from_front.item_key).await;
        
        assert!(result.is_ok(), "La requête devrait réussir");
        
        let dynamic_query_option = result.unwrap();
        
        assert!(dynamic_query_option.is_some(), "Une requête dynamique devrait être trouvée");
        
        let dynamic_query = dynamic_query_option.unwrap();
        
        assert_eq!(dynamic_query.query.item_key, datas_from_front.item_key);

        dynamic_query.execute::<Atelier>(&pool,datas_from_front)
            .await
            .expect("L'exécution de la requête devrait réussir");
    }

}