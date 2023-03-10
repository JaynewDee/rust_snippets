//////////////////////////////////////
/// Dependency injection with traits!
//////////////////////////////////////

// Trait enforces what functions must be defined
//      for the struct which implements it
trait Database {
    fn connect(&self) -> bool;
    fn disconnect(&self) -> bool;
    fn get_data(&self, query: &str) -> Vec<String>;
}

// Mock database
struct DatabaseConnection {
    conn_str: String,
}

impl Database for DatabaseConnection {
    fn connect(&self) -> bool {
        println!("Connecting to database...");
        true
    }

    fn disconnect(&self) -> bool {
        println!("Disconnecting from database...");
        true
    }

    fn get_data(&self, query: &str) -> Vec<String> {
        let result = vec![
            String::from("Alice, 28, Female"),
            String::from("Bob, 32, Male"),
            String::from("Charlie, 21, Male"),
        ];
        println!("Executing query: {}", query);
        result
    }
}

struct SqliteDatabase {
    connection: DatabaseConnection,
}

impl SqliteDatabase {
    fn new(conn_str: String) -> Self {
        SqliteDatabase {
            connection: DatabaseConnection { conn_str },
        }
    }
}

impl Database for SqliteDatabase {
    fn connect(&self) -> bool {
        self.connection.connect()
    }

    fn disconnect(&self) -> bool {
        self.connection.disconnect()
    }

    fn get_data(&self, query: &str) -> Vec<String> {
        self.connection.get_data(query)
    }
}

struct PostgresDatabase {
    connection: DatabaseConnection,
}

impl PostgresDatabase {
    fn new(conn_str: String) -> Self {
        PostgresDatabase {
            connection: DatabaseConnection { conn_str },
        }
    }
}

impl Database for PostgresDatabase {
    fn connect(&self) -> bool {
        self.connection.connect()
    }

    fn disconnect(&self) -> bool {
        self.connection.disconnect()
    }

    fn get_data(&self, query: &str) -> Vec<String> {
        self.connection.get_data(query)
    }
}

struct DataProcessor<D: Database> {
    db: D,
}

impl<D: Database> DataProcessor<D> {
    fn new(db: D) -> Self {
        Self { db }
    }

    fn process_data(&self, query: &str) {
        self.db.connect();
        let data = self.db.get_data(query);
        self.db.disconnect();

        println!("Processing data...");
        for row in data {
            println!("{}", row);
        }
    }
}

fn main() {
    // let data_processor_sqlite = DataProcessor::new(sqlite);
    // data_processor_sqlite.process_data("SELECT * FROM users");

    // let data_processor_postgres = DataProcessor::new(postgres);
    // data_processor_postgres.process_data("SELECT * FROM users");
}
