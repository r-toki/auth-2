import Axios, { AxiosError } from 'axios';

import { Tokens } from '@/types';
import { storage } from '@/utils/storage';

const baseURL = import.meta.env.VITE_APP_BACKEND_URL;

export const axios = Axios.create({ baseURL });

// NOTE: retry 時の config.headers が壊れるので再定義する
axios.interceptors.request.use((config) => {
  const accessToken = storage.get('access_token');
  config.headers = {
    Accept: 'application/json',
    'Content-Type': 'application/json',
    Authorization: accessToken ? `Bearer ${accessToken}` : undefined,
  };
  return config;
});

axios.interceptors.response.use(undefined, async (err: AxiosError) => {
  const refreshToken = storage.get('refresh_token');
  if (!err.config || err.response?.status !== 401 || !refreshToken) return Promise.reject(err);

  try {
    const { data } = await Axios.create({
      baseURL,
      headers: {
        Accept: 'application/json',
        'Content-Type': 'application/json',
        Authorization: `Bearer ${refreshToken}`,
      },
    }).patch<Tokens>('users/sessions');

    storage.set('access_token', data.accessToken);
    storage.set('refresh_token', data.refreshToken);

    return axios(err.config);
  } catch (err) {
    return Promise.reject(err);
  }
});
