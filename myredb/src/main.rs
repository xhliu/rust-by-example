use redb::{Database, Error, ReadableTable, TableDefinition, TableHandle, TableStats};
use dotenv::dotenv;
use std::env;

// Define a table with string keys and string values
const TABLE: TableDefinition<&str, &str> = TableDefinition::new("my_table"); // 

const TABLE1: TableDefinition<u64, u64> = TableDefinition::new("STATISTIC_TO_COUNT"); // my_table


fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    let database_path = env::var("REDB_PATH").expect("DATABASE_PATH must be set");

    // read_db_table(&database_path)?;
    // write_db_table(&database_path);

    list_tables(&database_path)?;
    list_table_entries(&database_path)?;

    Ok(())

}

fn read_db_table(db_path: &str) -> Result<(), Error> {
    // Open or create the database
    let db = Database::create(db_path)?;

    // Begin a read transaction
    let read_txn = db.begin_read()?;
    {
        // Open the table
        let table = read_txn.open_table(TABLE)?;

        // Retrieve values
        if let Some(value) = table.get("key1")? {
            println!("key1: {}", value.value());
        }

        if let Some(value) = table.get("key2")? {
            println!("key2: {}", value.value());
        };
    }

    Ok(())
}

fn write_db_table(db_path: &str) -> Result<(), Error> {
    // Open or create the database
    let db = Database::create(db_path)?;

    // Begin a write transaction
    let write_txn = db.begin_write()?;
    {
        // Open the table
        let mut table = write_txn.open_table(TABLE)?;

        // Insert key-value pairs
        table.insert("key1", "value1")?;
        table.insert("key2", "value2")?;
        table.insert("key3", "value3")?;
    }
    // Commit the write transaction
    write_txn.commit()?;

    Ok(())
}

fn list_table_entries(db_path: &str) -> Result<(), Error> {
    // Open the database
    let db = Database::open(db_path)?;

    // Begin a read transaction
    let read_txn = db.begin_read()?;
    {
        // Open the table
        let table = read_txn.open_table(TABLE1)?;

        // Iterate over the table and print keys and values
        for entry in table.iter()? {
            let (key, value) = entry?;
            println!("{}: {}", key.value(), value.value());
        }
    }

    Ok(())
}

fn list_tables(db_path: &str) -> Result<(), Error> {
    let db = Database::open(db_path)?;
    let read_txn = db.begin_read()?;
    let tables = read_txn.list_tables()?;
    
    for table in tables {
        println!("Table name: {}", table.name());
    }
    Ok(())
}