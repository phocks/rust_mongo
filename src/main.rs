use mongodb::{bson::doc, error::Result, Client};

#[tokio::main]
async fn main() -> Result<()> {
  let client = Client::with_uri_str("mongodb://localhost:27017").await?;
  let collection = client.database("storylab").collection("seats");

  let new_seats = vec![
    doc! { "type": "dog", "name": "Rondo" },
    doc! { "type": "cat", "name": "Smokey" },
    doc! { "type": "cat", "name": "Phil" },
  ];

  collection.insert_many(new_seats, None).await?;

  for db_name in client.list_database_names(None, None).await? {
    println!("{}", db_name);
  }

  Ok(())
}
