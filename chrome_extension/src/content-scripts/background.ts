import type { Messaging } from "../messagingType";

function sendRequest<T extends Messaging.Response["type"]>(message: Messaging.Message & {type: T}): Promise<Messaging.Response & {type: T}> {
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

export function setRegistered(isRegistered: boolean, {uuid}: {uuid: string}): Promise<Messaging.Response & {type: "set-registered"}> {
  return sendRequest({type: "set-registered", body: {isRegistered, world: {uuid}}});
}

export function updateCache(world: Messaging.World, cache: Messaging.WorldCache): Promise<Messaging.Response & {type: "update-cache"}> {
  return sendRequest({type: "update-cache", body: {world, cache}});
}
