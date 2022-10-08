import { Center, Spinner } from '@chakra-ui/react';
import { createContext, useContext, useEffect, useState } from 'react';

import { WithChildren } from '@/types';
import { assertDefined } from '@/utils/assert-defined';

type User = { name: string };

type State = {
  initialized: boolean;
  user: User | undefined;
  signUp: (name: string, password: string) => Promise<void>;
  signIn: (name: string, password: string) => Promise<void>;
  signOut: () => Promise<void>;
};

const useAuthProvider = (): State => {
  const [initialized, setInitialized] = useState(false);

  const [user, setUser] = useState<User>();

  const signUp = async (name: string, password: string) => {};

  const signIn = async (name: string, password: string) => {};

  const signOut = async () => {};

  useEffect(() => {
    (async () => {
      setInitialized(true);
    })();
  }, []);

  return {
    initialized,
    user,
    signUp,
    signIn,
    signOut,
  };
};

const AuthContext = createContext<State | undefined>(undefined);

export const AuthProvider = ({ children }: WithChildren) => {
  const state = useAuthProvider();
  if (!state.initialized)
    return (
      <Center h="full">
        <Spinner />
      </Center>
    );
  return <AuthContext.Provider value={state}>{children}</AuthContext.Provider>;
};

export const useAuth = () => {
  const state = useContext(AuthContext);
  assertDefined(state);
  return state;
};
