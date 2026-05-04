import { create } from "zustand"
import { getTags } from "../models/db"
import { listen } from "@tauri-apps/api/event";

interface TagsState {
  tags: Tag[],
  update: () => Promise<void>,
}

export const useTagsStore = create<TagsState>((set) => ({
  tags: [],
  update: async() => set({tags: await getTags()}),
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
