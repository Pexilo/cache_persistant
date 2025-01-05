use cache_exam::{Cache, CacheTrait};

fn main() {
    println!("Bonjour Benjamin,\nBonne correction !");
    println!("-------------------------");

    let filename = "cache_data.txt";

    // crée cache persistant avec capacité de 3 éléments
    let mut cache = Cache::new_persistent(3, filename);

    // ajoute des éléments au cache
    cache.put("A".to_string(), "value_a".to_string());
    cache.put("B".to_string(), "value_b".to_string());
    cache.put("C".to_string(), "value_c".to_string());
    cache.put("D".to_string(), "value_d".to_string()); // devrait supprimer A

    // sauvegarde le cache dans un fichier
    cache.save_to_file(filename);

    println!("Cache après sauvegarde");
    println!("A ➡️  {:?}", cache.get(&"A".to_string())); // devrait être None
    println!("B ➡️  {:?}", cache.get(&"B".to_string()));
    println!("C ➡️  {:?}", cache.get(&"C".to_string()));
    println!("D ➡️  {:?}", cache.get(&"D".to_string()));

    println!("-------------------------");
    println!("Pour tester le code\n> cargo test");
    println!("-------------------------");
    println!("Pour mesurer les performances des opérations 'get' du cache, j'ai mis en place Criterion,\n> cargo bench\nHtml d'analyses dans target\\criterion");
    println!("-------------------------");
    println!("Pour générer la doc\n> cargo doc --open");
    println!("-------------------------");
}
