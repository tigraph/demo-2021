use std::fs::File;

use timatch_demo::n_degree_data_generator::{gen_relationships_records, gen_users_records};

fn main() {
    let users = File::create("./test.users.csv").unwrap();
    gen_users_records(csv::Writer::from_writer(users)).unwrap();
    let relationships = File::create("./test.relationships.csv").unwrap();
    gen_relationships_records(csv::Writer::from_writer(relationships)).unwrap();
}
