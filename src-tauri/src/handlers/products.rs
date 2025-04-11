use crate::database;
use crate::error::AppResult;
use crate::models::{Product, CreateProductRequest, UpdateProductRequest};

/// Get all products
#[tauri::command]
pub async fn get_products() -> AppResult<Vec<Product>> {
    let products = database::get_products()?;
    Ok(products)
}

/// Create a new product
#[tauri::command]
pub async fn create_product(request: CreateProductRequest) -> AppResult<Product> {
    let product = database::create_product(&request)?;
    Ok(product)
}

/// Update an existing product
#[tauri::command]
pub async fn update_product(request: UpdateProductRequest) -> AppResult<Product> {
    // This would be implemented to update a product
    // For now, we'll just get the product to show the handler framework
    let product = database::get_product_by_id(&request.id)?
        .ok_or_else(|| crate::error::AppError::NotFound("Product not found".to_string()))?;
    
    Ok(product)
} 