use diesel::SqliteConnection;
use diesel::prelude::*;
use diesel::insert_into;
use super::models::{NewProduct, Product};
use super::schema::products::dsl::products;

embed_migrations!();

pub fn get_products(conn: &SqliteConnection) -> Vec<Product> {
    // Run the migrations to be sure that the `products` table is present
    let _result = embedded_migrations::run(conn);

    products
        .limit(5)
        .load::<Product>(conn)
        .expect("Error loading products")
}

pub fn create_product<'a>(
    conn: &SqliteConnection,
    title: &'a str,
    price: f32,
    link: String,
) -> QueryResult<usize> {
    use schema::products;
    // Run the migrations to be sure that the `products` table is present
    let _result = embedded_migrations::run(conn);

    let new_product = NewProduct {
        title: title,
        price: price,
        link: link,
    };

    // Insert the `NewProduct` in the DB
    insert_into(products::table)
        .values(&new_product)
        .execute(conn)
}
