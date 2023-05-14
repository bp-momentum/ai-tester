import { writable } from 'svelte/store';
import { invoke } from '@tauri-apps/api/tauri'

export const inputType = writable<"video" | "string">("string");

export const videoPath = writable<string>("");
export const videoJson = writable<string>("");
export const videoValid = writable<boolean | undefined>(undefined);
export const videoJustSet = writable<boolean>(false);

export const jsonObject = writable<string>("");
export const jsonValid = writable<boolean>(true);
export const landmarks = writable<Array<Array<object>>|undefined>();

jsonObject.subscribe((value) => {
  let $inputType;
  inputType.subscribe($ => $inputType = $);
  if ($inputType !== "string") return;
  
  invoke('unset_expectation');
  invoke('set_expectation_landmarks', { landmarks: value })
    .then(res => jsonValid.set(res as boolean));
});

videoPath.subscribe((value) => {
  let $inputType;
  inputType.subscribe($ => $inputType = $);
  if ($inputType !== "video") return;

  videoJson.set("");
  videoValid.set(undefined);
  if (value !== "")
    videoJustSet.set(true);
  invoke('unset_expectation');
});

videoJson.subscribe((value) => {
  let $inputType;
  inputType.subscribe($ => $inputType = $);
  if ($inputType !== "video") return;
  
  if (value !== "")
    invoke('set_expectation_landmarks', { landmarks: value })
      .then(res => 
        videoValid.set(res as boolean)
      );
});