import { Box } from '@chakra-ui/react';
import { zodResolver } from '@hookform/resolvers/zod';
import { SubmitHandler, useForm, UseFormProps, UseFormReturn } from 'react-hook-form';
import { z, ZodTypeAny } from 'zod';

type FormProps<Schema extends ZodTypeAny> = {
  onSubmit: SubmitHandler<z.infer<Schema>>;
  children: (methods: UseFormReturn<z.infer<Schema>>) => React.ReactNode;
  options?: UseFormProps<z.infer<Schema>>;
  schema?: Schema;
};

export const Form = <Schema extends ZodTypeAny>({
  onSubmit,
  children,
  options,
  schema,
}: FormProps<Schema>) => {
  const methods = useForm<z.infer<Schema>>({ ...options, resolver: schema && zodResolver(schema) });
  return (
    <Box as="form" onSubmit={methods.handleSubmit(onSubmit)}>
      {children(methods)}
    </Box>
  );
};
