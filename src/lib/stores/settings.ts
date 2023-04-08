import { invoke } from '@tauri-apps/api';
import { writable } from 'svelte/store';

export const aiHost = writable<string>("localhost");
export const aiPort = writable<string>("80");

export const serverPort = writable<string>("3000");

// eslint-disable-next-line @typescript-eslint/ban-types
const debounce = (func: Function, timeout = 1000) => {
  let timer: number;
  return (...args: any[]) => {
    clearTimeout(timer);
    timer = setTimeout(() => { func.apply(this, args); }, timeout);
  };
}

const switchPort = debounce((value: string) => {
  invoke("spawn_server", {
    "port": parseInt(value),
  });
});

serverPort.subscribe((value) => {
  switchPort(value);
});