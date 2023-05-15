import { invoke } from "@tauri-apps/api";
import { listen, type UnlistenFn } from "@tauri-apps/api/event";
import { setIntervalX } from "./helper";

export const testCleanup = async (
  unlisten: Promise<UnlistenFn>[],
  videoElem: HTMLVideoElement,
) => {
  unlisten.forEach(async element => {
    (await element)();
  });
  
  videoElem.pause();
  videoElem.currentTime = 0;
  videoElem.controls = true;
  videoElem.muted = false;
}

export const runTest = async (
  videoElem: HTMLVideoElement,
  liveFeedbackElem: HTMLTextAreaElement,
  persistentFeedbackElem: HTMLTextAreaElement,
  aiHost: string,
  aiPort: string,
  framesPerRep: number,
  token: {
    cancel?: () => void;
  },
): Promise<boolean> => {
  let res: (arg0: boolean) => void;
  const running = new Promise<boolean>((resolve,) => {
    res = resolve;
    token.cancel = async () => {
      await invoke("disconnect_ws", {
        "uuid": uuid
      });

      testCleanup(unlisten, videoElem);

      res(false);
    }
  });
  
  videoElem.controls = false;
  videoElem.muted = true;
  videoElem.pause();
  videoElem.currentTime = 0;

  liveFeedbackElem.value = "";
  persistentFeedbackElem.value = "";

  const unlisten: Promise<UnlistenFn>[] = [];

  unlisten.push(listen("live_feedback", (data) => {
    console.log(data.payload);
    liveFeedbackElem.value += JSON.stringify(data.payload, null, 2) + "\n";
  }));

  unlisten.push(listen("persistent_feedback", (data) => {
    persistentFeedbackElem.value = JSON.stringify(data.payload, null, 2);
    
    testCleanup(unlisten, videoElem);

    res(true);
  }));

  unlisten.push(listen("ws_connected", async () => {
    const fps = 10;
    const nReps = Math.floor(Math.round(videoElem.duration * fps) / framesPerRep);

    for (let i = 0; i < nReps; i++) {
      for (let j = 0; j < framesPerRep; j++) {
        videoElem.currentTime = (i * framesPerRep + j) / fps;
        const canvas = document.createElement('canvas');
        canvas.width = videoElem.videoWidth;
        canvas.height = videoElem.videoHeight;
        canvas.getContext('2d')?.drawImage(videoElem, 0, 0, canvas.width, canvas.height);
        // send image blob to ai
        invoke("send_image", {
          "image": canvas.toDataURL('image/jpeg', 0.5),
          "uuid": uuid
        });
      }
      await invoke("end_repetition", {
        "uuid": uuid
      });
    }

    await invoke("end_set", {
      "uuid": uuid
    });
  }));

  const uuid = await invoke("connect_ws", {
    "url": `http://${aiHost}:${aiPort}`,
  });

  return running;
}