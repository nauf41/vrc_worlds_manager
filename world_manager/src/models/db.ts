import { Tag, TagGroup } from "@/types/tags";
import { SortBy, World, WorldQuery, WorldQueryFilters } from "@/types/world";
import { invoke } from "@tauri-apps/api/core";

export async function upsert_world(query: WorldQuery): Promise<boolean> {
  return await invoke("upsert_world", { query });
}

export async function getWorlds(filter: WorldQueryFilters, sortBy: SortBy): Promise<World[] | null> {
  return await invoke("get_worlds", { filter, sortBy });
}

export async function createTag(name: string): Promise<Tag | null> {
  return await invoke("create_tag", { name });
}

export async function get_tags(): Promise<Tag[] | null> {
  return await invoke("get_tags");
}

export async function get_tags_with_children(): Promise<[Tag, number[]][] | null> {
  return await invoke("get_tags_with_children");
}

export async function get_tags_without_taggroup(): Promise<Tag[] | null> {
  return await invoke("get_tags_without_taggroup");
}

export async function get_favorited_worlds(): Promise<number[] | null> {
  return await invoke("get_favorited_worlds");
}

export async function attach_world(tagid: number, worldid: number): Promise<boolean> {
  return await invoke("attach_world", { tagid, worldid });
}

export async function detach_world(tagid: number, worldid: number): Promise<boolean> {
  return await invoke("detach_world", { tagid, worldid });
}

export async function change_tag(tagid: number, name: string): Promise<boolean> {
  return await invoke("change_tag", { tagid, data: { id: tagid, name } as Tag });
}

export async function delete_tag(tagid: number): Promise<boolean> {
  return await invoke("delete_tag", { tagid });
}

export async function create_tag_group(name: string): Promise<TagGroup | null> {
  return await invoke("create_tag_group", { name });
}

export async function get_tag_groups(): Promise<TagGroup[] | null> {
  return await invoke("get_tag_groups");
}

export async function get_tag_groups_with_tags(): Promise<[TagGroup, number[]][] | null> {
  return await invoke("get_tag_groups_with_tags");
}

export async function edit_tag_group_name(taggroupid: number, name: string): Promise<boolean> {
  return await invoke("edit_tag_group_name", { taggroupid, name });
}

export async function delete_tag_group(taggroupid: number): Promise<boolean> {
  return await invoke("delete_tag_group", { taggroupid });
}

export async function upsert_tag_group_attachment(tagid: number, taggroupid: number | null): Promise<boolean> {
  return await invoke("upsert_tag_group_attachment", { tagid, taggroupid });
}
