const storagePrefix = 'auth_2_';

type Key = 'access_token' | 'refresh_token';

export const storage = {
  get: (key: Key): string | null => {
    return window.localStorage.getItem(storagePrefix + key);
  },
  set: (key: Key, value: string) => {
    window.localStorage.setItem(storagePrefix + key, value);
  },
  clear: (key: Key) => {
    window.localStorage.removeItem(storagePrefix + key);
  },
};
