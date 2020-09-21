use super::schema::*;

#[derive(Queryable,Insertable,Debug)]
#[table_name = "Transactions"]
pub struct Transaction {
    pub id: Option<i32>,
    pub sequence: Option<i32>,
    pub tranType : Option<i32>,
    pub dateTime : Option<f64>,
    pub amount : Option<f64>,
}