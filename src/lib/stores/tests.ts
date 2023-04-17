import { writable } from 'svelte/store';

export interface Test {
  path: string;
  name: string;
  videoElem?: HTMLVideoElement;
  liveFeedbackElem?: HTMLTextAreaElement;
  persistentFeedbackElem?: HTMLTextAreaElement;
  running: boolean;
  token?: {
    cancel?: () => void;
  }
};

export const testPaths = writable<Test[]>([]);