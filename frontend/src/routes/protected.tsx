import { Navigate, Outlet } from 'react-router-dom';

import { Home } from '@/features/misc';

const App = () => <Outlet />;

export const protectedRoutes = [
  {
    path: '/app',
    element: <App />,
    children: [
      { path: '/', element: <Home /> },
      { path: '*', element: <Navigate to="." /> },
    ],
  },
];
