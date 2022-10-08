import Axios, { AxiosError } from 'axios';

import { Tokens } from '@/types';
import { storage } from '@/utils/storage';

const baseURL = import.meta.env.VITE_APP_BACKEND_URL;

export const axios = Axios.create({
  baseURL,
  headers: {
    Accept: 'application/json',
    'Content-Type': 'application/json',
  },
});

axios.interceptors.request.use((config) => {
  if (!config.headers) return config;

  const accessToken = storage.get('access_token');
  if (accessToken) {
    config.headers['Authorization'] = `Bearer ${accessToken}`;
  }
  return config;
});

axios.interceptors.response.use(undefined, async (err: AxiosError) => {
  if (err.response?.status !== 401) return Promise.reject(err);

  const refreshToken = storage.get('refresh_token');
  if (!refreshToken) return Promise.reject(err);

  try {
    const { data } = await Axios.create({
      baseURL,
      headers: {
        Accept: 'application/json',
        'Content-Type': 'application/json',
        Authorization: `Bearer ${refreshToken}`,
      },
    }).put<Tokens>('users/sessions');

    storage.set('access_token', data.accessToken);
    storage.set('refresh_token', data.refreshToken);

    return axios(err.config);
  } catch (err) {
    return Promise.reject(err);
  }
});
