import { create } from "zustand"
import { getTags } from "../models/db"

interface TagsState {
  tags: Tag[],
  update: () => Promise<void>,
}

export const useTagsStore = create<TagsState>((set) => ({
  tags: [],
  update: async() => set({tags: await getTags()}),
}))

export type Tag = {
  id: number,
  name: string,
}
