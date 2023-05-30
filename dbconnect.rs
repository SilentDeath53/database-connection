use tokio_postgres::{NoTls, Error};

async fn main() -> Result<(), Error> {
    let (client, connection) = tokio_postgres::connect("host=localhost user=postgres dbname=mydatabase", NoTls).await?;

    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("connection error: {}", e);
        }
    });

    create_table(&client).await?;
    insert_data(&client, 1, "John Doe").await?;
    insert_data(&client, 2, "Jane Smith").await?;
    select_data(&client).await?;

    client.close().await?;

    Ok(())
}

async fn create_table(client: &tokio_postgres::Client) -> Result<(), Error> {
    client.batch_execute(
        "
        CREATE TABLE IF NOT EXISTS users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL
        )
        ",
    )
    .await?;

    Ok(())
}

async fn insert_data(client: &tokio_postgres::Client, id: i32, name: &str) -> Result<(), Error> {
    client
        .execute(
            "INSERT INTO users (id, name) VALUES ($1, $2)",
            &[&id, &name],
        )
        .await?;

    Ok(())
}

async fn select_data(client: &tokio_postgres::Client) -> Result<(), Error> {
    let rows = client
        .query("SELECT id, name FROM users", &[])
        .await?;

    for row in rows {
        let id: i32 = row.get(0);
        let name: &str = row.get(1);

        println!("User: id={}, name={}", id, name);
    }

    Ok(())
}
