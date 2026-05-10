import { Button } from "@/components/ui/button"
import {
  Card,
  CardAction,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card"
import { ScrollArea } from "@/components/ui/scroll-area";
import { Table, TableBody, TableCell, TableHead, TableHeader, TableRow } from "@/components/ui/table";
import { attach_world } from "@/models/db";
import type { World as TWorld } from "@/types/world";
import { useAppStore } from "@/viewmodels/app";
import { useTagStore } from "@/viewmodels/tags";
import { useWorldStore } from "@/viewmodels/world";
import {BookmarkPlus, ExternalLink, Footprints, Plus, UserCheck} from "lucide-react";
import { open } from "@tauri-apps/plugin-shell";

export function Worlds() {
  const worlds = useWorldStore();

  return (
    <ScrollArea className="block h-screen w-full">
      <div className="grid grid-cols-12 h-full w-full gap-2">
        { worlds.now.map(world => ( <World key={world.id} world={world} /> )) }
      </div>
    </ScrollArea>
  )
}

export function WorldTable() {
  const tagStore = useTagStore();
  const worldStore = useWorldStore();

  return (
    <div className="h-full w-full mb-3">
      <ScrollArea className="h-full w-full max-h-screen">
        <Table className="table-fixed w-full">
          <TableHeader>
            <TableRow>
              <TableHead>Title</TableHead>
              <TableHead className="w-[50px] min-w-[50px] max-w-[50px]"><UserCheck className="inline" size="1.1em" /></TableHead>
              <TableHead className="w-[50px] min-w-[50px] max-w-[50px]"><Footprints className="inline" size="1.1em" /></TableHead>
              <TableHead></TableHead>
            </TableRow>
          </TableHeader>
          <TableBody>
          { worldStore.now.map(world => (
            <TableRow key={world.id} className="w-full">
              <TableCell className="w-full truncate">{world.title}</TableCell>
              <TableCell className="w-[50px] min-w-[50px] max-w-[50px]">{world.capacity}</TableCell>
              <TableCell className="w-[50px] min-w-[50px] max-w-[50px]">{world.self_visits}</TableCell>
              <TableCell>
                <div className="flex flex-row gap-1">
                  { !tagStore.tags.find(v => v[0].id === 0)![1].includes(world.id) && (
                    <Button className="p-2" onClick={async () => {await attach_world(0, world.id, false); await worldStore.updateWorld(useAppStore.getState().now); await tagStore.update();}}><Plus className="w-full h-full" /></Button>
                  )}
                  <Button className="p-2"><BookmarkPlus className="w-full h-full" /></Button>
                  <Button className="p-2" onClick={() => {open(`https://vrchat.com/home/world/${world.uuid}/info`);}}><ExternalLink className="w-full h-full" /></Button>
                </div>
              </TableCell>
            </TableRow>
          ))}
          </TableBody>
        </Table>
      </ScrollArea>
    </div>
  )
}

export function World(props: {world: TWorld}) {
  const tagStore = useTagStore();
  const worldStore = useWorldStore();
  const appStore = useAppStore();

  return (
    <div className="col-span-4">
      <Card className="relative mx-auto pt-0 h-full flex flex-col">
        <div className="absolute inset-0 z-30 aspect-video bg-black/35" />
        <img
          src="/testimage.png"
          alt={`sumbneil of ${props.world.title}`}
          className="relative z-20 aspect-video w-full object-cover"
        />
        <CardHeader className="flex-grow">
          <CardAction>
          </CardAction>
          <CardTitle>{props.world.title ?? props.world.uuid} <span className="text-gray-500">By</span> {props.world.publisher_name}</CardTitle>
          <CardDescription className="line-clamp-3">
            {props.world.description}
          </CardDescription>
        </CardHeader>
        <CardContent className="flex flex-row gap-2">
          <span className=""><UserCheck className="inline" size="1.1em" /> {props.world.capacity ?? "N/A"}</span>
          <span className=""><Footprints className="inline" size="1.1em" /> {props.world.self_visits ?? 0}</span>
        </CardContent>
        <CardFooter>
          <div className="flex flex-row w-full gap-1">
            { !tagStore.tags.find(v => v[0].id === 0)![1].includes(props.world.id) && (
              <Button className="p-2" onClick={async () => {await attach_world(0, props.world.id, false); await worldStore.updateWorld(useAppStore.getState().now); await tagStore.update();}}><Plus className="w-full h-full" /></Button>
            )}
            <Button className="p-2" onClick={() => {appStore.change_dialog({type: "world_tag", world: props.world})}}><BookmarkPlus className="w-full h-full" /></Button>
            <Button className="p-2" onClick={() => {open(`https://vrchat.com/home/world/${props.world.uuid}/info`);}}><ExternalLink className="w-full h-full" /></Button>
          </div>
        </CardFooter>
      </Card>
    </div>
  )
}
