use commits_stats::{commits_per_author, commits_per_week};
pub use std::fs;
fn main() {
    let contents = fs::read_to_string("commits.json").unwrap();
    let serialized = json::parse(&contents).unwrap();
    println!("{:?}", commits_per_week(&serialized));
    println!("{:?}", commits_per_author(&serialized));
}
