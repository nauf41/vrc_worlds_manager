import { createTagGroup, getTagGroups } from "@/models/db"
import { Tag, TagGroup } from "@/types/tags"
import { create } from "zustand"

interface TagGroupState {
  groups: TagGroup[],
  create: (name: string) => Promise<void>,
  update: () => Promise<void>,
  change_name: (name: string) => Promise<void>,
  delete: (id: number) => Promise<void>,
}

export const useTagGroupStore = create<TagGroupState>((set, get) => ({
  groups: [],
  create: async (name: string) => {
    await createTagGroup(name);
    get().update();
  }
  update: async () => {
    set({groups: await getTagGroups()})
  }
}))