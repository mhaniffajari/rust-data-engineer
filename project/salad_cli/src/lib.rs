use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn create_fruit_salad(num_fruits:usize) -> Vec<String> {
    let fruits: Vec<String> = vec![
        "Apple".to_string(),
        "Mango".to_string(),
        "Banana".to_string(),
        "Fig".to_string(),
        "Grapes".to_string(),
        "Kiwi".to_string(),
        "Orange".to_string(),
        "Peach".to_string(),
        "Pear".to_string(),
        "Pineapple".to_string(),
        "Plum".to_string(),
        "Strawberry".to_string(),
        "Watermelon".to_string(),
    ];
    let mut rng= thread_rng();
    let mut fruits = fruits;
    fruits.shuffle(&mut rng);
    fruits.into_iter().take(num_fruits).collect()


}