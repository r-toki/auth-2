import { Box, Button, Center, Divider, Stack } from '@chakra-ui/react';
import { Link, useNavigate } from 'react-router-dom';
import { z } from 'zod';

import { Form, InputField } from '@/components/Form';

const schema = z
  .object({
    name: z
      .string()
      .min(3, 'Name must be over 3 characters')
      .max(15, 'Name must be no more than 15 characters'),
    password: z.string().min(8, 'Password must be over 8 characters'),
    confirm: z.string().min(1, 'Required'),
  })
  .refine((fields) => fields.password === fields.confirm, {
    message: "Passwords don't match",
    path: ['confirm'],
  });

type RegisterValue = {
  name: string;
  password: string;
  confirm: string;
};

export const SignUp = () => {
  const navigate = useNavigate();

  const onSubmit = async ({ name, password }: RegisterValue) => {
    console.log(name, password);
  };

  return (
    <Box>
      <Center>
        <Stack w="md" mx="4" p="8" borderRadius="md" bg="white">
          <Box alignSelf="center" fontWeight="bold" fontSize="2xl">
            Vite + React + TS
          </Box>

          <Form<RegisterValue, typeof schema> onSubmit={onSubmit} schema={schema}>
            {({ register, formState: { errors } }) => (
              <Stack spacing="6">
                <Stack spacing="4">
                  <InputField label="Name" registration={register('name')} error={errors.name} />

                  <InputField
                    type="password"
                    label="Password"
                    registration={register('password')}
                    error={errors.password}
                  />

                  <InputField
                    type="password"
                    label="Confirm"
                    registration={register('confirm')}
                    error={errors.confirm}
                  />
                </Stack>

                <Divider />

                <Button type="submit" colorScheme="blue">
                  Sign Up
                </Button>
              </Stack>
            )}
          </Form>

          <Box>
            <Link to="/auth/sign-in">to Sign In</Link>
          </Box>
        </Stack>
      </Center>
    </Box>
  );
};
