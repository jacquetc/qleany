pub mod db_context;
pub mod transactions;
pub(crate) mod db_helpers;
use anyhow::Result;

pub trait CommandUnitOfWork {
    fn begin_transaction(&mut self) -> Result<()>;
    fn commit(&mut self) -> Result<()>;
    fn rollback(&mut self) -> Result<()>;
}

pub trait QueryUnitOfWork {
    fn begin_transaction(&self) -> Result<()>;
    fn end_transaction(&self) -> Result<()>;
}

use bincode::{deserialize, serialize};
use redb::Key;
use redb::{TypeName, Value};
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use std::any::type_name;
use std::cmp::Ordering;
use std::fmt::Debug;

/// Wrapper type to handle keys and values using bincode serialization
#[derive(Debug)]
pub struct Bincode<T>(pub T);

impl<T> Value for Bincode<T>
where
    T: Debug + Serialize + for<'a> Deserialize<'a>,
{
    type SelfType<'a>
        = T
    where
        Self: 'a;

    type AsBytes<'a>
        = Vec<u8>
    where
        Self: 'a;

    fn fixed_width() -> Option<usize> {
        None
    }

    fn from_bytes<'a>(data: &'a [u8]) -> Self::SelfType<'a>
    where
        Self: 'a,
    {
        deserialize(data).unwrap()
    }

    fn as_bytes<'a, 'b: 'a>(value: &'a Self::SelfType<'b>) -> Self::AsBytes<'a>
    where
        Self: 'a,
        Self: 'b,
    {
        serialize(value).unwrap()
    }

    fn type_name() -> TypeName {
        TypeName::new(&format!("Bincode<{}>", type_name::<T>()))
    }
}

impl<T> Key for Bincode<T>
where
    T: Debug + Serialize + DeserializeOwned + Ord,
{
    fn compare(data1: &[u8], data2: &[u8]) -> Ordering {
        Self::from_bytes(data1).cmp(&Self::from_bytes(data2))
    }
}
