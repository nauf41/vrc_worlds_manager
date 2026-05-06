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
