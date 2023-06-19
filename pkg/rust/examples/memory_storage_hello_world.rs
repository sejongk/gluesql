#[cfg(feature = "memory-storage")]
mod api_usage {
    use gluesql::{memory_storage::MemoryStorage, prelude::{Glue, Payload, Value}};

    pub async fn run() {
        let storage = MemoryStorage::default();
        let mut glue = Glue::new(storage);

        let queries = "
            CREATE TABLE greet (name TEXT);
            INSERT INTO greet VALUES ('World');
        ";

        glue.execute(queries).await.expect("Execution failed");

        /*
            Select inserted row
        */
        let queries = "
            SELECT name FROM greet
        ";

        let result = glue.execute(queries).await.expect("Failed to execute");

        /*
            Query results are wrapped into a payload enum, on the basis of the query type
        */
        assert_eq!(result.len(), 1);
        let rows = match &result[0] {
            Payload::Select { labels: _, rows } => rows,
            _ => panic!("Unexpected result: {:?}", result),
        };

        let first_row = &rows[0];
        let first_value = first_row.iter().next().unwrap();

        /*
            Row values are wrapped into a value enum, on the basis of the result type
        */
        let to_greet = match first_value {
            Value::Str(to_greet) => to_greet,
            value => panic!("Unexpected type: {:?}", value),
        };

        println!("Hello {}!", to_greet); // Will always output "Hello World!"
    }
}

fn main() {
    #[cfg(feature = "memory-storage")]
    futures::executor::block_on(api_usage::run());
}
