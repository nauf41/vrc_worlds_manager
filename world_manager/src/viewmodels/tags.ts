import { change_tag, create_tag_group, createTag, delete_tag, delete_tag_group, edit_tag_group_name, get_tag_groups_with_tags, get_tags, get_tags_with_children, get_tags_without_taggroup, upsert_tag_group_attachment } from "@/models/db";
import { Tag, TagGroup } from "@/types/tags";
import { create } from "zustand";

export interface TagState {
  tags: [Tag, number[]][],
  tags_without_taggroups: Tag[],
  taggroups: [TagGroup, number[]][],
  addTag: (name: string, under?: number) => Promise<void>,
  addTagGroup: (name: string) => Promise<void>,
  changeTagName: (tagid: number, name: string) => Promise<void>,
  changeTagGroupName: (taggroupid: number, name: string) => Promise<void>,
  removeTag: (tagid: number) => Promise<void>,
  removeTagGroup: (taggroupid: number) => Promise<void>,
  update: () => Promise<void>
}

function l<T>(a: T): T {
  console.log(a);
  return a;
}

export const useTagStore = create<TagState>((set, get) => ({
  tags: [],
  tags_without_taggroups: [],
  taggroups: [],
  update: async () => {
    const tags = l(await get_tags_with_children()) ?? [];
    const tags_without_taggroups = (await get_tags_without_taggroup()) ?? [];
    const taggroups = (await get_tag_groups_with_tags()) ?? [];

    set({tags, tags_without_taggroups, taggroups});

    console.log(get());
  },
  addTag: async (name, under) => {
    const tag = await createTag(name)!;
    if (under !== undefined) {
      await upsert_tag_group_attachment(tag?.id!, under);
    }
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

export async function init() {
  await useTagStore.getState().update();
}
