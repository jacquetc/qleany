use anyhow::Result;
use common::database::{db_context::DbContext, transactions::Transaction};
use common::types::Savepoint;
use redb::{Database, ReadableTable, TableDefinition};
use std::sync::Arc;

// Define a test table
const TEST_TABLE: TableDefinition<&str, u64> = TableDefinition::new("test_table");

// Helper function to create a table and insert initial data
fn setup_database(db: &Database) -> Result<()> {
    let write_txn = db.begin_write()?;
    {
        let mut table = write_txn.open_table(TEST_TABLE)?;
        table.insert("key1", 1)?;
        table.insert("key2", 2)?;
    }
    write_txn.commit()?;
    Ok(())
}

// Helper function to read data from the database
fn read_value(db: &Database, key: &str) -> Result<Option<u64>> {
    let read_txn = db.begin_read()?;
    let table = read_txn.open_table(TEST_TABLE)?;
    let value = table.get(key)?.map(|v| v.value());
    Ok(value)
}

#[test]
fn test_restore_savepoint() -> Result<()> {
    // Create a DbContext with an in-memory database
    let db_context = DbContext::new()?;
    let db = db_context.get_database();

    // Setup the database with initial data
    setup_database(db)?;

    // Verify initial state
    assert_eq!(read_value(db, "key1")?, Some(1));
    assert_eq!(read_value(db, "key2")?, Some(2));
    assert_eq!(read_value(db, "key3")?, None);

    let mut savepoint: Option<Savepoint> = None;
    // Create a transaction and modify the database

    let mut write_txn = db.begin_write()?;

    // Create a savepoint before modifications
    savepoint = Some(write_txn.persistent_savepoint()?);

    // Commit the transaction
    write_txn.commit()?;

    // Modify the database

    let mut write_txn = db.begin_write()?;
    let mut table = write_txn.open_table(TEST_TABLE)?;
    table.insert("key1", 100)?; // Update existing key
    table.insert("key3", 3)?; // Add new key
    drop(table); // Release the table before starting a new transaction

    write_txn.commit()?;
    // Verify changes are present (using a separate read transaction)

    let read_txn = db.begin_read()?;
    let table = read_txn.open_table(TEST_TABLE)?;
    assert_eq!(table.get("key1")?.map(|v| v.value()), Some(100));
    assert_eq!(table.get("key3")?.map(|v| v.value()), Some(3));
    drop(read_txn);

    let mut write_txn = db.begin_write()?;
    // Restore to the savepoint
    let redb_savepoint = write_txn.get_persistent_savepoint(savepoint.unwrap())?;
    write_txn.restore_savepoint(&redb_savepoint)?;

    // Commit the transaction
    write_txn.commit()?;

    // Verify that changes were rolled back
    assert_eq!(read_value(db, "key1")?, Some(1)); // Should be back to original value
    assert_eq!(read_value(db, "key2")?, Some(2)); // Should remain unchanged
    assert_eq!(read_value(db, "key3")?, None); // Should not exist

    Ok(())
}

#[test]
fn test_get_persistent_savepoint() -> Result<()> {
    // Create a DbContext with an in-memory database
    let db_context = DbContext::new()?;
    let db = db_context.get_database();

    // Setup the database with initial data
    {
        let write_txn = db.begin_write()?;
        {
            let mut table = write_txn.open_table(TEST_TABLE)?;
            table.insert("key1", 1)?;
        }
        write_txn.commit()?;
    }

    // Create a savepoint
    let savepoint1: Savepoint;
    {
        let mut write_txn = db.begin_write()?;
        // Create the first savepoint
        savepoint1 = write_txn.persistent_savepoint()?;
        write_txn.commit()?;
    }

    // Modify the database - add key2 and key3
    {
        let mut write_txn = db.begin_write()?;
        // Modify the database - add key2
        let mut table = write_txn.open_table(TEST_TABLE)?;
        table.insert("key2", 2)?;
        drop(table);

        // Modify the database again - add key3
        let mut table = write_txn.open_table(TEST_TABLE)?;
        table.insert("key3", 3)?;
        drop(table);

        write_txn.commit()?;
    }

    // Verify all changes are present
    {
        let read_txn = db.begin_read()?;
        let table = read_txn.open_table(TEST_TABLE)?;
        assert_eq!(table.get("key1")?.map(|v| v.value()), Some(1));
        assert_eq!(table.get("key2")?.map(|v| v.value()), Some(2));
        assert_eq!(table.get("key3")?.map(|v| v.value()), Some(3));
        drop(read_txn);
    }

    // Restore to the first savepoint
    {
        let mut write_txn = db.begin_write()?;
        let redb_savepoint = write_txn.get_persistent_savepoint(savepoint1)?;
        write_txn.restore_savepoint(&redb_savepoint)?;
        write_txn.commit()?;
    }

    // Verify that changes after savepoint1 were rolled back
    assert_eq!(read_value(db, "key1")?, Some(1)); // Should remain unchanged
    assert_eq!(read_value(db, "key2")?, None); // Should not exist
    assert_eq!(read_value(db, "key3")?, None); // Should not exist

    Ok(())
}
