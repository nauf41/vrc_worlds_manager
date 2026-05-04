import { create } from "zustand"
import { changeTag, deleteTag, getTags } from "../models/db"
import { listen } from "@tauri-apps/api/event";
import { invoke } from "@tauri-apps/api/core";

interface TagsState {
  tags: Tag[],
  update: () => Promise<void>,
  remove: (tag_id: number) => Promise<void>,
  change: (tag_id: number, after: Tag) => Promise<void>,
}

export const useTagsStore = create<TagsState>((set) => ({
  tags: [],
  update: async() => set({tags: await getTags()}),
  remove: async(tag_id: number) => {await deleteTag(tag_id); await useTagsStore.getState().update()},
  change: async(tag_id: number, after: Tag) => {await changeTag(tag_id, after); await useTagsStore.getState().update()},
}))

listen('new-world', () => {
  useTagsStore.getState().update();
})

listen('favorite-status-updated', () => {
  useTagsStore.getState().update();
})

listen('world-cache-updated', () => {
  useTagsStore.getState().update();
})

listen('registered-status-updated', () => {
  useTagsStore.getState().update();
})

export type Tag = {
  id: number,
  name: string,
}
