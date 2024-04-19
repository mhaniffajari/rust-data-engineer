use clap::Parser;
use salad_cli::create_fruit_salad;


fn main() {
    println!("Hello, world!");
}

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Salad CLI",
    about = "A CLI for creating fruit salads"
)]

struct Opts {
    #[clap(short,long)]
    number: usize
}

fn main() {
    let opts = Opts::parse();
    //Get number of fruits the user requested
    let num_fruits = opts.number;
    //Create a fruit salad
    create_fruit_salad(num_fruits);
    //Print the fruit salad
    println!(
        "Create fruit salad with {} fruits {?}"
        num_fruits,create_fruit_salad(num_fruits)
    );
}
