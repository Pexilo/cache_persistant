use std::collections::{HashMap, VecDeque};
use std::fs;
use std::io::{self, Write};
use std::path::Path;

/// définit les fonctionnalités du cache
pub trait CacheTrait<K, V> {
    fn get(&mut self, key: &K) -> Option<&V>; // récupère une valeur pour une clé donnée
    fn put(&mut self, key: K, value: V); // ajoute ou met à jour une paire clé-valeur dans le cache
}

/// cache LRU (Least Recently Used)
pub struct Cache<K, V> {
    capacity: usize,
    map: HashMap<K, V>,
    usage_order: VecDeque<K>,
}

impl<K: Eq + std::hash::Hash + Clone, V> Cache<K, V> {
    /// crée un nouveau cache avec une capacité maximale
    ///
    /// # Arguments
    ///
    /// * `capacity` - nombre maximal d'éléments dans le cache
    ///
    /// # Retour
    ///
    /// cache vide avec la capacité donnée
    pub fn new(capacity: usize) -> Self {
        Cache {
            capacity,
            map: HashMap::new(),
            usage_order: VecDeque::new(),
        }
    }

    /// crée un cache persistant en chargeant les données du fichier donnée
    ///
    /// # Arguments
    ///
    /// * `capacity` - capacité maximale du cache
    /// * `filename` - nom du fichier contenant les données
    ///
    /// # Retour
    ///
    /// cache avec les données chargées du fichier
    pub fn new_persistent(capacity: usize, filename: &str) -> Self
    where
        K: std::fmt::Display + std::str::FromStr,
        V: std::fmt::Display + std::str::FromStr,
    {
        let mut cache = Cache::new(capacity);
        cache.load_from_file(filename).unwrap_or_else(|_| ());
        cache
    }

    /// sauvegarde le contenu du cache dans un fichier
    ///
    /// # Arguments
    ///
    /// * `filename` - nom du fichier où les données seront écrites
    pub fn save_to_file(&self, filename: &str)
    where
        K: std::fmt::Display,
        V: std::fmt::Display,
    {
        let path = Path::new(filename);
        let mut file = fs::File::create(path).expect("Impossible de créer le fichier");
        for (key, value) in &self.map {
            writeln!(file, "{} ➡️ {}", key, value).expect("Impossible d'écrire dans le fichier");
        }
    }

    /// charge les données d'un fichier et les insère dans le cache
    ///
    /// # Arguments
    ///
    /// * `filename` - nom du fichier à lire
    ///
    /// # Retour
    ///
    /// instance `io::Result` représentant le succès ou l'échec
    pub fn load_from_file(&mut self, filename: &str) -> io::Result<()>
    where
        K: std::str::FromStr,
        V: std::str::FromStr,
    {
        let path = Path::new(filename);
        if !path.exists() {
            return Ok(());
        }
        let content = fs::read_to_string(path)?;
        for line in content.lines() {
            if let Some((key_str, value_str)) = line.split_once(':') {
                if let (Ok(k), Ok(v)) = (key_str.parse::<K>(), value_str.parse::<V>()) {
                    self.put(k, v);
                }
            }
        }
        Ok(())
    }
}

impl<K: Eq + std::hash::Hash + Clone, V> CacheTrait<K, V> for Cache<K, V> {
    /// récupère une valeur pour une clé donnée
    ///
    /// met à jour l'ordre d'utilisation pour la clé donnée
    ///
    /// # Arguments
    ///
    /// * `key` - clé pour laquelle récupérer la valeur
    ///
    /// # Retour
    ///
    ///  option contenant la valeur si elle est trouvée, ou `None` sinon
    fn get(&mut self, key: &K) -> Option<&V> {
        if self.map.contains_key(key) {
            self.usage_order.retain(|k| k != key);
            self.usage_order.push_back(key.clone());
            self.map.get(key)
        } else {
            None
        }
    }

    /// ajoute ou met à jour une paire clé-valeur dans le cache
    ///
    /// si la clé existe déjà, elle est mise à jour avec la nouvelle valeur
    /// si le cache est plein, l'élément le moins récemment utilisé est supprimé
    ///
    /// # Arguments
    ///
    /// * `key` - clé à ajouter ou mettre à jour
    /// * `value` - valeur associée à la clé
    fn put(&mut self, key: K, value: V) {
        if self.map.contains_key(&key) {
            self.map.insert(key.clone(), value);
            self.usage_order.retain(|k| k != &key);
            self.usage_order.push_back(key);
        } else {
            if self.map.len() == self.capacity {
                if let Some(lru_key) = self.usage_order.pop_front() {
                    self.map.remove(&lru_key);
                }
            }
            self.map.insert(key.clone(), value);
            self.usage_order.push_back(key);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lru_cache() {
        let mut cache = Cache::new(3); // Taille de 3
        cache.put("A", String::from("value_a"));
        cache.put("B", String::from("value_b"));
        cache.put("C", String::from("value_c"));
        cache.put("D", String::from("value_d"));
        // Cache == [B, C, D]

        let my_value = cache.get(&"A");
        assert_eq!(my_value, None);
        let my_value = cache.get(&"D");
        assert_eq!(my_value, Some(&String::from("value_d")));
        // Cache == [B, C, D]

        let my_value = cache.get(&"B");
        assert_eq!(my_value, Some(&String::from("value_b")));
        // Cache == [C, D, B]

        let my_value = cache.get(&"C");
        assert_eq!(my_value, Some(&String::from("value_c")));
        // Cache == [D, B, C]

        let my_value = cache.get(&"X");
        assert_eq!(my_value, None);
        // Cache == [D, B, C]

        cache.put("A", String::from("value_a"));
        // Cache == [B, C, A]

        cache.put("X", String::from("value_x"));
        // Cache == [C, A, X]

        let my_value = cache.get(&"B");
        assert_eq!(my_value, None);
        // Cache == [C, A, X]

        let my_value = cache.get(&"D");
        // Cache == [C, A, X]
        assert_eq!(my_value, None);
    }
}
