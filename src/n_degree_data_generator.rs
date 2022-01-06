use std::fs::File;

use rand::distributions::Alphanumeric;
use rand::Rng;
use serde::Serialize;

use crate::{MAX_FRIENDS_COUNT, MIN_FRIENDS_COUNT};

const RECORD_COUNT: u64 = 1000000000;
const FLUSH_STEP: u64 = 1000;

#[derive(Debug, Serialize)]
struct UsersRecord {
    id: u64,
    name: String,
}

pub fn gen_users_records(mut writer: csv::Writer<File>) -> csv::Result<()> {
    for i in 0..RECORD_COUNT {
        let name: String = rand::thread_rng()
            .sample_iter(&Alphanumeric)
            .take(7)
            .map(char::from)
            .collect();
        let record = UsersRecord { id: i, name };
        writer.serialize(record)?;
        if (i + 1) % FLUSH_STEP == 0 {
            writer.flush()?;
        }
    }
    writer.flush()?;

    Ok(())
}

#[derive(Debug, Serialize)]
struct RelationshipsRecord {
    user_id: u64,
    friend_id: u64,
}

pub fn gen_relationships_records(mut writer: csv::Writer<File>) -> csv::Result<()> {
    let mut rng = rand::thread_rng();

    let mut count = 0;
    let mut i = 0;
    while i < RECORD_COUNT && count < RECORD_COUNT {
        let mut friends_count = rng.gen_range(MIN_FRIENDS_COUNT..MAX_FRIENDS_COUNT);
        while friends_count > 0 {
            let friend_id = rng.gen_range(0..RECORD_COUNT);
            if friend_id == i {
                continue;
            }
            let record = RelationshipsRecord {
                user_id: i,
                friend_id,
            };
            writer.serialize(record)?;
            count += 1;
            friends_count -= 1;
            if (count + 1) % FLUSH_STEP == 0 {
                writer.flush()?;
            }
        }
        i += 1;
    }
    writer.flush()?;

    Ok(())
}
