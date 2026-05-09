import { Tag, TagGroup } from "@/types/tags";
import { World } from "@/types/world";
import { create } from "zustand";
import { subscribeWithSelector } from "zustand/middleware";

export interface AppState {
  now: Now,
  display: "grid" | "list",
  change_display: (after: "grid" | "list") => void,
  change_type: (query: Now) => void,
  dialog: Dialog,
  change_dialog: (d: Dialog) => void,
}

export const useAppStore = create<AppState>()(subscribeWithSelector((set) => ({
  now: {type: "all-favorited"},
  display: "grid",
  change_display: (after) => { set({display: after}) },
  change_type: (query) => {
    set({now: query})
  },
  dialog: {type: "none"},
  change_dialog: (d) => {
    set({dialog: d});
  },
})));

export type Now =
  | {type: "settings"}
  | {type: "create-tag", under?: TagGroup}
  | {type: "edit-tag", tag: Tag}
  | {type: "create-tag-group"}
  | {type: "edit-tag-group", taggroup: TagGroup}
  | {type: "non-tagged"}
  | {type: "all-favorited"}
  | {type: "all"}
  | {type: "tagged", tag: Tag}
  | {type: "import-tags"}

export type Dialog =
  | {type: "none"}
  | {type: "world_tag", world: World}
