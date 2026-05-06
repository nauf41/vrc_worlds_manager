import { Button } from "@/components/ui/button";
import { Command, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList, CommandShortcut } from "@/components/ui/command";
import { FolderPlus, Grid, List, MoreHorizontal, MoreVertical, Plus, PlusIcon, Settings } from "lucide-react";
import { ToggleGroup, ToggleGroupItem } from "@/components/ui/toggle-group";
import { Badge } from "@/components/ui/badge";
import { useTagStore } from "@/viewmodels/tags";
import { AppState } from "@/viewmodels/app";
import { ScrollArea } from "@/components/ui/scroll-area";

export function SideBar(props: {state: AppState}) {
  const tags = useTagStore();

  return (
    <div className="col-span-4 flex flex-col border h-screen">
      <Command className="col-span-4 flex flex-col border">
        <CommandInput placeholder="Type a tag or search ..." />
        <div className="flex">
          <Button className="my-2 flex-grow-2" variant="secondary" onClick={() => {props.state.change_type({type: "create-tag-group"})}} style={props.state.now.type === "create-tag-group" ? {backgroundColor: "var(--color-gray-600) !important"} : {}}><FolderPlus />Add a tag folder</Button>
          <Button className="my-2 flex-grow-2" variant="secondary" onClick={() => {props.state.change_type({type: "create-tag"})}} style={props.state.now.type === "create-tag" && props.state.now.under === undefined ? {backgroundColor: "var(--color-gray-600) !important"} : {}}><PlusIcon />Add a tag</Button>
        </div>
        <ScrollArea className="flex-shrink overflow-y-auto min-h-0">
          <CommandList>
            <CommandEmpty>No tags found.</CommandEmpty>
            <CommandGroup>
              <CommandItem style={props.state.now.type === "all-favorited" ? {backgroundColor: "var(--color-gray-600) !important"} : {}} onSelect={() => {props.state.change_type({type: "all-favorited"})}}><Badge variant="outline">Auto</Badge>All</CommandItem>
              <CommandItem style={props.state.now.type === "all" ? {backgroundColor: "var(--color-gray-600) !important"} : {}} onSelect={() => {props.state.change_type({type: "all"})}}><Badge variant="outline">Auto</Badge>Recently Visited</CommandItem>
              <CommandItem style={props.state.now.type === "non-tagged" ? {backgroundColor: "var(--color-gray-600) !important"} : {}} onSelect={() => {props.state.change_type({type: "non-tagged"})}}><Badge variant="outline">Auto</Badge>Unclassified</CommandItem>
              {
                tags.tags_without_taggroups.filter(t => t.id !== 0).map((item, index) => ((
                  <CommandItem key={index} className="p-2" onSelect={() => {props.state.change_type({type: "tagged", tag: item})}} style={props.state.now.type === "tagged" && props.state.now.tag.id === item.id ? {backgroundColor: "var(--color-gray-600) !important"} : {}}>
                    <span>{item.name}</span>
                    <span className="hidden">{item.id}</span>
                    <CommandShortcut>
                      <Button size="icon" variant="ghost" className="h-4 w-4 opacity-0 transition-opacity group-hover/command-item:opacity-100" onClick={(e) => {props.state.change_type({type: "edit-tag", tag: item}); e.stopPropagation();}}>
                        <MoreHorizontal className="h-3 w-3" />
                      </Button>
                    </CommandShortcut>
                  </CommandItem>
                )))
              }
            </CommandGroup>
            { tags.taggroups.map(([group, children]) => (
              <CommandGroup heading={
                (
                  <div className="flex">
                    <span>{group.name}</span>
                    <div className="ml-auto flex gap-2">
                      <Button size="icon" variant="ghost" className="h-4 w-4" onClick={() => {props.state.change_type({type: "edit-tag-group", taggroup: group})}}><MoreHorizontal className="h-3 w-3" /></Button>
                      <Button size="icon" variant="ghost" className="h-4 w-4" onClick={() => {props.state.change_type({type: "create-tag", under: group})}}><Plus className="h-3 w-3" /></Button>
                    </div>
                  </div>
                )
              }>
                {
                  children.map(v => tags.tags.find(vv => vv[0].id === v)!).map((item, index) => ((
                    <CommandItem key={index} className="p-2" onSelect={() => {props.state.change_type({type: "tagged", tag: item[0]})}} style={props.state.now.type === "tagged" && props.state.now.tag.id === item[0].id ? {backgroundColor: "var(--color-gray-600) !important"} : {}}>
                      <span>{item[0].name}</span>
                      <span className="hidden">{item[0].id}</span>
                      <CommandShortcut>
                        <Button size="icon" variant="ghost" className="h-4 w-4 opacity-0 transition-opacity group-hover/command-item:opacity-100" onClick={(e) => {props.state.change_type({type: "edit-tag", tag: item[0]}); e.stopPropagation();}}>
                          <MoreHorizontal className="h-3 w-3" />
                        </Button>
                      </CommandShortcut>
                    </CommandItem>
                  )))
                }
              </CommandGroup>
            ))
            }
          </CommandList>
        </ScrollArea>
        <div className="mt-auto flex flex-col">
          <div className="flex flex-row items-center mx-2 pb-2 gap-2">
            <ToggleGroup className="" type="single" variant="outline" value={props.state.display} onValueChange={(e) => {props.state.change_display(e as ("grid" | "list"))}}>
              <ToggleGroupItem value="grid"><Grid /></ToggleGroupItem>
              <ToggleGroupItem value="list"><List /></ToggleGroupItem>
            </ToggleGroup>
            <Button className="p-2 flex-1" variant="outline" onClick={() => props.state.change_type({type: "settings"})} style={props.state.now.type === "settings" ? {backgroundColor: "var(--color-gray-600) !important"} : {}}><Settings /> Settings</Button>
          </div>
        </div>
      </Command>
    </div>
  )
}
