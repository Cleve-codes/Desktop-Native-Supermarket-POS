// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod database;
mod error;
mod models;
mod handlers;
mod utils;

use tauri_plugin_sql::{Migration, MigrationKind};

fn main() {
    let mut migrations = Vec::new();
    
    // Add initial migration to create schema
    migrations.push(Migration {
        version: 1,
        description: "Initial migration",
        sql: include_str!("../migrations/001_initial_schema.sql"),
        kind: MigrationKind::Up,
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_sql::Builder::default().add_migrations(
            "sqlite:pos_database.db",
            migrations,
        ).build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            handlers::users::login, 
            handlers::users::create_user,
            handlers::products::get_products,
            handlers::products::create_product,
            handlers::products::update_product,
            handlers::transactions::create_transaction,
            handlers::transactions::get_transactions,
            handlers::transactions::void_transaction,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
