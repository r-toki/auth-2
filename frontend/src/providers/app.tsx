import { ChakraProvider } from '@chakra-ui/react';
import { BrowserRouter } from 'react-router-dom';

import { WithChildren } from '@/components/types';
import { AuthProvider } from '@/lib/auth';

export const AppProvider = ({ children }: WithChildren) => (
  <ChakraProvider>
    <AuthProvider>
      <BrowserRouter>{children}</BrowserRouter>
    </AuthProvider>
  </ChakraProvider>
);
