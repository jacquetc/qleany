use redb::{ReadTransaction, WriteTransaction};
use crate::database::db_context::DbContext;
use anyhow::Result;


enum TransactionType {
    Read(Option<ReadTransaction>),
    Write(Option<WriteTransaction>),
}

pub struct Transaction {
    transaction: TransactionType,
}

impl Transaction {
    pub fn begin_write_transaction(db_context: &DbContext) -> Result<Transaction> {
        let transaction = db_context.get_database().begin_write()?;
        Ok(Transaction {
            transaction: TransactionType::Write(Some(transaction)),
        })
    }

    pub fn begin_read_transaction(db_context: &DbContext) -> Result<Transaction> {
        let transaction = db_context.get_database().begin_read()?;
        Ok(Transaction {
            transaction: TransactionType::Read(Some(transaction)),
        })
    }
    pub fn commit(&mut self) -> Result<()> {
        match &mut self.transaction {
            TransactionType::Read(_) => Ok(()),
            TransactionType::Write(transaction_option) => transaction_option.take().unwrap().commit(),
        }?;
        Ok(())
    }

    pub fn rollback(&mut self) -> Result<()> {
        match &mut self.transaction {
            TransactionType::Read(_) => Ok(()),
            TransactionType::Write(transaction_option) => transaction_option.take().unwrap().abort(),
        }?;
        Ok(())
    }

    pub fn end_read_transaction(&mut self) -> Result<()> {
        match &mut self.transaction {
            TransactionType::Read(transaction_option) => {
                transaction_option.take().unwrap().close()?;
                Ok(())
            }
            TransactionType::Write(_) => Ok(()),
        }
    }

    pub(crate) fn get_read_transaction(&self) -> &ReadTransaction {
        match &self.transaction {
            TransactionType::Read(Some(transaction)) => transaction,
            _ => panic!("Transaction is not a read transaction"),
        }
    }

    pub(crate) fn get_write_transaction(&self) -> &WriteTransaction {
        match &self.transaction {
            TransactionType::Write(Some(transaction)) => transaction,
            _ => panic!("Transaction is not a write transaction"),
        }
    }
}


