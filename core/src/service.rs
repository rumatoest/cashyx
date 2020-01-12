use crate::model::*;
use crate::storage::*;

pub struct CashyCore {
    storage: Box<dyn Storage>,
}

impl CashyCore {
    fn new(store: Box<dyn Storage>) -> CashyCore {
        CashyCore { storage: store }
    }

    fn currencies(&self) -> Vec<Currency> {
        unimplemented!()
        // self.storage.currencies().all()
    }

    fn categories(&self) -> Vec<RecordCategory> {
        unimplemented!()
        // self.storage.categories().all()
    }

    fn find_records(&self, year: usize) -> Vec<Record> {
        unimplemented!()
        // self.storage.records().all()
    }

    fn add_record(&mut self, record: Record) {
        unimplemented!()
        // self.storage.records().add(record);
    }
}
