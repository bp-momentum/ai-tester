// eslint-disable-next-line @typescript-eslint/ban-types
export const setIntervalX = (callback: Function, delay: number, repetitions: number) => {
  let res: (value: boolean) => void;
  const promise = new Promise<boolean>((resolve,) => {
    res = resolve;
  });
  let x = 0;
  const intervalID = window.setInterval(function () {
    callback();

    if (++x === repetitions) {
      window.clearInterval(intervalID);
      res(true);
    }
  }, delay);

  return promise;
}