use std::fs::File;

use timatch_demo::alter::{
    append_ddl, append_orders_dml, append_relationships_dml, append_users_dml,
};

fn main() {
    let file = File::create("./alter.sql").unwrap();
    append_ddl(&file).unwrap();
    append_orders_dml(&file).unwrap();
    append_users_dml(&file).unwrap();
    append_relationships_dml(&file).unwrap();
}
