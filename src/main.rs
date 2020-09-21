
#[macro_use]
extern crate diesel_migrations;

use diesel_sqlite_example::models::*;

use diesel::{Connection, SqliteConnection, RunQueryDsl};
use diesel::{dsl::sql, prelude::*, sql_types::{Nullable,Double}};
use std::time::{SystemTime, UNIX_EPOCH};

embed_migrations!("migrations/");

fn connect() -> SqliteConnection {
    SqliteConnection::establish("report.db")
        .unwrap_or_else(|_| panic!("Error connecting to report.db"))
}

fn do_migrate(){
    let con = connect();
    let _re = embedded_migrations::run_with_output(&con, &mut std::io::stdout());
}


fn do_insert_or_update(rs: &Transaction) {

    
    use diesel_sqlite_example::schema::Transactions::dsl::*;

    let connection = connect();

    match diesel::replace_into(Transactions)
        .values(rs)
        .execute(&connection) {
            Ok(_) => println!("Inserted"),
            Err(e) => println!("Error {:?}", e)
        };
        
}


fn do_insert(rs: &Transaction) {

    
    use diesel_sqlite_example::schema::Transactions::dsl::*;

    let connection = connect();

    match diesel::insert_into(Transactions)
        .values(rs)
        .execute(&connection) {
            Ok(_) => println!("Inserted"),
            Err(e) => println!("Error {:?}", e)
        };
}

fn get_last() {
    use diesel_sqlite_example::schema::Transactions::dsl::*;

    let connection = connect();

    let results = Transactions.filter(sequence.ge(0))
        .order(id.desc()) 
        .limit(1)
        .load::<Transaction>(&connection);

    for tran in results {
        println!("{:?}", tran);
    }
}

fn groupby_sum() {
    use diesel_sqlite_example::schema::*;

    let connection = connect();

    let results = Transactions::table
        .group_by(Transactions::tranType)
        .filter(Transactions::sequence.ge(0))
        .select((sql::<Nullable<Double>>("sum(amount) AS sum"), Transactions::tranType))
        .load::<(Option<f64>, Option<i32>)>(&connection)
        .expect("Error loading transactions");
    
    
    for tran in results {
        println!("{:?}", tran);
    }
}

fn main() {


    do_migrate();

    let start = SystemTime::now();
    let since_the_epoch = start
        .duration_since(UNIX_EPOCH)
        .expect("Time went backwards");
        
    let tran1 = Transaction {
        id: None,
        sequence: Some(1),
        tranType: Some(1),
        dateTime: Some(since_the_epoch.as_secs_f64()),
        amount: Some(1000.0),
    };

    let tran2 = Transaction {
        id: None,
        sequence: Some(1),
        tranType: Some(1),
        dateTime: Some(since_the_epoch.as_secs_f64()),
        amount: Some(2000.0),
    };

    let tran3 = Transaction {
        id: None,
        sequence: Some(2),
        tranType: Some(1),
        dateTime: Some(since_the_epoch.as_secs_f64()),
        amount: Some(1000.0),
    };


    let tran4 = Transaction {
        id: None,
        sequence: Some(1),
        tranType: Some(2),
        dateTime: Some(since_the_epoch.as_secs_f64()),
        amount: Some(2000.0),
    };

    do_insert(&tran1);
    do_insert_or_update(&tran2);
    do_insert_or_update(&tran3);
    do_insert_or_update(&tran4);
    get_last();

    groupby_sum()
}
