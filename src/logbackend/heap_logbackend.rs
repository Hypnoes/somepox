//! # Heap Based Log Backend.
//! Use Heap Memory to store log messages.
//!
//! **Details** :
//! 1. queriable
//! 2. indexed
//! 3. ordered
//! 4. versioned
//!
//! data memory map:
//!
//!     `RefCell<BTreeMap<u64, RefCell<LinkedList<Bytes>>>>>`,
//!
//! 1. refcell ➡️ the log items is growable
//! 2. btreemap ➡️ log can be queried by index id
//! 3. refcell ➡️ version can be bumped
//! 4. linkedlist ➡️ last item indecated the last version of the log content
//!
//!
#![allow(unused)]
use anyhow::{anyhow, Result};
use bytes::Bytes;
use std::{
    cell::RefCell,
    collections::{BTreeMap, LinkedList},
};

use super::{Queryable, Writable};

pub struct HeapLogBackend {
    table: RefCell<BTreeMap<u64, RefCell<LinkedList<Bytes>>>>,
}

impl HeapLogBackend {
    pub fn new() -> HeapLogBackend {
        HeapLogBackend {
            table: RefCell::new(BTreeMap::new()),
        }
    }
}

impl Writable for HeapLogBackend {
    fn write(&self, id: u64, data: Bytes) -> Result<()> {
        let mut table_ref = self.table.try_borrow_mut()?;

        match table_ref.get(&id) {
            Some(version_his_ref) => Ok(version_his_ref.borrow_mut().push_back(data.clone())),
            None => {
                table_ref.insert(id, RefCell::new(LinkedList::from([data.clone()])));
                Ok(())
            }
        }
    }
}

impl Queryable for HeapLogBackend {
    fn query(&self, id: u64) -> Result<Bytes> {
        let table_ref = self.table.try_borrow()?;

        match table_ref.get(&id) {
            Some(version_his_ref) => version_his_ref
                .borrow()
                .back()
                .map_or(Err(anyhow!("Item Not Found.")), |v| Ok(v.clone())),
            None => Err(anyhow!("Item Not Found.")),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::logbackend::HeapLogBackend;
    use crate::logbackend::Writable;
    use bytes::Bytes;
    use std::collections::LinkedList;
    use std::fmt::{Display, Formatter, Result as FmtResult};

    impl Display for HeapLogBackend {
        fn fmt(&self, f: &mut Formatter<'_>) -> FmtResult {
            fn format_list(list: &LinkedList<Bytes>) -> String {
                let result = String::from("[");

                let content: String = list
                    .iter()
                    .flat_map(|item| String::from_utf8((&item).to_vec()))
                    .map(|str| str + ",")
                    .collect();

                let content_len = content.len();

                result + &content[0..(content_len - 1)] + "]"
            }

            let table_view: Vec<String> = self
                .table
                .borrow()
                .iter()
                .map(|(id, version_his)| format!("{}: {}", id, format_list(&version_his.borrow())))
                .collect();

            write!(f, "{}", table_view.join("\n"))
        }
    }

    #[test]
    fn heap_logbackend_new_test() {
        let test_backend = HeapLogBackend::new();
        assert_eq!(test_backend.table.borrow().len(), 0, "test_backend table is not empty");
    }

    #[test]
    fn heap_logbackend_write_test() {
        let test_backend = HeapLogBackend::new();

        let r1 = test_backend.write(1, "test1".into());
        assert!(r1.is_ok(), "Error in write log into heap backend");

        print!("{}", test_backend.to_string());
        println!("\n-------------");

        let r2 = test_backend.write(2, "test2".into());
        assert!(r2.is_ok(), "Error in write log into heap backend");

        print!("{}", test_backend.to_string());
        println!("\n-------------");

        let r3 = test_backend.write(1, "test1.1".into());
        assert!(r3.is_ok(), "Error in write log into heap backend");

        print!("{}", test_backend.to_string());
        println!("\n-------------");

        assert!(true, "Error in write log into heap backend");
    }
}
