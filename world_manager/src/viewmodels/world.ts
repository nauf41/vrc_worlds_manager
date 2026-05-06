import { World } from "@/types/world";
import { create } from "zustand";
import { Now as AppNow, useAppStore } from "./app";
import { getWorlds } from "@/models/db";
import { listen } from "@tauri-apps/api/event";

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

export function init() {
  const update = () => useWorldStore.getState().updateWorld(useAppStore.getState().now);
  listen("world-cache-updated", update);
  listen("registered-status-updated", update);
  listen("favorite-status-updated", update);
  update();
  useAppStore.subscribe(state => state.now, now => useWorldStore.getState().updateWorld(now));
}
