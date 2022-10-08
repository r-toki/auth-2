import { Route, Routes } from 'react-router-dom';

import { SignIn, SignUp } from '@/features/auth';

const AuthRoutes = () => (
  <Routes>
    <Route path="sign-up" element={<SignUp />} />
    <Route path="sign-in" element={<SignIn />} />
  </Routes>
);

export const publicRoutes = [
  {
    path: '/auth/*',
    element: <AuthRoutes />,
  },
];
