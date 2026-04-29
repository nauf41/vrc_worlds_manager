import { invoke } from "@tauri-apps/api/core";
import { SortBy, World, WorldQueryFilters } from "../types/world";
import { Tag, useTagsStore } from "../viewmodels/tags";

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