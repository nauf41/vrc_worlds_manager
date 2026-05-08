import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import { Dialog, DialogClose, DialogContent, DialogDescription, DialogFooter, DialogHeader, DialogTitle } from "@/components/ui/dialog";
import { Field } from "@/components/ui/field";
import { Label } from "@/components/ui/label";
import { ScrollArea } from "@/components/ui/scroll-area";
import { Separator } from "@/components/ui/separator";
import { attach_world, detach_world } from "@/models/db";
import { World } from "@/types/world";
import { AppState } from "@/viewmodels/app";
import { TagState } from "@/viewmodels/tags";
import { useWorldStore } from "@/viewmodels/world";
import { Fragment, useState } from "react";

export function WorldDialog(props: {world: World, tags: TagState, app: AppState}) {
  const mp: Map<number, [boolean, boolean]> = new Map();
  props.tags.tags.forEach((tag) => { mp.set(tag[0].id, [tag[1].includes(props.world.id), tag[1].includes(props.world.id)]) });
  const [checkedStatus, setCheckedStatus] = useState<Map<number, [boolean, boolean]>>(mp); // [tag, [now, default]]

  return (
    <Dialog open={true} onOpenChange={(e) => {!e && props.app.change_dialog({type: "none"})}}>
      <DialogContent className="h-full max-h-[90vh] flex flex-col p-0">
        <form className="flex flex-col flex-1 min-h-0 p-6" onSubmit={async (e) => {
          e.preventDefault();
          const tasks = [];
          for await (const [tagid, [nowStatus, defaultStatus]] of checkedStatus.entries()) {
            if (nowStatus !== defaultStatus) {
              if (nowStatus) {
                console.log(`Attaching tag ${tagid}`);
                tasks.push(attach_world(tagid, props.world.id));
              } else {
                console.log(`Detaching tag ${tagid}`);
                tasks.push(detach_world(tagid, props.world.id));
              }
            }
          }

          await Promise.all(tasks);
          await Promise.all([props.tags.update(), useWorldStore.getState().updateWorld(props.app.now)]);

          props.app.change_dialog({type: "none"});
        }}>
          <DialogHeader>
            <DialogTitle><span className="text-gray-400">Tags for</span> {props.world.title}</DialogTitle>
            <DialogDescription>You can assign or remove tags from the world here.</DialogDescription>
            <Separator className="my-1" />
          </DialogHeader>
            <ScrollArea className="flex-1 pr-4 min-h-0">
              { props.tags.tags_without_taggroups.map((tag, index) => (
                <Field orientation="horizontal" className="py-1" key={index}>
                  <Checkbox checked={checkedStatus.get(tag.id)?.[0]} onCheckedChange={(checked) => {
                    setCheckedStatus((prev) => {
                      const res = new Map(prev);
                      res.set(tag.id, [!!checked, prev.get(tag.id)?.[1] ?? false]);
                      return res;
                    })
                  }} />
                  <Label>{tag.name}</Label>
                </Field>
              ))}
              <Separator className="my-1" />
              { props.tags.taggroups.filter(t => t[0].id !== 0).map((taggroup, index) => (
                <Fragment key={index}>
                  <span>{taggroup[0].name}</span>
                  { taggroup[1].map(id => props.tags.tags.find(t => t[0].id === id)).filter(v => v !== undefined).map((tag) => (
                    <Field orientation="horizontal" className="py-1 ms-3" key={tag[0].id}>
                      <Checkbox key={tag[0].id} checked={checkedStatus.get(tag[0].id)?.[0]} onCheckedChange={(checked) => {
                        setCheckedStatus((prev) => {
                          const res = new Map(prev);
                          res.set(tag[0].id, [!!checked, prev.get(tag[0].id)?.[1] ?? false]);
                          return res;
                        })
                      }} />
                      <Label>{tag[0].name}</Label>
                    </Field>
                  ))}
                  <Separator className="my-1" />
                </Fragment>
              ))}
            </ScrollArea>
          <DialogFooter className="p-2">
            <DialogClose asChild>
              <Button variant="outline">Cancel</Button>
            </DialogClose>
            <Button type="submit">Save changes</Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  )
}