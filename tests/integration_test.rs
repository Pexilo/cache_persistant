use cache_exam::{Cache, CacheTrait};
use std::fs;

#[test]
fn test_persistent_cache_integration() {
    let filename = "integration_test_cache.txt";

    // créer et sauvegarder un cache de persistance 2
    {
        let mut cache = Cache::new_persistent(2, filename);
        cache.put("A".to_string(), "val_A".to_string());
        cache.put("B".to_string(), "val_B".to_string());
        cache.put("C".to_string(), "val_C".to_string()); // devrait supprimer A (évincer)
        cache.save_to_file(filename);
    }

    // charger le cache et vérifier les valeurs
    {
        let mut cache = Cache::new_persistent(2, filename);
        assert_eq!(cache.get(&"A".to_string()), None);
        assert_eq!(cache.get(&"B".to_string()), Some(&"val_B".to_string()));
        assert_eq!(cache.get(&"C".to_string()), Some(&"val_C".to_string()));
    }

    // Nettoyage
    let _ = fs::remove_file(filename);
}
