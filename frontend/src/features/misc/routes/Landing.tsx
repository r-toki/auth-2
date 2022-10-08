import { Navigate } from 'react-router-dom';

import { useAuth } from '@/lib/auth';

export const Landing = () => {
  const { user } = useAuth();
  return user ? <Navigate to="/app" /> : <Navigate to="/auth/sign-up" />;
};
