import type { Messaging } from "../messagingType";

function sendRequest(message: Messaging.Message): Promise<Messaging.Response> {
  return new Promise(async (resolve, reject) => {
    try {
      resolve(await chrome.runtime.sendMessage(message));
    } catch (error) {
      reject(error);
    }
  })
}

export function checkFavoriteStatus(uuid: string): Promise<Messaging.Response & {type: "favorite-status"}> {
  return sendRequest({type: "favorite-status", body: {uuid}});
}
