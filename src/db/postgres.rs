
use crate::db::nested::service1;

pub fn connect() {
    println!("Connecting to PostgreSQL...");
    service1::transaction_in_connect();
}
