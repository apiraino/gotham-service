use super::schema::products;
// use diesel::prelude::*;

#[derive(Queryable, Serialize, Debug)]
pub struct Product {
    pub id: Option<i32>,
    pub title: String,
    pub price: f32,
    pub link: String,
}

#[derive(Insertable, Deserialize)]
#[table_name = "products"]
pub struct NewProduct<'a> {
    pub title: &'a str,
    pub price: f32,
    pub link: String,
}
