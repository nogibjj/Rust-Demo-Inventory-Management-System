use rusqlite::{params, Connection, Result};
use crate::models::{product::Product, category::Category};

pub fn initialize_database(conn: &Connection) -> Result<()> {
    conn.execute(
        "CREATE TABLE IF NOT EXISTS category (
            category_id INTEGER PRIMARY KEY AUTOINCREMENT,
            category_name TEXT NOT NULL UNIQUE
        )",
        params![],
    )?;

    conn.execute(
        "CREATE TABLE IF NOT EXISTS products (
            id INTEGER PRIMARY KEY,
            name TEXT NOT NULL,
            price REAL NOT NULL,
            stock INTEGER NOT NULL,
            category_name TEXT NOT NULL,
            FOREIGN KEY(category_name) REFERENCES category(category_name)
        )",
        params![],
    )?;

    Ok(())
}
//add a product to the database
pub fn add_product(conn: &Connection, product: &Product) -> Result<usize> {
    conn.execute(
        "INSERT INTO products (name, price, stock, category_name) VALUES (?1, ?2, ?3, ?4)",
        params![product.name, product.price, product.stock, product.category_name],
    )
}
//add a category to the database
pub fn add_category(conn: &Connection, category: &Category) -> Result<usize> {
    conn.execute(
        "INSERT INTO category (category_name) VALUES (?1)",
        params![category.category_name],
    )
}
//update a product in the database
pub fn update_product(conn: &Connection, product: &Product) -> Result<usize> {
    conn.execute(
        "UPDATE products SET name = ?1, price = ?2, stock = ?3, category_id = ?4 WHERE id = ?5",
        params![product.name, product.price, product.stock, product.category_name, product.id],
    )
}
//delete a product from the database
pub fn delete_product(conn: &Connection, product_id: i32) -> Result<usize> {
    conn.execute(
        "DELETE FROM products WHERE id = ?1",
        params![product_id],
    )
}
//list all products in the database
pub fn list_categories(conn: &Connection) -> Result<Vec<Category>> {
    let mut stmt = conn.prepare("SELECT category_id, category_name FROM category")?;
    let category_iter = stmt.query_map(params![], |row| {
        Ok(Category {
            category_id: row.get(0)?,
            category_name: row.get(1)?,
        })
    })?;

    category_iter.collect()
}

pub fn list_products(conn: &Connection) -> Result<Vec<Product>> {
    let mut stmt = conn.prepare("SELECT id, name, price, stock, category_name FROM products")?;
    let product_iter = stmt.query_map(params![], |row| {
        Ok(Product {
            id: row.get(0)?,
            name: row.get(1)?,
            price: row.get(2)?,
            stock: row.get(3)?,
            category_name: row.get(4)?,
        })
    })?;

    product_iter.collect()
}
