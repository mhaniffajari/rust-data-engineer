use std::collections::VecDeque;

fn main() {
    let mut fruit: VecDeque<&str> = VecDeque::new();
    fruit.push_back("Arbutus");
    fruit.push_back("Fig");
    fruit.push_back("Apple");
    fruit.push_back("Banana");
    fruit.push_back("Mango");

    println!("Fruit Salad:");
    for (i,item) in fruit.iter().enumerate() {
        if i !=fruit.len() - 1 {
            print!("{}, ", item);
        } else {
            println!("{}", item);
        }
    }
}
