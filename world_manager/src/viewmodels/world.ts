import { World } from "@/types/world";
import { create } from "zustand";
import { Now as AppNow, useAppStore } from "./app";
import { getWorlds } from "@/models/db";
import { listen } from "@tauri-apps/api/event";
import { useTagStore } from "./tags";

interface WorldState {
  now: World[],
  updateWorld: (query: AppNow) => Promise<void>
}

export const useWorldStore = create<WorldState>((set) => ({
  now: [],
  updateWorld: async(query) => {
    switch (query.type) {
      case "settings": case "edit-tag": case "edit-tag-group": {
        set({now: []});
        break;
      }
      case "all-favorited": {
        set({now: await getWorlds({tag_id: 0}, "Recency") ?? []});
        break;
      }
      case "tagged": {
        set({now: await getWorlds({tag_id: query.tag.id}, "Recency") ?? []});
        break;
      }
      case "non-tagged": {
        set({now: await getWorlds({tag_id: -1}, "Recency") ?? []});
        break;
      }
      case "all": {
        set({now: await getWorlds({tag_id: null}, "Recency") ?? []});
      }
    }
  }
}));

export async function init() {
  const update = async () => {useTagStore.getState().update(); await useWorldStore.getState().updateWorld(useAppStore.getState().now).then(() => {console.log(useAppStore.getState(), useWorldStore.getState());})};
  listen("world-cache-updated", () => setTimeout(update, 300));
  listen("registered-status-updated", () => setTimeout(update, 300));
  listen("favorite-status-updated", () => setTimeout(update, 300));
  useAppStore.subscribe(state => state.now, now => useWorldStore.getState().updateWorld(now));
  await update();
}
