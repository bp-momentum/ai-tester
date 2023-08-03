import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/tauri'
import type { NormalizedLandmark } from '@mediapipe/tasks-vision';

export const inputType = writable<"video" | "string">("string");
export const frames_per_repetition = writable<number>(0);

export const videoPath = writable<string>("");
export const videoJson = writable<NormalizedLandmark[][] | undefined>(undefined);
export const videoValid = writable<boolean | undefined>(undefined);
export const videoJustSet = writable<boolean>(false);

export const jsonObject = writable<string>("");
export const jsonValid = writable<boolean>(true);

jsonObject.subscribe((value) => {
  let $inputType;
  inputType.subscribe($ => $inputType = $);
  if ($inputType !== "string") return;
  
  invoke('unset_expectation');
  invoke('set_expectation_landmarks', { landmarks: value })
    .then(res => {
      jsonValid.set(res as boolean);
      if (value !== "")
        frames_per_repetition.set(JSON.parse(value).length);
    });
});

videoPath.subscribe((value) => {
  let $inputType;
  inputType.subscribe($ => $inputType = $);
  if ($inputType !== "video") return;

  videoJson.set(undefined);
  videoValid.set(undefined);
  if (value !== "")
    videoJustSet.set(true);
  invoke('unset_expectation');
});

videoJson.subscribe((value) => {
  let $inputType;
  inputType.subscribe($ => $inputType = $);
  if ($inputType !== "video") return;
  if (!value) return;

  invoke('set_expectation_landmarks', { landmarks: JSON.stringify(value) })
    .then(res => {
      videoValid.set(res as boolean);
      frames_per_repetition.update(() => value.length);
    });
});