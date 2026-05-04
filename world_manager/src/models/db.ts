import { invoke } from "@tauri-apps/api/core";
import { SortBy, World, WorldQueryFilters } from "../types/world";
import { useTagsStore } from "../viewmodels/tags";
import type {TagGroup, Tag} from "../types/tags";

export function getWorlds(filter: WorldQueryFilters, sortBy: SortBy): Promise<World[]> {
  return new Promise(async(res, rej) => {
    try {
      const result = await invoke("get_worlds", {filter, sortBy});
      res(result as World[]);
    } catch (err) {
      rej(err);
    }
  })
}

export function getTags(): Promise<Tag[]> {
  return new Promise(async(res, rej) => {
    try {
      const result = await invoke("get_tags");
      res(result as Tag[]);
    } catch (err) {
      rej(err);
    }
  })
}

export function createTag(name: string): Promise<Tag> {
  return new Promise(async(res, rej) => {
    try {
      const result = await invoke("create_tag", {name});
      res(result as Tag);
      useTagsStore.getState().update();
    } catch (err) {
      rej(err);
    }
  })
}

export async function changeTag(tagid: number, data: Tag): Promise<void> {
  await invoke("change_tag", {tagid, data});
}

export async function deleteTag(tagid: number): Promise<void> {
  await invoke("delete_tag", {tagid});
}

export function createTagGroup(name: string): Promise<TagGroup | null> {
  return invoke("create_tag_group", {name}) as Promise<TagGroup | null>;
}

export function getTagGroups(): Promise<TagGroup[] | null> {
  return invoke("get_tag_groups", {}) as Promise<TagGroup[] | null>
}

export function editTaggroupName(taggroupid: number, name: string): Promise<boolean> {
  return invoke("edit_tag_group_name", { taggroupid, name }) as Promise<boolean>
}

export function deleteTagGroup(taggroupid: number): Promise<boolean> {
  return invoke("delete_tag_group", { taggroupid }) as Promise<boolean>
}

export function upsertTagGroupAttachment(tagid: number, taggroupid: number | null): Promise<boolean> {
  return invoke("upsert_tag_group_attachment", { tagid, taggroupid }) as Promise<boolean>
}
