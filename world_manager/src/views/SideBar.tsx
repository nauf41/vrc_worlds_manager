import { AppState } from "../viewmodels/app";
import { useTagsStore } from "../viewmodels/tags";
import { Button } from "@/components/ui/button";
import { Command, CommandEmpty, CommandGroup, CommandInput, CommandItem, CommandList, CommandShortcut } from "@/components/ui/command";
import { FolderPlus, Grid, List, MoreHorizontal, MoreVertical, Plus, Settings } from "lucide-react";
import { ToggleGroup, ToggleGroupItem } from "@/components/ui/toggle-group";
import { Badge } from "@/components/ui/badge";

export function SideBar(props: {state: AppState}) {
  const tags = useTagsStore();

  return (
    <div className="col-span-4 flex flex-col border">
      <Command className="col-span-4 flex flex-col border">
        <CommandInput placeholder="Type a tag or search ..." />
        <Button className="my-2"><FolderPlus />Add a tag folder</Button>
        <CommandList>
          <CommandEmpty>No tags found.</CommandEmpty>
          <CommandGroup>
            <CommandItem><Badge variant="outline">Auto</Badge>All</CommandItem>
            <CommandItem><Badge variant="outline">Auto</Badge>Recently Visited</CommandItem>
            <CommandItem><Badge variant="outline">Auto</Badge>Unclassified</CommandItem>
          </CommandGroup>
          <CommandGroup heading={
            (
              <div className="flex">
                <span>Seasons</span>
                <div className="ml-auto flex gap-2">
                  <Button size="icon" variant="ghost" className="h-4 w-4"><MoreHorizontal className="h-3 w-3" /></Button>
                  <Button size="icon" variant="ghost" className="h-4 w-4"><Plus className="h-3 w-3" /></Button>
                </div>
              </div>
            )
          }>
            {
              tags.tags.filter((_, idx) => idx < 4).map((item, index) => ((
                <CommandItem key={index} className="p-2">
                  <span>{item.name}</span>
                  <span className="hidden">{item.id}</span>
                  <CommandShortcut>
                    <Button size="icon" variant="ghost" className="h-4 w-4 opacity-0 transition-opacity group-hover/command-item:opacity-100">
                      <MoreVertical className="h-3 w-3" />
                    </Button>
                  </CommandShortcut>
                </CommandItem>
              )))
            }
          </CommandGroup>
        </CommandList>
        <div className="mt-auto flex flex-col">
          <div className="flex flex-row items-center mx-2 pb-2 gap-2">
            <ToggleGroup className="" type="single" variant="outline" value={props.state.display} onValueChange={(e) => {props.state.change_display(e as ("grid" | "list"))}}>
              <ToggleGroupItem value="grid"><Grid /></ToggleGroupItem>
              <ToggleGroupItem value="list"><List /></ToggleGroupItem>
            </ToggleGroup>
            <Button className="p-2 flex-1" variant="outline" onClick={() => props.state.change_type({type: "settings"})}><Settings /> Settings</Button>
          </div>
        </div>
      </Command>
    </div>
  )
}

/* <div className="col-span-3 bg-gray-800">
      <li className={"list-group-item list-group-item-action" + (props.state.now.type === "dashboard" ? " list-group-item-light" : "")} onClick={() => props.state.now.type === "dashboard" || props.state.change_type({type: "dashboard"})}><MdOutlineDashboard /> Dashboard</li>
      <li className={"list-group-item list-group-item-action" + (props.state.now.type === "all-worlds" ? " list-group-item-light" : "")} onClick={() => props.state.now.type === "all-worlds" || props.state.change_type({type: "all-worlds"})}><MdApps /> All<MdOutlineMoreVert className="float-end" style={{height: "100%"}} /></li>
      <li className={"list-group-item list-group-item-action" + (props.state.now.type === "recent-worlds" ? " list-group-item-light" : "")} onClick={() => props.state.now.type === "recent-worlds" || props.state.change_type({type: "recent-worlds"})}><MdHistory /> Recently Visited<MdOutlineMoreVert className="float-end" style={{height: "100%"}} /></li>
      <li className={"list-group-item list-group-item-action" + (props.state.now.type === "unclassified-worlds" ? " list-group-item-light" : "")} onClick={() => props.state.now.type === "unclassified-worlds" || props.state.change_type({type: "unclassified-worlds"})}><MdOutlineInbox /> Unclassified<MdOutlineMoreVert className="float-end" style={{height: "100%"}} /></li>
      {
        tags.tags.map((item, index) => (
          <li key={index} className={"list-group-item list-group-item-action" + ((props.state.now.type === "tag" && props.state.now.tag.id === item.id) ? " list-group-item-light" : "")} onClick={() => (props.state.now.type === "tag" && props.state.now.tag.id === item.id) || props.state.change_type({type: "tag", tag_id: item.id})}>
            <MdLabelOutline /> {item.name}
            <MdOutlineMoreVert className="float-end" onClick={() => {
              props.state.change_type({type: "edit_category", category_id: item.id, form: {name: item.name}});
            }} />
          </li>
        ))
      }
    </div> */