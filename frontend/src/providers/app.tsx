import { ChakraProvider } from '@chakra-ui/react';
import { BrowserRouter } from 'react-router-dom';

import { AuthProvider } from '@/lib/auth';
import { WithChildren } from '@/types';

export const AppProvider = ({ children }: WithChildren) => (
  <ChakraProvider>
    <AuthProvider>
      <BrowserRouter>{children}</BrowserRouter>
    </AuthProvider>
  </ChakraProvider>
);
