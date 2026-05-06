import { Badge } from "@/components/ui/badge"
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
import { Field } from "@/components/ui/field";
import { ScrollArea } from "@/components/ui/scroll-area";

import type { World as TWorld } from "@/types/world";
import { useWorldStore } from "@/viewmodels/world";
import { BookmarkPlus, Footprints, Plus, UserCheck } from "lucide-react";

export function Worlds() {
  const worlds = useWorldStore();

  return (
    <ScrollArea className="block h-screen w-full">
      <div className="grid grid-cols-12 h-full w-full gap-2">
        { worlds.now.map(world => ( <World world={world} /> )) }
      </div>
    </ScrollArea>
  )
}

export function World(props: {world: TWorld}) {
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
            <Button className="p-2"><Plus className="w-full h-full" /></Button>
            <Button className="p-2"><BookmarkPlus className="w-full h-full" /></Button>
          </div>
        </CardFooter>
      </Card>
    </div>
  )
}