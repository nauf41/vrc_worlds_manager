import type { NativeMessaging } from "./nativeMessagingType";

const connection = chrome.runtime.connectNative("io.github.nauf41.world_manager");
const dispatcher = new EventTarget;

function sendNativeMessage(message: NativeMessaging.Message) {
  connection.postMessage(message);
}

connection.onMessage.addListener((rawMessage: Object) => {
  const message = rawMessage as NativeMessaging.Response;
  console.log("Received native message:", message);
  dispatcher.dispatchEvent(new CustomEvent(message.id.toString(), {detail: message.body}));
});

function sendMessage<T extends NativeMessaging.Response["body"]["type"]>(
  message: NativeMessaging.Message["body"] & {type: T},
  handler: (detail: Extract<NativeMessaging.Response["body"], {type: T}>) => void
): Promise<Extract<NativeMessaging.Response["body"], {type: T}>> {
  return new Promise((resolve, reject) => {
    const id = Math.floor(Math.random() * 1 * 1000 * 1000 * 1000); // f64
    try {
      const listener: EventListener = (e) => {
        const event = e as CustomEvent<Extract<NativeMessaging.Response["body"], {type: T}>>;

        dispatcher.removeEventListener(id.toString(), listener);
        handler(event.detail);
        resolve(event.detail);
      }
      dispatcher.addEventListener(id.toString(), listener);
      sendNativeMessage({id, body: message});
    } catch (e) {
      reject(e);
    }
  })
}



export async function getFavoriteStatus(uuid: string): Promise<NativeMessaging.CheckFavoriteResponse> {
  return (await sendMessage(
    {type: "favorite-status", body: {uuid}},
    (res) => {}
  )).body;
}

export async function updateCache(world: NativeMessaging.World, cache: NativeMessaging.WorldCache): Promise<boolean> {
  return (await sendMessage(
    {type: "update-cache", body: {world, cache}},
    (res) => {}
  )).body;
}

export async function setRegistered(isRegistered: boolean, world: NativeMessaging.World): Promise<boolean> {
  return (await sendMessage(
    {type: "set-registered", body: {isRegistered, world}},
    (res) => {}
  )).body;
}
