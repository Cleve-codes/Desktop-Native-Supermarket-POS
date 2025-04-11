import { Link } from "react-router-dom";

const NotFoundPage = () => {
  return (
    <div className="flex flex-col items-center justify-center h-full">
      <h1 className="text-6xl font-bold text-gray-800">404</h1>
      <p className="text-2xl text-gray-600 mb-6">Page Not Found</p>
      <p className="text-gray-500 mb-6">
        The page you are looking for might have been removed or is temporarily unavailable.
      </p>
      <Link
        to="/dashboard"
        className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors"
      >
        Back to Dashboard
      </Link>
    </div>
  );
};

export default NotFoundPage; 