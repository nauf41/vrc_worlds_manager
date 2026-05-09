export type GuildInfo = {
  id: string,
  name: string,
}

export type ChannelInfo = {
  id: string,
  name: string,
  is_category: boolean,
  parent_id: string | null,
}
