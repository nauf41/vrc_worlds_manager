import { World } from "@/types/world";
import { create } from "zustand";
import { Now as AppNow, useAppStore } from "./app";
import { getWorlds } from "@/models/db";

interface WorldState {
  now: World[],
  updateWorld: (query: AppNow) => Promise<void>
}

export const useWorldState = create<WorldState>((set) => ({
  now: [],
  updateWorld: async(query) => {
    switch (query.type) {
      case "settings": case "edit-tag": case "edit-tag-group": {
        set({now: []});
        break;
      }
      case "all-favorited": {
        set({now: await getWorlds({tag_id: null, registered: true, classified: null}, "Recency") ?? []});
        break;
      }
      case "tagged": {
        set({now: await getWorlds({tag_id: query.tag.id, registered: null, classified: null}, "Recency") ?? []});
        break;
      }
      case "non-tagged": {
        set({now: await getWorlds({tag_id: -1, registered: null, classified: null}, "Recency") ?? []});
        break;
      }
      case "all": {
        set({now: await getWorlds({tag_id: null, registered: null, classified: null}, "Recency") ?? []});
      }
    }
  }
}));

export function init() {
  useAppStore.subscribe(state => state.now, now => useWorldState.getState().updateWorld(now))
}
