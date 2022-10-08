import { Link as ChakraLink, LinkProps as ChakraLinkProps } from '@chakra-ui/react';
import { FC } from 'react';
import { Link as RouterLink, LinkProps as RouterLinkProps } from 'react-router-dom';

export const Link: FC<RouterLinkProps & ChakraLinkProps> = (props) => {
  return <ChakraLink as={RouterLink} {...props} />;
};
