import { create } from "zustand/react";
import { World } from "../types/world";
import { getWorlds } from "../models/db";
import { listen } from "@tauri-apps/api/event";

export interface AppState {
  now: NowSelected,
  display: Display,
  change_type: (query: ChangeTypeQuery) => Promise<void>,
  change_display: (display: Display) => void,
  update: () => Promise<void>,
}

export const useAppStore = create<AppState>((set, get) => ({
  now: {type: "dashboard"},
  display: "grid",
  change_type: async (query) => {
    switch (query.type) {
      case "edit_category":
        set({now: {type: "edit_category", form: {name: ""}, category_id: query.category_id}});
        break;

      case "add_world":
        set({now: {type: "add_world", form: {world_url: "", tags: []}}});
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
  }
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

type NowSelected =
| {type: "edit_category", form: CreateCategoryForm, category_id: number}
| {type: "add_world", form: AddWorldForm}
| {type: "settings", form: SettingsForm}
| {type: "dashboard"}
| {type: "all-worlds", worlds: World[]}
| {type: "recent-worlds", worlds: World[]}
| {type: "unclassified-worlds", worlds: World[]}
| {type: "tag", tag: Tag, worlds: World[]}

type ChangeTypeQuery =
| {type: "edit_category", category_id: number}
| {type: "add_world"}
| {type: "settings"}
| {type: "dashboard"}
| {type: "all-worlds"}
| {type: "recent-worlds"}
| {type: "unclassified-worlds"}
| {type: "tag", tag_id: number}

type Tag = {
  id: number,
  name: string,
}

type Display = "grid" | "list";

type CreateCategoryForm = {
  name: string,
}

type AddWorldForm = {
  world_url: string,
  tags: Tag[],
}

type SettingsForm = {
  use_discord_link: boolean,
  discord_token: string,
}

