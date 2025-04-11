import { createBrowserRouter, Navigate } from "react-router-dom";
import App from "./App";
import LoginPage from "./pages/auth/LoginPage";
import DashboardPage from "./pages/dashboard/DashboardPage";
import ProductsPage from "./pages/products/ProductsPage";
import TransactionsPage from "./pages/transactions/TransactionsPage";
import PosPage from "./pages/pos/PosPage";
import NotFoundPage from "./pages/NotFoundPage";

// Define routes
export const router = createBrowserRouter([
  {
    path: "/",
    element: <App />,
    children: [
      {
        index: true,
        element: <Navigate to="/login" />,
      },
      {
        path: "login",
        element: <LoginPage />,
      },
      {
        path: "dashboard",
        element: <DashboardPage />,
      },
      {
        path: "products",
        element: <ProductsPage />,
      },
      {
        path: "transactions",
        element: <TransactionsPage />,
      },
      {
        path: "pos",
        element: <PosPage />,
      },
      {
        path: "*",
        element: <NotFoundPage />,
      },
    ],
  },
]); 