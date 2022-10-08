import { AppProvider } from '@/providers/app';
import { AppRoutes } from '@/routes';

export const App = () => (
  <AppProvider>
    <AppRoutes />
  </AppProvider>
);
