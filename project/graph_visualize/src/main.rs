extern crate rasciigraph;
use rasciigraph::{plot, Config};

fn main() {
    let cities = vec!["Tokyo", "New York", "Dubai", "London", "Hong Kong"];
    let distance_travelled = vec![10, 20, 30, 40, 50];
    println!("{}",cities.join(">"));
    println!(
        "{}",
        plot(distance_travelled.into_iter().map(|d|d as f64).collect(),
        Config::default().with_offset(10)
        .with_height(10)
        .with_caption("Travelled distances (km)".to_string())));
    }
