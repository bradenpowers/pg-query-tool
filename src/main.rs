use clap::Parser;
use sqlx::postgres::PgPoolOptions;
use sqlx::FromRow;

#[derive(Parser)]
struct Args {
    #[arg(short, long, default_value = "")]
    filter: String,
}

#[derive(FromRow, Debug)]
struct Item {
    id: i32,
    name: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect("postgres://bradenpowers@localhost/postgres")
        .await?;

    let pattern = format!("%{}%", args.filter);

    let items: Vec<Item> = sqlx::query_as(
        "SELECT id, name FROM items WHERE name ILIKE $1 ORDER BY id"
    )
    .bind(pattern)
    .fetch_all(&pool)
    .await?;

    for item in &items {
        println!("{:>4} | {}", item.id, item.name);
    }

    println!("\n{} rows", items.len());
    Ok(())
}