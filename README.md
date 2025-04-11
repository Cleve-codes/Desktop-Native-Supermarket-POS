# POS System

A native desktop Point of Sale (POS) system built with Tauri, Rust, React, and SQLite.

## Features

- **Native Performance**: Built with Tauri for a lightweight, fast, and secure desktop application
- **Offline-First**: Fully functional without internet connectivity
- **Secure Authentication**: Role-based access control with secure password hashing
- **Real-time Inventory Management**: Track stock levels and get low-stock alerts
- **Transaction Processing**: Fast and reliable sales processing
- **Receipt Generation**: Generate and print receipts
- **Reporting and Analytics**: Sales reports and business insights
- **Data Backup and Recovery**: Secure local data storage with backup capabilities

## Technology Stack

- **Frontend**: React, TypeScript, TailwindCSS, React Query
- **Backend**: Rust, Tauri
- **Database**: SQLite with Prisma
- **State Management**: React Query
- **Form Handling**: React Hook Form
- **Validation**: Zod
- **Printing**: pdf-lib

## Prerequisites

- [Node.js](https://nodejs.org/) (v16+)
- [Rust](https://www.rust-lang.org/tools/install)
- [Bun](https://bun.sh/) (recommended)
- [Tauri Prerequisites](https://tauri.app/v1/guides/getting-started/prerequisites)

## Development Setup

1. Clone the repository
   ```bash
   git clone https://github.com/yourusername/pos-system.git
   cd pos-system
   ```

2. Install dependencies
   ```bash
   bun install
   ```

3. Run the development server
   ```bash
   bun run tauri dev
   ```

## Build for Production

```bash
bun run tauri build
```

This will generate platform-specific binaries in the `src-tauri/target/release` folder.

## Project Structure

```
pos-system/
├── src/                   # React frontend
│   ├── api/               # API client for Tauri commands
│   ├── components/        # Reusable UI components
│   ├── pages/             # Application pages
│   ├── App.tsx            # Main application component
│   └── main.tsx           # Application entry point
├── src-tauri/             # Rust backend
│   ├── src/               # Rust source code
│   │   ├── database.rs    # Database operations
│   │   ├── error.rs       # Error handling
│   │   ├── handlers/      # Command handlers
│   │   ├── models.rs      # Data models
│   │   ├── utils.rs       # Utility functions
│   │   └── main.rs        # Rust entry point
│   ├── migrations/        # SQL migrations
│   └── Cargo.toml         # Rust dependencies
├── prisma/                # Prisma schema and client
│   └── schema.prisma      # Database schema
└── package.json           # Node dependencies
```

## Default Login

- **Username**: admin
- **Password**: admin123

## License

MIT
