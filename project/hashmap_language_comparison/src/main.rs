use std::{collections::HashMap, i32};

fn init_languanges() -> HashMap<String,i32>{
    let mut languages = HashMap::new();
    languages.insert("JavaScript".to_string(), 1995);
    languages.insert("HTML/CSS".to_string(), 1990);
    languages.insert("Python".to_string(), 1991);
    languages.insert("Java".to_string(), 1995);
    languages.insert("C#".to_string(),2000);
    languages.insert("PHP".to_string(), 1995);

    languages
}
fn calcualte_weight(years_active: &mut HashMap<String, i32>) -> HashMap<String,i32>
{
    for year in years_active.values_mut(){
        *year = 2024-*year;
}
    let min_year = years_active.values().min().unwrap();
    let max_year = years_active.values().max().unwrap();
    let mut weights = HashMap::new();
    for (language, &year) in years_active.iter(){
        let normalized_year = (year - min_year) as f64 / (max_year - min_year) as f64;
        let weight = (normalized_year * 99.0) as i32 + 1 ;
        weights.insert(language.to_string(), weight);
    }
    weights
}

fn main() {
    let mut languages = init_languanges();
    let weights = calcualte_weight(&mut languages);
    println!("Language weihing from 1-100 by age(1 is newest, 100 is oldest:");
    for (language, weight) in weights.iter(){
        println!("{}: {}", language, weight);
    }
}
