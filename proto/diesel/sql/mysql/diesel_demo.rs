use crate::SQL_ROOT;

use diesel::prelude::*;
use diesel::Connection;
use diesel::mysql::MysqlConnection as SQLConnection;


use crate::sql::models::*;
use crate::sql::schema::Users::table;

fn establish_connection() -> SQLConnection {
    let database_url = SQL_ROOT.clone();
    SQLConnection::establish(&database_url)
        .expect(&format!("Error connecting to db"))
}

pub fn demo() {

    let connection = establish_connection();
    let results = User.filter()
        .limit(5)
        .load::<User>(&connection)
        .expect("Error loading posts");

    println!("Displaying {} users", results.len());
    for post in results {
        println!("{}", post.name);
        println!("----------\n");
        println!("{}", post.pubkey);
    }
}