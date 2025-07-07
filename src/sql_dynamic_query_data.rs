use std::collections::HashMap;

/// Cette structure est utilisée pour représenter les données d'une requête SQL dynamique.
/// Elle contient la clé de l'élément (`item_key`) et un ensemble de paramètres associés à cette requête.
/// /// # Champs
/// - `item_key`: Clé d'élément unique pour identifier la requête dynamique.
/// - `params`: Un `HashMap` contenant les paramètres de la requête, où la clé est le nom du paramètre et la valeur est sa valeur sous forme de chaîne de caractères.
/// 

#[derive(Debug, Clone)]
pub struct SqlDynamicQueryData
{
    pub item_key: String,
    pub params: HashMap<String, String>,
}

impl SqlDynamicQueryData {
    /// Crée une nouvelle instance de `SqlDynamicQueryData`.
    ///
    /// # Arguments
    ///
    /// * `item_key` - Clé d'élément unique pour identifier la requête dynamique.
    /// * `params` - HashMap contenant les paramètres de la requête.
    
    pub fn new(item_key: String, params: HashMap<String, String>) -> Self {
        SqlDynamicQueryData {
            item_key,
            params,
        }
    }

    /// Crée une nouvelle instance de `SqlDynamicQueryData` avec des paramètres vides.
    ///
    /// # Arguments
    ///
    /// * `item_key` - Clé d'élément unique pour identifier la requête dynamique.
    ///
    pub fn empty(item_key: String) -> Self {
        SqlDynamicQueryData {
            item_key,
            params: HashMap::new(),
        }
    }

    /// Ajoute un paramètre à la requête dynamique.
    ///
    /// # Arguments
    ///
    /// * `key` - Nom du paramètre.
    /// * `value` - Valeur du paramètre.
    
    pub fn add_param(&mut self, key: String, value: String) {
        self.params.insert(key, value);
    }

    /// Récupère la valeur d'un paramètre par sa clé.
    ///
    /// # Arguments
    ///
    /// * `key` - Nom du paramètre à récupérer.
    ///
    /// # Retourne
    ///
    /// Une `Option<&String>` contenant la valeur du paramètre si elle existe.
    
    pub fn get_param(&self, key: &str) -> Option<&String> {
        self.params.get(key)
    }
}