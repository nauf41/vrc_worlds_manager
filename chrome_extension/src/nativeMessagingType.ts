export namespace NativeMessaging {
  export type Message =
  | {type: "favorite-status", body: CheckFavorite}

  export type CheckFavorite = {
    uuid: string,
  }

  export type Response =
  | {type: "favorite-status", body: CheckFavoriteResponse}

  export type CheckFavoriteResponse = {
    uuid: string,
    isFavorite: boolean,
  }
}
