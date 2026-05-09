import { attach_world, create_tag_group, createTag, get_discord_channels, get_discord_guilds, parse_channel, upsert_tag_group_attachment } from "@/models/db";
import { ChannelInfo, GuildInfo } from "@/types/discord";
import { TagGroup } from "@/types/tags";
import { create } from "zustand";

interface DiscordState {
  guilds: [GuildInfo, {categories: [ChannelInfo, ChannelInfo[]][], rootChannels: ChannelInfo[]}][],
  channels: ChannelInfo[],
}

export const useDiscordStore = create<DiscordState>((set) => ({
  guilds: [],
  channels: [],
}));

export function init() {
  setTimeout(async() => {
    const guilds = (await get_discord_guilds())!;
    console.log("guilds: ", guilds);
    const res: [GuildInfo, {categories: [ChannelInfo, ChannelInfo[]][], rootChannels: ChannelInfo[]}][] = [];

    const chas: ChannelInfo[] = [];
    for await (const guild of guilds) {
      const cha = await get_discord_channels(guild.id);
      (cha ?? []).forEach(v => v[1].forEach(c => chas.push(c)));

      console.log("Processing guild with channels: ", cha);

      const rootChannels: ChannelInfo[] = [];
      const categories: ChannelInfo[] = [];
      const now_res: [ChannelInfo, ChannelInfo[]][] = [];

      cha?.find(v => v[0] === null)?.[1].forEach(child => {
        if (child.is_category) {
          categories.push(child);
        } else {
          rootChannels.push(child);
        }
      })

      console.log("categories: ", categories, ", channels: ", rootChannels);

      cha?.filter(v => v[0] !== null).forEach(([category, children]) => {
        now_res.push([
          categories.find(v => v.id.toString() === category?.toString())!,
          children
        ]);
      })

      res.push([guild, {categories: now_res, rootChannels}]);
    }

    console.log("Discord channels: ", res);
    useDiscordStore.setState({guilds: res, channels: chas});
  }, 3000);
}

export async function importChannels(channels: ChannelInfo[]) {
  console.log("importing: ", channels);

  const data = useDiscordStore.getState().guilds;
  const rootChannels: string[] = [];
  data.forEach(guild => rootChannels.push(...guild[1].rootChannels.map(v => v.id)));
  const categories = data.flatMap(guild => guild[1].categories.map(v => v[0]));

  console.log("Now recognized root channels: ", rootChannels, ", categories: ", categories);

  let p = new Set<string>();
  channels.forEach(channel => {
    if (channel.parent_id) {
      p.add(channel.parent_id);
    }
  });
  const parents = [];
  for (const parent of p) {
    parents.push(parent);
  }

  console.log("Parents: ", parents);

  const taggroups: [string, TagGroup][] = await Promise.all(parents.map(async parent => {
    return [parent, (await create_tag_group(categories.find(cat => cat.id === parent)?.name || ""))!]
  }));

  console.log("Tag groups: ", taggroups);


  for await (const channel of channels) {
    const parsedWorlds = await parse_channel(channel.id);
    const worlds = Array.isArray(parsedWorlds) ? parsedWorlds : [];
    const tag = await createTag(channel.name);
    console.log("worlds parsed: ", parsedWorlds, ", tag created: ", tag);
    if (channel.parent_id !== null) {
      await upsert_tag_group_attachment(tag?.id!, taggroups?.find(v => v[0] === channel.parent_id)?.[1]?.id ?? null);
    }
    for (const world of worlds) {
      await attach_world(0, world.id);
      await attach_world(tag?.id!, world.id);
    }
  }
}
