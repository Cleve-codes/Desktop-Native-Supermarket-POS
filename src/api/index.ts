import { invoke } from "@tauri-apps/api/core";

// Auth API
export interface LoginRequest {
  username: string;
  password: string;
}

export interface User {
  id: string;
  username: string;
  role: "ADMIN" | "CASHIER";
  createdAt: string;
  lastLogin?: string;
}

export interface LoginResponse {
  user: User;
  token: string;
}

export const authApi = {
  login: async (request: LoginRequest): Promise<LoginResponse> => {
    return invoke<LoginResponse>("login", { request });
  },
  
  createUser: async (request: {
    username: string;
    password: string;
    role: "ADMIN" | "CASHIER";
  }): Promise<User> => {
    return invoke<User>("create_user", { request });
  },
};

// Products API
export interface Product {
  id: string;
  barcode: string;
  name: string;
  description?: string;
  price: number;
  stockQuantity: number;
  reorderLevel: number;
  category?: string;
  createdAt: string;
  updatedAt: string;
}

export interface CreateProductRequest {
  barcode: string;
  name: string;
  description?: string;
  price: number;
  stockQuantity: number;
  reorderLevel: number;
  category?: string;
}

export interface UpdateProductRequest {
  id: string;
  barcode?: string;
  name?: string;
  description?: string;
  price?: number;
  stockQuantity?: number;
  reorderLevel?: number;
  category?: string;
}

export const productApi = {
  getProducts: async (): Promise<Product[]> => {
    return invoke<Product[]>("get_products");
  },
  
  createProduct: async (request: CreateProductRequest): Promise<Product> => {
    return invoke<Product>("create_product", { request });
  },
  
  updateProduct: async (request: UpdateProductRequest): Promise<Product> => {
    return invoke<Product>("update_product", { request });
  },
};

// Transactions API
export interface TransactionItem {
  id: string;
  transactionId: string;
  productId: string;
  product?: Product;
  quantity: number;
  unitPrice: number;
  subtotal: number;
}

export interface Transaction {
  id: string;
  cashierId: string;
  totalAmount: number;
  paymentMethod: string;
  status: "COMPLETED" | "VOIDED" | "PENDING";
  createdAt: string;
  voidedAt?: string;
  voidReason?: string;
  items: TransactionItem[];
}

export interface CreateTransactionRequest {
  cashierId: string;
  paymentMethod: string;
  items: Array<{
    productId: string;
    quantity: number;
  }>;
}

export interface VoidTransactionRequest {
  transactionId: string;
  userId: string;
  voidReason: string;
}

export const transactionApi = {
  getTransactions: async (): Promise<Transaction[]> => {
    return invoke<Transaction[]>("get_transactions");
  },
  
  createTransaction: async (request: CreateTransactionRequest): Promise<Transaction> => {
    return invoke<Transaction>("create_transaction", { request });
  },
  
  voidTransaction: async (request: VoidTransactionRequest): Promise<Transaction> => {
    return invoke<Transaction>("void_transaction", { request });
  },
}; 