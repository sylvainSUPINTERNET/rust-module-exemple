
// Importer le module `db` qui se trouve dans le fichier `db/mod.rs`
mod db;


// static LANGUAGE: &str = "Rust";



// enum VeryVerboseEnumOfThingsToDoWithNumbers {
//     Add,
//     Subtract,
// }


// impl VeryVerboseEnumOfThingsToDoWithNumbers {
//     fn run(&self, x:i32) -> i32 {
//         match self {
//             Self::Add => x + 1,
//             Self::Subtract => x - 1,
//         }
//     }
// }

fn main() {
    db::postgres::connect();
    db::nested::service1::transaction();
    // let x = VeryVerboseEnumOfThingsToDoWithNumbers::Add;
    // println!("{}", x.run(5));

    // println!("{}", LANGUAGE)
}


// enum Pogger {
//     KeyPress(char) // tuple struct
// }

// #[derive(Debug)]
// struct People {
//     name: String,
//     age: u8
// }


// fn main() {
//     let ppl: People = People { name: String::from("Sylvain"), age: 26 };
//     println!("{:?}", ppl)


// }
