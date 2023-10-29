use rusqlite::Connection;
use crate::models::{product::Product, category::Category};
use crate::db::crud;
use prettytable::{Table, Row, Cell};
use prettytable::row;
use prettytable::cell;

pub fn run_cli(conn: &Connection) {
    loop {
        println!("Enter a command (add_product, list_products, update_product, delete_product, add_category, list_categories, exit):");
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        let command: &str = input.trim();

        match command {
            "add_product" => handle_add_product(conn),
            "list_products" => handle_list_products(conn),
            "update_product" => handle_update_product(conn),
            "delete_product" => handle_delete_product(conn),
            "add_category" => handle_add_category(conn),
            "list_categories" => handle_list_categories(conn),
            "exit" | "quit" => {
                println!("Exiting...");
                break;
            },
            _ => println!("Unknown command"),
        }
    }
}

fn prompt_for_input(prompt: &str) -> String {
    println!("{}", prompt);
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn handle_add_product(conn: &Connection) {
    let name = prompt_for_input("Enter product name:");
    let price: f64 = prompt_for_input("Enter product price:").parse().expect("Invalid price");
    let stock: i32 = prompt_for_input("Enter product stock:").parse().expect("Invalid stock count");
    let category_name: String = prompt_for_input("Enter category name:");

    let product = Product {
        id: 0,
        name,
        price,
        stock,
        category_name,
    };

    match crud::add_product(conn, &product) {
        Ok(_) => println!("Product added successfully!"),
        Err(e) => println!("Error adding product: {}", e),
    }
}


fn handle_update_product(conn: &Connection) {
    let id: i32 = prompt_for_input("Enter product ID to update:").parse().expect("Invalid product ID");
    let name = prompt_for_input("Enter new product name:");
    let price: f64 = prompt_for_input("Enter new product price:").parse().expect("Invalid price");
    let stock: i32 = prompt_for_input("Enter new product stock:").parse().expect("Invalid stock count");
    let category_name: String = prompt_for_input("Enter new category name:");

    let product = Product {
        id,
        name,
        price,
        stock,
        category_name,
    };

    match crud::update_product(conn, &product) {
        Ok(_) => println!("Product updated successfully!"),
        Err(e) => println!("Error updating product: {}", e),
    }
}

fn handle_delete_product(conn: &Connection) {
    let id: i32 = prompt_for_input("Enter product ID to delete:").parse().expect("Invalid product ID");

    match crud::delete_product(conn, id) {
        Ok(_) => println!("Product deleted successfully!"),
        Err(e) => println!("Error deleting product: {}", e),
    }
}

fn handle_add_category(conn: &Connection) {
    let category_name = prompt_for_input("Enter category name:");

    let category = Category {
        category_id: 0,
        category_name,
    };

    match crud::add_category(conn, &category) {
        Ok(_) => println!("Category added successfully!"),
        Err(e) => println!("Error adding category: {}", e),
    }
}


fn handle_list_products(conn: &Connection) {
    match crud::list_products(conn) {
        Ok(products) => {
            let mut table = Table::new();
            // Add a row per time
            table.add_row(row!["ID", "Product Name", "Price ($)", "Stock", "Category Name"]);
            for product in products {
                table.add_row(Row::new(vec![
                    Cell::new(&product.id.to_string()),
                    Cell::new(&product.name),
                    Cell::new(&format!("{:.2}", product.price)),
                    Cell::new(&product.stock.to_string()),
                    Cell::new(&product.category_name),
                ]));
            }
            // Print the table to stdout
            table.printstd();
        },
        Err(e) => println!("Error listing products: {}", e),
    }
}

fn handle_list_categories(conn: &Connection) {
    match crud::list_categories(conn) {
        Ok(categories) => {
            let mut table = Table::new();
            table.add_row(row!["Category ID", "Name"]);
            for category in categories {
                table.add_row(Row::new(vec![
                    Cell::new(&category.category_id.to_string()),
                    Cell::new(&category.category_name),
                ]));
            }
            table.printstd();
        },
        Err(e) => println!("Error listing categories: {}", e),
    }
}



