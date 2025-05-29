use rand::Rng;
use tabled::{
    Table, Tabled,
    settings::{Panel, Style},
};

pub fn random_hash(length: usize) -> String {
    let mut rng = rand::thread_rng();
    let mut hash = String::new();
    for _ in 0..length {
        hash.push(rng.gen_range(b'a'..=b'z') as char);
    }
    hash
}

pub fn get_test_db_path(name: &str) -> String {
    format!("tmp/test_{}.db", name)
}

pub fn print_table<T: Tabled>(data: Vec<T>, title: &str) {
    println!(
        "{}",
        Table::new(data)
            .with(Style::modern())
            .with(Panel::header(title))
    );
}
