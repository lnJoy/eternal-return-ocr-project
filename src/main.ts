import { invoke } from "@tauri-apps/api/tauri";
import { isRegistered, register, unregister } from "@tauri-apps/api/globalShortcut";
import { LogicalSize, appWindow } from '@tauri-apps/api/window';
import { currentMonitor } from '@tauri-apps/api/window';

import { createWorker } from "tesseract.js";

const toggleWindow = async () => {
  const isWindowOpen = await appWindow.isVisible();
  if (isWindowOpen) {
    await appWindow.show();
    await appWindow.setFocus();
  }
};

const recognize = async (url: string): Promise<string> => {
  const worker = await createWorker("eng+kor+jpn+chi_sim", 1);
  const { data: { text } } = await worker.recognize(url);
  await worker.terminate();
  return text;
}

const process = async () => {
  const processedImage: string[] = await invoke("process");
  processedImage.reduce(async (_acc, cur, idx): Promise<void> => {
    let playerIFrame = document.querySelector("#player-iframe-" + (idx + 1)) as HTMLIFrameElement;
    if (playerIFrame) {
      const resultText = (await recognize("data:image/jpg;base64," + cur)).trim();
      console.log("https://dak.gg/er/players/" + resultText);
      playerIFrame.src = "https://dak.gg/er/players/" + resultText;
    }  
  }, {});
  toggleWindow();
}

const init = async() => {
  const monitor = await currentMonitor();
  if (monitor) {
    await appWindow.setSize(new LogicalSize(
      Math.floor(monitor?.size.width / 1.8), 
      Math.floor(monitor?.size.height / 1.8), 
    ));
  }
  
  await unregister('F10');
  try {
    const isShortkeyAlreadyRegistered = await isRegistered(
      'F10'
    );
    if (!isShortkeyAlreadyRegistered) {
      await register('F10', () => process());
    }
  } catch (error) {
    console.log(error);
  }
}

init();

window.addEventListener("DOMContentLoaded", async () => {
  document.querySelector("#greet-form")?.addEventListener("submit", (e) => {
    e.preventDefault();
    process();
  });
});

