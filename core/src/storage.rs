use std::collections::HashMap;
use std::error::Error;
use std::fmt::Debug;
use std::fs::File;

use csv::ReaderBuilder;
use serde::{de::DeserializeOwned, Deserialize, Serialize};

use crate::model::{Identifiable, RecordsPartition};

#[derive(Serialize, Deserialize, Debug)]
pub struct CurrencyRow {
    primary: bool,
    code: String,
    name: String,
}

impl Identifiable for CurrencyRow {
    fn id(&self) -> &String {
        &self.code
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct AccountRow {
    id: String,
    name: String,
    currency: String,
}

impl Identifiable for AccountRow {
    fn id(&self) -> &String {
        &self.id
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct GroupRow {
    id: String,
    parent_id: String,
    name: String,
}

impl Identifiable for GroupRow {
    fn id(&self) -> &String {
        &self.id
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct CategoryRow {
    id: String,
    name: String,
}

impl Identifiable for CategoryRow {
    fn id(&self) -> &String {
        &self.id
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct RecordRow {
    id: String,
    date: String,
    time: String,
    location: String,
    currency: String,
    amount: String,
    flow: String,
    account: String,
    group: String,
    category: String,
    info: String,
    meta: String,
}

impl Identifiable for RecordRow {
    fn id(&self) -> &String {
        &self.id
    }
}

pub trait StorageEntity {
    type VALUE;

    fn all(&self) -> &Vec<Self::VALUE>;
    fn add(&mut self, record: Self::VALUE);
    fn load(&mut self) -> Result<(), Box<dyn Error>>;
    fn save_all(&mut self) -> Result<(), Box<dyn Error>>;
}

pub trait Storage {
    fn currencies(&self) -> Box<dyn CurrenciesStorage>;

    fn accounts(&self) -> Box<dyn AccountsStorage>;

    fn groups(&self) -> Box<dyn GroupsStorage>;

    fn categories(&self) -> Box<dyn CategoriesStorage>;

    fn records(&self) -> Box<dyn RecordsStorage>;

    fn save_all(&mut self) {
        self.currencies().save_all().expect("Save currencies");
    }
}

pub trait CurrenciesStorage: StorageEntity<VALUE = CurrencyRow> {}

pub trait AccountsStorage: StorageEntity<VALUE = AccountRow> {}

pub trait GroupsStorage: StorageEntity<VALUE = GroupRow> {}

pub trait CategoriesStorage: StorageEntity<VALUE = CategoryRow> {}

pub trait RecordsStorage: StorageEntity<VALUE = RecordRow> {}

struct CsvStorageEntity<R: Identifiable> {
    uri: String,
    data: Vec<R>,
}

impl<R: Identifiable> CsvStorageEntity<R> {
    fn reader(&self) -> csv::Result<csv::Reader<File>> {
        ReaderBuilder::new()
            .has_headers(true)
            .from_path(self.uri.clone())
    }
}

impl<R: DeserializeOwned + Serialize + Identifiable> StorageEntity for CsvStorageEntity<R> {
    type VALUE = R;

    fn all(&self) -> &Vec<Self::VALUE> {
        &self.data
    }

    fn add(&mut self, record: Self::VALUE) {
        self.data.push(record)
    }

    fn load(&mut self) -> Result<(), Box<dyn Error>> {
        let mut rdr = self.reader()?;
        let mut dt = Vec::<Self::VALUE>::new();
        for result in rdr.deserialize() {
            let r = result?;
            dt.push(r)
            // println!("{:?}", record);
        }
        self.data = dt;
        Ok(())
    }

    fn save_all(&mut self) -> Result<(), Box<dyn Error>> {
        Ok(())
    }
}

struct Records {
    updated: bool,
    records: CsvStorageEntity<RecordRow>,
}

struct CsvStorageRecords {
    storage: HashMap<RecordsPartition, Records>,
}

pub struct CsvStorage {
    currencies: CsvStorageEntity<CurrencyRow>,

    accounts: CsvStorageEntity<AccountRow>,

    groups: CsvStorageEntity<GroupRow>,

    categories: CsvStorageEntity<CategoryRow>,
}
/*
impl Storage for CsvStorage {
    fn currencies(&self) -> Box<dyn CurrenciesStorage> {}

    fn accounts(&self) -> Box<dyn AccountsStorage> {}

    fn groups(&self) -> Box<dyn GroupsStorage> {}

    fn categories(&self) -> Box<dyn CategoriesStorage> {}

    fn records(&self) -> Box<dyn RecordsStorage> {}
}
 */

#[cfg(test)]
mod test {
    use super::*;
    use std::path::PathBuf;

    #[test]
    fn currencies_load() {
        let mut pth = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
        pth.push("test/currencies.csv");

        let mut store = CsvStorageEntity::<CurrencyRow> {
            uri: String::from(pth.to_str().expect("Path")),
            data: Vec::new(),
        };

        store.load().expect("Should load");
        let all = store.all();
        assert_eq!(all.len(), 4);

        assert_eq!(all[0].primary, true);
        assert_eq!(all[1].primary, false);
        assert_eq!(all[2].name, "Dollar");
        assert_eq!(all[3].code, "CNY");
    }
}
