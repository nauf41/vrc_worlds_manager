import type { Messaging } from "./messagingType";
import "./nativeMessaging";
import { getFavoriteStatus, setRegistered, updateCache } from "./nativeMessaging";

chrome.runtime.onMessage.addListener(async(message: Messaging.Message, sender, sendResponse) => {
  console.log("Sending message:", message);
  switch (message.type) {
    case "favorite-status": {
      const {uuid} = message.body;
      sendResponse({
        type: "favorite-status",
        body: {
          uuid,
          isFavorite: (await getFavoriteStatus(uuid)).isFavorite,
        }
      } as Messaging.Response);
      break;
    }
    case "update-cache": {
      const {world} = message.body;

      const result = await updateCache({...world});
      sendResponse({
        type: "update-cache",
        body: result,
      } as Messaging.Response);
      break;
    }
    case "set-registered": {
      const {isRegistered, world} = message.body;
      const result = await setRegistered(isRegistered, world);
      sendResponse({
        type: "set-registered",
        body: result,
      } as Messaging.Response);
      break;
    }
  }
})
