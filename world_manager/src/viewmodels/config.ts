import { get_config } from "@/models/db";
import { create } from "zustand";

interface ConfigState {
  is_discord_bot_token_some: boolean | null,
}

export const useConfigStore = create<ConfigState>((set) => ({
  is_discord_bot_token_some: null,
}))

export function init() {
  get_config().then(conf => {
    useConfigStore.setState({is_discord_bot_token_some: !!conf?.discord_bot_token})
  });
}