import { ReactNode } from 'react';

export type WithChildren<T = Record<string, unknown>> = {
  children: ReactNode;
} & T;
