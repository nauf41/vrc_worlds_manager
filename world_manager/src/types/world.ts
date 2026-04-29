export type World = {
  id: number,
  uuid: string,
  publisher: number,
  publisher_name?: string,
  description?: string,
  title?: string,
  visits?: number,
  favorites?: number,
  capacity?: number,
  published_at?: number,
  supports_windows?: boolean,
  supports_android?: boolean,
  supports_ios?: boolean,
}

export type WorldQueryFilters = {
  tag_id: number | null,
  registered: boolean | null,
  classified: boolean | null,
}

export type SortBy = "Recency";
