import { useEffect, useState } from "react";
import { Outlet, useNavigate, useLocation } from "react-router-dom";
import Sidebar from "./components/layout/Sidebar";
import Header from "./components/layout/Header";

function App() {
  const [isLoggedIn, setIsLoggedIn] = useState(false);
  const navigate = useNavigate();
  const location = useLocation();
  
  // Check if user is logged in
  useEffect(() => {
    const token = localStorage.getItem("token");
    const isLoginPage = location.pathname === "/login";
    
    if (!token && !isLoginPage) {
      navigate("/login");
    } else if (token && isLoginPage) {
      navigate("/dashboard");
    }
    
    setIsLoggedIn(!!token);
  }, [location.pathname, navigate]);

  // If not logged in and not on login page, render only the Outlet (which will be the login page)
  if (!isLoggedIn && location.pathname === "/login") {
    return <Outlet />;
  }

  return (
    <div className="flex h-screen bg-gray-100">
      {isLoggedIn && <Sidebar />}
      <div className="flex flex-col flex-1 overflow-hidden">
        {isLoggedIn && <Header />}
        <main className="flex-1 overflow-x-hidden overflow-y-auto p-4">
          <Outlet />
        </main>
      </div>
    </div>
  );
}

export default App;
