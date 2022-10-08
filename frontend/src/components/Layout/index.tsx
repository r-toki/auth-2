import { Box } from '@chakra-ui/react';

import { WithChildren } from '../types';

export const Layout = ({ children }: WithChildren) => {
  return (
    <Box h="full" overflow="auto" py="10" bg="gray.100">
      {children}
    </Box>
  );
};
