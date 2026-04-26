import type { Messaging } from "./messagingType";
import "./nativeMessaging";
import { getFavoriteStatus } from "./nativeMessaging";

chrome.runtime.onMessage.addListener(async(message: Messaging.Message, sender, sendResponse) => {
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
  }
})
