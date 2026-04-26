import type { NativeMessaging } from "./nativeMessagingType";

const connection = chrome.runtime.connectNative("io.github.nauf41.world_manager");
const dispatcher = new EventTarget;

function sendNativeMessage(message: NativeMessaging.Message) {
  connection.postMessage(JSON.stringify(message));
}

connection.onMessage.addListener((rawMessage: string) => {
  const message = JSON.parse(rawMessage) as NativeMessaging.Response;
  dispatcher.dispatchEvent(new CustomEvent(message.type, {detail: message.body}));
});

export function getFavoriteStatus(uuid: string): Promise<NativeMessaging.CheckFavoriteResponse> {
  return new Promise((resolve, reject) => {
    const handler: EventListener = (event) => {
      const customEvent = event as CustomEvent<NativeMessaging.CheckFavoriteResponse>;
      if (customEvent.detail.uuid === uuid) {
        resolve(customEvent.detail);
        dispatcher.removeEventListener("favorite-status", handler);
      }
    }
    dispatcher.addEventListener("favorite-status", handler);
    sendNativeMessage({type: "favorite-status", body: {uuid}});
  });
}
