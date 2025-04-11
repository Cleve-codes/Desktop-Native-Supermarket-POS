use serde::{Serialize, Deserialize};
use uuid::Uuid;
use chrono::{DateTime, Utc};

// User Models
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum Role {
    ADMIN,
    CASHIER,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct User {
    pub id: String,
    pub username: String,
    #[serde(skip_serializing)]
    pub password_hash: String,
    pub role: Role,
    pub created_at: DateTime<Utc>,
    pub last_login: Option<DateTime<Utc>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateUserRequest {
    pub username: String,
    pub password: String,
    pub role: Role,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginRequest {
    pub username: String,
    pub password: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LoginResponse {
    pub user: User,
    pub token: String,
}

// Product Models
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Product {
    pub id: String,
    pub barcode: String,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub stock_quantity: i32,
    pub reorder_level: i32,
    pub category: Option<String>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateProductRequest {
    pub barcode: String,
    pub name: String,
    pub description: Option<String>,
    pub price: f64,
    pub stock_quantity: i32,
    pub reorder_level: i32,
    pub category: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct UpdateProductRequest {
    pub id: String,
    pub barcode: Option<String>,
    pub name: Option<String>,
    pub description: Option<String>,
    pub price: Option<f64>,
    pub stock_quantity: Option<i32>,
    pub reorder_level: Option<i32>,
    pub category: Option<String>,
}

// Transaction Models
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum TransactionStatus {
    COMPLETED,
    VOIDED,
    PENDING,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Transaction {
    pub id: String,
    pub cashier_id: String,
    pub total_amount: f64,
    pub payment_method: String,
    pub status: TransactionStatus,
    pub created_at: DateTime<Utc>,
    pub voided_at: Option<DateTime<Utc>>,
    pub void_reason: Option<String>,
    pub items: Vec<TransactionItem>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TransactionItem {
    pub id: String,
    pub transaction_id: String,
    pub product_id: String,
    pub product: Option<Product>,
    pub quantity: i32,
    pub unit_price: f64,
    pub subtotal: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTransactionRequest {
    pub cashier_id: String,
    pub payment_method: String,
    pub items: Vec<CreateTransactionItemRequest>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTransactionItemRequest {
    pub product_id: String,
    pub quantity: i32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VoidTransactionRequest {
    pub transaction_id: String,
    pub user_id: String,
    pub void_reason: String,
}

// Audit Log Models
#[derive(Debug, Serialize, Deserialize)]
pub struct AuditLog {
    pub id: String,
    pub user_id: String,
    pub action: String,
    pub entity_type: String,
    pub entity_id: String,
    pub changes: String,
    pub created_at: DateTime<Utc>,
}

// Sync Queue Models
#[derive(Debug, Serialize, Deserialize, Clone)]
pub enum SyncStatus {
    PENDING,
    PROCESSING,
    COMPLETED,
    FAILED,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SyncQueue {
    pub id: String,
    pub operation: String,
    pub entity_type: String,
    pub entity_id: String,
    pub payload: String,
    pub created_at: DateTime<Utc>,
    pub attempts: i32,
    pub last_attempt: Option<DateTime<Utc>>,
    pub status: SyncStatus,
}

// Helper function to generate a new UUID string
pub fn generate_id() -> String {
    Uuid::new_v4().to_string()
} 