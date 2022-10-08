import { Box, Button, Center, Stack } from '@chakra-ui/react';
import { format } from 'date-fns';
import { useNavigate } from 'react-router-dom';

import { Layout } from '@/components/Layout';
import { useMe } from '@/hooks/useMe';
import { useAuth } from '@/lib/auth';

export const Home = () => {
  const { signOut } = useAuth();
  const navigate = useNavigate();
  const { me } = useMe();

  const onSignOut = async () => {
    await signOut();
    navigate('/auth/sign-in');
  };

  return (
    <Layout>
      <Center>
        <Stack w="md" mx="4" p="8" borderRadius="md" bg="white">
          <Box alignSelf="center" fontWeight="bold" fontSize="2xl">
            User Info
          </Box>

          <Box fontFamily="mono" whiteSpace="pre-wrap">
            <Box>
              {'name'.padEnd(10, ' ')}: {me.name}
            </Box>
            <Box>
              {'created'.padEnd(10, ' ')}: {fmt(me.createdAt)}
            </Box>
            <Box>
              {'updated'.padEnd(10, ' ')}: {fmt(me.updatedAt)}
            </Box>
          </Box>

          <Button mt="4!" onClick={onSignOut}>
            Sign Out
          </Button>
        </Stack>
      </Center>
    </Layout>
  );
};

const fmt = (s: string) => format(new Date(s), 'yyyy/MM/dd HH:mm:ss');
