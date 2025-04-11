use crate::database;
use crate::error::{AppError, AppResult};
use crate::models::{LoginRequest, LoginResponse, CreateUserRequest, User};
use crate::utils;

/// Login handler
#[tauri::command]
pub async fn login(request: LoginRequest) -> AppResult<LoginResponse> {
    // Get user by username
    let user = database::get_user_by_username(&request.username)?
        .ok_or_else(|| AppError::Authentication("Invalid username or password".to_string()))?;
    
    // Verify password
    let is_valid = utils::verify_password(&request.password, &user.password_hash)?;
    if !is_valid {
        return Err(AppError::Authentication("Invalid username or password".to_string()));
    }
    
    // Update last login
    database::update_last_login(&user.id)?;
    
    // Generate token
    let role = match user.role {
        crate::models::Role::ADMIN => "ADMIN",
        crate::models::Role::CASHIER => "CASHIER",
    };
    let token = utils::generate_token(&user.id, role);
    
    // Return user and token
    let response = LoginResponse {
        user,
        token,
    };
    
    Ok(response)
}

/// Create user handler
#[tauri::command]
pub async fn create_user(request: CreateUserRequest) -> AppResult<User> {
    // Check if user exists
    let existing_user = database::get_user_by_username(&request.username)?;
    if existing_user.is_some() {
        return Err(AppError::Validation("Username already exists".to_string()));
    }
    
    // Hash password
    let password_hash = utils::hash_password(&request.password)?;
    
    // Create user
    let user = database::create_user(&request, &password_hash)?;
    
    Ok(user)
} 