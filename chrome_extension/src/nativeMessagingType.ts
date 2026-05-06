export namespace NativeMessaging {
  export type Message = {
    id: number,
    body: (
      | {type: "favorite-status", body: CheckFavorite}
      | {type: "update-cache", body: UpdateCache}
      | {type: "set-registered", body: SetRegistered}
    )
  }

  export type CheckFavorite = {
    uuid: string,
  }

  export type UpdateCache = {
    world: WorldQuery,
  }

  export type SetRegistered = {
    isRegistered: boolean,
    world: string,
  }

  export type Response = {
    id: number,
    body: (
      | {type: "favorite-status", body: CheckFavoriteResponse}
      | {type: "update-cache", body: boolean}
      | {type: "set-registered", body: boolean}
    )
  }

  export type CheckFavoriteResponse = {
    uuid: string,
    isFavorite: boolean,
  }

  export type WorldQuery = {
    uuid: string;
    publisher_uuid?: string;
    publisher_name?: string;
    registered_at?: number;
    description?: string;
    title?: string;
    visits?: number;
    favorites?: number;
    capacity?: number;
    published_at?: number;
    does_support_windows?: number;
    does_support_android?: number;
    does_support_ios?: number;
    latest_at?: number;
  }
}
