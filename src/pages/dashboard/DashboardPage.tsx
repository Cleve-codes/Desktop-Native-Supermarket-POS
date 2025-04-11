const DashboardPage = () => {
  return (
    <div>
      <h1 className="text-2xl font-bold mb-6">Dashboard</h1>
      
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        {/* Sales Overview Card */}
        <div className="bg-white p-6 rounded-lg shadow-md">
          <h2 className="text-gray-600 text-sm uppercase font-bold mb-2">
            Total Sales Today
          </h2>
          <p className="text-3xl font-bold text-blue-600">$1,234.56</p>
          <div className="flex items-center mt-2">
            <span className="text-green-500 text-sm mr-1">+12%</span>
            <span className="text-gray-500 text-sm">from yesterday</span>
          </div>
        </div>
        
        {/* Transactions Card */}
        <div className="bg-white p-6 rounded-lg shadow-md">
          <h2 className="text-gray-600 text-sm uppercase font-bold mb-2">
            Transactions
          </h2>
          <p className="text-3xl font-bold text-blue-600">45</p>
          <div className="flex items-center mt-2">
            <span className="text-gray-500 text-sm">today</span>
          </div>
        </div>
        
        {/* Low Stock Card */}
        <div className="bg-white p-6 rounded-lg shadow-md">
          <h2 className="text-gray-600 text-sm uppercase font-bold mb-2">
            Low Stock Items
          </h2>
          <p className="text-3xl font-bold text-orange-500">8</p>
          <div className="flex items-center mt-2">
            <span className="text-gray-500 text-sm">need attention</span>
          </div>
        </div>
        
        {/* Active Users Card */}
        <div className="bg-white p-6 rounded-lg shadow-md">
          <h2 className="text-gray-600 text-sm uppercase font-bold mb-2">
            Active Users
          </h2>
          <p className="text-3xl font-bold text-blue-600">3</p>
          <div className="flex items-center mt-2">
            <span className="text-gray-500 text-sm">currently online</span>
          </div>
        </div>
      </div>
      
      {/* Recent Activity Section */}
      <div className="mt-8">
        <h2 className="text-xl font-bold mb-4">Recent Activity</h2>
        <div className="bg-white rounded-lg shadow-md overflow-hidden">
          <table className="min-w-full divide-y divide-gray-200">
            <thead className="bg-gray-50">
              <tr>
                <th scope="col" className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Time
                </th>
                <th scope="col" className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  User
                </th>
                <th scope="col" className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Action
                </th>
                <th scope="col" className="px-6 py-3 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                  Details
                </th>
              </tr>
            </thead>
            <tbody className="bg-white divide-y divide-gray-200">
              {[1, 2, 3, 4, 5].map((item) => (
                <tr key={item}>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                    10:23 AM
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                    John Doe
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                    Completed Sale
                  </td>
                  <td className="px-6 py-4 whitespace-nowrap text-sm text-gray-500">
                    $156.78
                  </td>
                </tr>
              ))}
            </tbody>
          </table>
        </div>
      </div>
    </div>
  );
};

export default DashboardPage; 