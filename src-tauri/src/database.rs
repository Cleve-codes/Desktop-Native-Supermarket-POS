use crate::error::{AppError, AppResult};
use rusqlite::{Connection, params};
use std::path::Path;
use crate::models;
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::str::FromStr;

const DB_PATH: &str = "pos_database.db";

pub fn get_connection() -> AppResult<Connection> {
    let path = Path::new(DB_PATH);
    let conn = Connection::open(path).map_err(AppError::Database)?;
    Ok(conn)
}

pub fn init_database() -> AppResult<()> {
    let conn = get_connection()?;
    // Migration is handled by the Tauri SQL plugin
    Ok(())
}

// User functions
pub fn get_user_by_username(username: &str) -> AppResult<Option<models::User>> {
    let conn = get_connection()?;
    let mut stmt = conn.prepare(
        "SELECT id, username, password_hash, role, created_at, last_login 
         FROM users 
         WHERE username = ?1"
    ).map_err(AppError::Database)?;
    
    let user = stmt.query_row(params![username], |row| {
        let role_str: String = row.get(3)?;
        let role = match role_str.as_str() {
            "ADMIN" => models::Role::ADMIN,
            "CASHIER" => models::Role::CASHIER,
            _ => models::Role::CASHIER,
        };
        
        let created_at: String = row.get(4)?;
        let created_at = DateTime::from_str(&created_at).unwrap_or_else(|_| Utc::now());
        
        let last_login: Option<String> = row.get(5)?;
        let last_login = last_login.and_then(|t| DateTime::from_str(&t).ok());
        
        Ok(models::User {
            id: row.get(0)?,
            username: row.get(1)?,
            password_hash: row.get(2)?,
            role,
            created_at,
            last_login,
        })
    });
    
    match user {
        Ok(user) => Ok(Some(user)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(AppError::Database(e)),
    }
}

pub fn create_user(user: &models::CreateUserRequest, password_hash: &str) -> AppResult<models::User> {
    let conn = get_connection()?;
    let id = Uuid::new_v4().to_string();
    let role_str = match user.role {
        models::Role::ADMIN => "ADMIN",
        models::Role::CASHIER => "CASHIER",
    };
    
    conn.execute(
        "INSERT INTO users (id, username, password_hash, role, created_at) 
         VALUES (?1, ?2, ?3, ?4, ?5)",
        params![
            id,
            user.username,
            password_hash,
            role_str,
            Utc::now().to_string(),
        ],
    ).map_err(AppError::Database)?;
    
    let created_user = models::User {
        id,
        username: user.username.clone(),
        password_hash: password_hash.to_string(),
        role: user.role.clone(),
        created_at: Utc::now(),
        last_login: None,
    };
    
    Ok(created_user)
}

pub fn update_last_login(user_id: &str) -> AppResult<()> {
    let conn = get_connection()?;
    conn.execute(
        "UPDATE users SET last_login = ?1 WHERE id = ?2",
        params![Utc::now().to_string(), user_id],
    ).map_err(AppError::Database)?;
    
    Ok(())
}

// Product functions
pub fn get_products() -> AppResult<Vec<models::Product>> {
    let conn = get_connection()?;
    let mut stmt = conn.prepare(
        "SELECT id, barcode, name, description, price, stock_quantity, reorder_level, 
                category, created_at, updated_at 
         FROM products"
    ).map_err(AppError::Database)?;
    
    let products = stmt.query_map([], |row| {
        let created_at: String = row.get(8)?;
        let created_at = DateTime::from_str(&created_at).unwrap_or_else(|_| Utc::now());
        
        let updated_at: String = row.get(9)?;
        let updated_at = DateTime::from_str(&updated_at).unwrap_or_else(|_| Utc::now());
        
        Ok(models::Product {
            id: row.get(0)?,
            barcode: row.get(1)?,
            name: row.get(2)?,
            description: row.get(3)?,
            price: row.get(4)?,
            stock_quantity: row.get(5)?,
            reorder_level: row.get(6)?,
            category: row.get(7)?,
            created_at,
            updated_at,
        })
    }).map_err(AppError::Database)?;
    
    let mut result = Vec::new();
    for product in products {
        result.push(product.map_err(AppError::Database)?);
    }
    
    Ok(result)
}

pub fn get_product_by_id(id: &str) -> AppResult<Option<models::Product>> {
    let conn = get_connection()?;
    let mut stmt = conn.prepare(
        "SELECT id, barcode, name, description, price, stock_quantity, reorder_level, 
                category, created_at, updated_at 
         FROM products 
         WHERE id = ?1"
    ).map_err(AppError::Database)?;
    
    let product = stmt.query_row(params![id], |row| {
        let created_at: String = row.get(8)?;
        let created_at = DateTime::from_str(&created_at).unwrap_or_else(|_| Utc::now());
        
        let updated_at: String = row.get(9)?;
        let updated_at = DateTime::from_str(&updated_at).unwrap_or_else(|_| Utc::now());
        
        Ok(models::Product {
            id: row.get(0)?,
            barcode: row.get(1)?,
            name: row.get(2)?,
            description: row.get(3)?,
            price: row.get(4)?,
            stock_quantity: row.get(5)?,
            reorder_level: row.get(6)?,
            category: row.get(7)?,
            created_at,
            updated_at,
        })
    });
    
    match product {
        Ok(product) => Ok(Some(product)),
        Err(rusqlite::Error::QueryReturnedNoRows) => Ok(None),
        Err(e) => Err(AppError::Database(e)),
    }
}

pub fn create_product(product: &models::CreateProductRequest) -> AppResult<models::Product> {
    let conn = get_connection()?;
    let id = Uuid::new_v4().to_string();
    let now = Utc::now().to_string();
    
    conn.execute(
        "INSERT INTO products (id, barcode, name, description, price, stock_quantity, 
                              reorder_level, category, created_at, updated_at) 
         VALUES (?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8, ?9, ?10)",
        params![
            id,
            product.barcode,
            product.name,
            product.description,
            product.price,
            product.stock_quantity,
            product.reorder_level,
            product.category,
            now,
            now,
        ],
    ).map_err(AppError::Database)?;
    
    let created_product = models::Product {
        id,
        barcode: product.barcode.clone(),
        name: product.name.clone(),
        description: product.description.clone(),
        price: product.price,
        stock_quantity: product.stock_quantity,
        reorder_level: product.reorder_level,
        category: product.category.clone(),
        created_at: Utc::now(),
        updated_at: Utc::now(),
    };
    
    Ok(created_product)
}

// Only partially implemented to save space - would need implementation for other database operations
// In a real implementation, you would add functions for all CRUD operations for each model 