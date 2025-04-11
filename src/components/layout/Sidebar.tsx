import { NavLink } from "react-router-dom";

// Define sidebar navigation items
const navItems = [
  { path: "/dashboard", label: "Dashboard", icon: "📊" },
  { path: "/pos", label: "Point of Sale", icon: "💰" },
  { path: "/products", label: "Products", icon: "📦" },
  { path: "/transactions", label: "Transactions", icon: "🧾" },
];

const Sidebar = () => {
  return (
    <aside className="bg-gray-800 text-white w-64 flex-shrink-0 hidden md:block">
      <div className="p-6">
        <h1 className="text-2xl font-bold">POS System</h1>
      </div>
      <nav className="mt-6">
        <ul>
          {navItems.map((item) => (
            <li key={item.path} className="px-6 py-2">
              <NavLink
                to={item.path}
                className={({ isActive }) =>
                  `flex items-center p-2 rounded-lg transition-colors ${
                    isActive
                      ? "bg-blue-700 text-white"
                      : "text-gray-300 hover:bg-gray-700"
                  }`
                }
              >
                <span className="mr-3">{item.icon}</span>
                <span>{item.label}</span>
              </NavLink>
            </li>
          ))}
        </ul>
      </nav>
    </aside>
  );
};

export default Sidebar; 