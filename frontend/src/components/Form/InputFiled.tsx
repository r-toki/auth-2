import { Input, InputProps } from '@chakra-ui/react';
import { UseFormRegisterReturn } from 'react-hook-form';

import { FieldWrapper, FieldWrapperProps } from './FieldWrapper';

export type InputFieldProps = FieldWrapperProps & {
  type?: 'text' | 'email' | 'password';
  registration: Partial<UseFormRegisterReturn>;
} & InputProps;

export const InputField = ({
  label,
  error,
  type = 'text',
  registration,
  ...rest
}: InputFieldProps) => {
  return (
    <FieldWrapper label={label} error={error}>
      <Input {...rest} type={type} {...registration} />
    </FieldWrapper>
  );
};
