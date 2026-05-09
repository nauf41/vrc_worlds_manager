export type World = {
  id: number;
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
  image_cache_file?: string; // base64-encoded
  self_visits?: number;
}

export type WorldQueryFilters = {
  tag_id: number | null,
}

export type SortBy = "Recency";

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
  image_cache?: string; // base64-encoded
}


export type WorldDBStructure = {
  id: number,
  uuid: string,
  publisher_uuid: string | null,
  publisher_name: string | null,
  registered_at: number | null,
  description: string | null,
  title: string | null,
  visits: number | null,
  favorites: number | null,
  capacity: number | null,
  published_at: number | null,
  does_support_windows: number | null,
  does_support_android: number | null,
  does_support_ios: number | null,
  latest_at: number | null,
  image_cache_file: string | null,
}