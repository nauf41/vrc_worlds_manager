export namespace Messaging {
  export type Message =
  | {type: "favorite-status", body: CheckFavoriteStatusMessage}

  export type CheckFavoriteStatusMessage = {
    uuid: string,
  }

  export type Response =
  | {type: "favorite-status", body: CheckFavoriteStatusResponse}

  export type CheckFavoriteStatusResponse = {
    uuid: string,
    isFavorite: boolean,
  }
}