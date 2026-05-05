import { create } from "zustand/react";
import { World } from "../types/world";
import { getWorlds } from "../models/db";
import { listen } from "@tauri-apps/api/event";
import { Tag } from "@/types/tags";

export interface AppState {
  now: NowSelected,
  display: Display,
  change_type: (query: ChangeTypeQuery) => Promise<void>,
  change_display: (display: Display) => void,
  update: () => Promise<void>,
  update_with: (a: NowSelected) => void,
}

export const useAppStore = create<AppState>((set, get) => ({
  now: {type: "dashboard"},
  display: "grid",
  change_type: async (query) => {
    switch (query.type) {
      case "edit_category":
        set({now: {type: "edit_category", form: query.form, category_id: query.category_id}});
        break;

      case "settings":
        set({now: {type: "settings", form: {use_discord_link: false, discord_token: ""}}});
        break;

      case "dashboard":
        set({now: {type: "dashboard"}});
        break;

      case "all-worlds": {
        set({now: {type: "all-worlds", worlds: await getWorlds({registered: true, classified: null, tag_id: null}, "Recency")}});
        break;
      }

      case "recent-worlds": {
        set({now: {type: "recent-worlds", worlds: await getWorlds({registered: null, classified: null, tag_id: null}, "Recency")}});
        break;
      }

      case "unclassified-worlds": {
        set({now: {type: "unclassified-worlds", worlds: await getWorlds({registered: true, classified: false, tag_id: null}, "Recency")}});
        break;
      }
    }
  },
  change_display: (display) => set({ display }),
  update: async () => {
    const state = get();
    switch (state.now.type) {
      case "all-worlds": {
        set({now: {type: "all-worlds", worlds: await getWorlds({registered: true, classified: null, tag_id: null}, "Recency")}});
        break;
      }

      case "recent-worlds": {
        set({now: {type: "recent-worlds", worlds: await getWorlds({registered: null, classified: null, tag_id: null}, "Recency")}});
        break;
      }

      case "unclassified-worlds": {
        set({now: {type: "unclassified-worlds", worlds: await getWorlds({registered: true, classified: false, tag_id: null}, "Recency")}});
        break;
      }
    }
  },
  update_with: (a) => set({now: a}),
}))

listen('new-world', () => {
  useAppStore.getState().update();
})

listen('favorite-status-updated', () => {
  useAppStore.getState().update();
})

listen('world-cache-updated', () => {
  useAppStore.getState().update();
})

listen('registered-status-updated', () => {
  useAppStore.getState().update();
})

export type NowSelected =
| {type: "edit_category", form: CreateCategoryForm, category_id: number}
| {type: "settings", form: SettingsForm}
| {type: "dashboard"}
| {type: "all-worlds", worlds: World[]}
| {type: "recent-worlds", worlds: World[]}
| {type: "unclassified-worlds", worlds: World[]}
| {type: "tag", tag: Tag, worlds: World[]}

export type ChangeTypeQuery =
| {type: "edit_category", category_id: number, form: CreateCategoryForm}
| {type: "settings"}
| {type: "dashboard"}
| {type: "all-worlds"}
| {type: "recent-worlds"}
| {type: "unclassified-worlds"}
| {type: "tag", tag_id: number}

export type Display = "grid" | "list";

export type CreateCategoryForm = {
  name: string,
}

export type SettingsForm = {
  use_discord_link: boolean,
  discord_token: string,
}

