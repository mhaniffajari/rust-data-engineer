/*
linkedin list higher memory usage
not best choice unless have specific needs
*/
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::thread_rng;
use std::collections::LinkedList;

fn main() {
    let mut fruit: LinkedList<&str> = LinkedList::new();
    fruit.push_back("Apple");
    fruit.push_back("Mango");
    let mut rng: ThreadRng = thread_rng();

    let mut fruit: Vec<_> = fruit.into_iter().collect::<Vec<_>>();
    fruit.shuffle(&mut rng);
    /*
    converted Vec to LinkedList
     */
    let mut fruit: LinkedList<_> = fruit.into_iter().collect();
    fruit.push_back("Banana");
    fruit.push_back("Fig");

    println!("Fruit Salad:");
    for (i, item) in fruit.iter().enumerate() {
        if i != fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            print!("{}", item)
        }
    }
}
