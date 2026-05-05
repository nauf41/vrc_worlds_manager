import { change_tag, create_tag_group, createTag, delete_tag, delete_tag_group, edit_tag_group_name, get_tag_groups_with_tags, get_tags, get_tags_without_taggroup } from "@/models/db";
import { Tag, TagGroup } from "@/types/tags";
import { create } from "zustand";

export interface TagState {
  tags: Tag[],
  tags_without_taggroups: Tag[],
  taggroups: [TagGroup, number[]][],
  addTag: (name: string) => Promise<void>,
  addTagGroup: (name: string) => Promise<void>,
  changeTagName: (tagid: number, name: string) => Promise<void>,
  changeTagGroupName: (taggroupid: number, name: string) => Promise<void>,
  removeTag: (tagid: number) => Promise<void>,
  removeTagGroup: (taggroupid: number) => Promise<void>,
  update: () => Promise<void>
}

export const useTagStore = create<TagState>((set) => ({
  tags: [],
  tags_without_taggroups: [],
  taggroups: [],
  update: async () => {
    const tags = (await get_tags()) ?? [];
    const tags_without_taggroups = (await get_tags_without_taggroup()) ?? [];
    const taggroups = (await get_tag_groups_with_tags()) ?? [];

    set({tags, tags_without_taggroups, taggroups});
  },
  addTag: async (name) => {
    await createTag(name);
    await useTagStore.getState().update();
  },
  addTagGroup: async (name) => {
    await create_tag_group(name);
    await useTagStore.getState().update();
  },
  changeTagName: async (tagid, name) => {
    await change_tag(tagid, name);
    await useTagStore.getState().update();
  },
  changeTagGroupName: async (taggroupid, name) => {
    await edit_tag_group_name(taggroupid, name);
    await useTagStore.getState().update();
  },
  removeTag: async (tagid) => {
    await delete_tag(tagid);
    await useTagStore.getState().update();
  },
  removeTagGroup: async (taggroupid) => {
    await delete_tag_group(taggroupid);
    await useTagStore.getState().update();
  },
}));

export function init() {
  useTagStore.getState().update();
}
