import { FormControl, FormErrorMessage, FormLabel } from '@chakra-ui/react';
import { FieldError } from 'react-hook-form';

import { WithChildren } from '@/types';

export type FieldWrapperProps = {
  label?: string;
  error?: FieldError;
};

export const FieldWrapper = ({ label, error, children }: WithChildren<FieldWrapperProps>) => {
  return (
    <FormControl isInvalid={!!error?.message}>
      {label && <FormLabel>{label}</FormLabel>}
      {children}
      {error?.message && <FormErrorMessage>{error.message}</FormErrorMessage>}
    </FormControl>
  );
};
