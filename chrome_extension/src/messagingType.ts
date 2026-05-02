export namespace Messaging {
  export type Message =
    | {type: "favorite-status", body: CheckFavorite}
    | {type: "update-cache", body: UpdateCache}
    | {type: "set-registered", body: SetRegistered}

  export type CheckFavorite = {
    uuid: string,
  }

  export type UpdateCache = {
    world: World,
    cache: WorldCache,
  }

  export type SetRegistered = {
    isRegistered: boolean,
    world: World,
  }

  export type Response =
    | {type: "favorite-status", body: CheckFavoriteResponse}
    | {type: "update-cache", body: boolean}
    | {type: "set-registered", body: boolean}

  export type CheckFavoriteResponse = {
    uuid: string,
    isFavorite: boolean,
  }

  export type World = {
    uuid: string,
  }

  export type WorldCache = {
    description: string | null,
    title: string | null,
    visits: number | null,
    favorites: number | null,
    capacity: number | null,
    published_at: number | null, // timestamp, in milliseconds
    does_support_windows: boolean | null,
    does_support_android: boolean | null,
    does_support_ios: boolean | null,
  }
}
