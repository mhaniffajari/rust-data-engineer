use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::HashSet;

fn generate_fruit() -> &'static str 
{
    let fruits = 
    [
        "Apple",
        "Banana",
        "Cherry",
        "Date",
        "Elderberry",
        "Fig",
        "Blackberry",
        "Strawberry",
        "Raspberry",
        "Honeydew",
    ];
    let mut rng = thread_rng();
    fruits.choose(&mut rng).unwrap()
}
fn main()
{
    let num_fruit = 10;
    let mut fruit_set = HashSet::new();
    print!("Generating {} random fruits....",num_fruit);
    for _  in 0..num_fruit
    {
        fruit_set.insert(generate_fruit());
    }
    println!("Number of unique fruits: {:?}", fruit_set.len());
}