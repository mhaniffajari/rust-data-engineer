use std::collections::HashMap;

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
fn main() {
    println!("{}",init_languanges());
}
