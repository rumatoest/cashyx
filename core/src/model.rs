use chrono::prelude::*;

/// Record that can identify itself
pub trait Identifiable {
    fn id(&self) -> &String;
}

#[derive(Debug)]
pub struct Currency {
    primary: bool,
    code: String,
    name: String,
}

#[derive(Debug)]
pub struct Money {
    currency: Currency,
    amount: f64,
}

#[derive(Debug)]
pub struct Account {
    id: String,
    name: String,
    currency: String,
}

#[derive(Debug)]
pub struct RecordGroup {
    id: String,
    parent_id: String,
    name: String,
}

#[derive(Debug)]
pub struct RecordCategory {
    id: String,
    name: String,
}

#[derive(Debug)]
pub enum CashFlow {
    OUT,
    IN,
}

#[derive(Debug)]
pub struct Record {
    id: String,
    date: NaiveDate,
    amount: Money,
    flow: CashFlow,
    group: Option<RecordGroup>,
    category: Option<RecordCategory>,
    comment: String,
}

#[derive(Hash, Eq, Debug)]
pub struct RecordsPartition {
    year: usize,
    month: usize,
}

impl PartialEq for RecordsPartition {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year && self.month == other.month
    }
}
