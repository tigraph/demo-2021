use std::fs::File;
use std::io::{Result, Write};

use rand::distributions::Alphanumeric;
use rand::Rng;

const ROW_COUNT: i64 = 10000;
const STEP: i64 = 1000;
const MIN_FRIENDS_COUNT: i64 = 3;
const MAX_FRIENDS_COUNT: i64 = 10;

pub fn append_ddl(mut writer: &File) -> Result<()> {
    let all_tables_ddl = r"
USE test;

CREATE TABLE orders
(
id      BIGINT PRIMARY KEY AUTO_INCREMENT,
user_id BIGINT,
item_id BIGINT
);

CREATE TABLE users
(
id   BIGINT PRIMARY KEY,
name VARCHAR(64)
);

CREATE TABLE relationships
(
user_id   BIGINT,
friend_id BIGINT
);

";

    writer.write_all(all_tables_ddl.as_bytes())?;

    Ok(())
}

pub fn append_orders_dml(mut writer: &File) -> Result<()> {
    let mut rng = rand::thread_rng();

    let mut cols = String::new();
    for i in 0..ROW_COUNT {
        let value = &format!(" ({}, {}),", i, rng.gen_range(0..STEP));
        cols.push_str(value);

        if (i + 1) % STEP == 0 {
            cols.pop();
            writer.write_all(
                format!("insert into orders(user_id,item_id) values{};\n\n", cols).as_bytes(),
            )?;
            cols.clear();
        }
    }

    Ok(())
}

pub fn append_users_dml(mut writer: &File) -> Result<()> {
    let mut cols = String::new();
    for i in 0..ROW_COUNT {
        let name: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();

        let value = &format!(" ({}, '{}'),", i, name);
        cols.push_str(value);

        if (i + 1) % STEP == 0 {
            cols.pop();
            writer.write_all(format!("insert into users values{};\n\n", cols).as_bytes())?;
            cols.clear();
        }
    }

    Ok(())
}

pub fn append_relationships_dml(mut writer: &File) -> Result<()> {
    let mut rng = rand::thread_rng();

    let mut cols = String::new();
    let mut count = 0;
    for i in 0..ROW_COUNT {
        let mut friends_count = rng.gen_range(MIN_FRIENDS_COUNT..MAX_FRIENDS_COUNT);
        while friends_count > 0 {
            let to = rng.gen_range(0..ROW_COUNT);
            if to == i {
                continue;
            }
            let value = &format!(" ({}, {}),", i, to);
            cols.push_str(value);
            count += 1;
            friends_count -= 1;

            if (count + 1) % STEP == 0 {
                cols.pop();
                writer.write_all(
                    format!("insert into relationships values{};\n\n", cols).as_bytes(),
                )?;
                cols.clear();
            }
        }
    }

    if !cols.is_empty() {
        cols.pop();
        writer.write_all(format!("insert into relationships values{};\n\n", cols).as_bytes())?;
    }

    Ok(())
}
