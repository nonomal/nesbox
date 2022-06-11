/// <reference types="vite/client" />

import type * as window from '@tauri-apps/api/window';
import type * as shell from '@tauri-apps/api/shell';

declare global {
  interface Window {
    __TAURI__?: {
      window: typeof window;
      shell: typeof shell;
    };
  }
}
