// Entrega 2: tipos y estructuras de datos

use std::collections::HashMap;

fn main() {
    // concurencia de una letra
    let mut concurrence: HashMap<char, u32> = HashMap::new();

    // declaración de string
    let random_string : String = String::from("abbcccdddd");

    for item in random_string.chars() {

        // si está lo actualiza, si no lo inicializa
        if !concurrence.contains_key(&item) {
            concurrence.insert(item, 1);
        } else {
            *concurrence.get_mut(&item).unwrap() += 1;
        }
    }

    // imprimer los resultados
    for (key, value) in &concurrence {
        println!("{key} - {value}", key=key, value=value);
    }
}