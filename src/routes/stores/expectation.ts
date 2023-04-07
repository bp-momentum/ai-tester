import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/tauri'

export const inputType = writable<"video" | "string">("video");
export const videoPath = writable<string>("");
export const jsonObject = writable<string>("");
export const jsonValid = writable<boolean>(true);

jsonObject.subscribe((value) => {
  let $inputType;
  inputType.subscribe($ => $inputType = $);
  if ($inputType === "string") {
    invoke('set_expectation_landmarks', { landmarks: value })
      .then(res => jsonValid.set(res as boolean));
  }
});

videoPath.subscribe((value) => {
  let $inputType;
  inputType.subscribe($ => $inputType = $);
  if ($inputType === "video") {
    invoke('set_expectation_video', { video: value });
  }
});