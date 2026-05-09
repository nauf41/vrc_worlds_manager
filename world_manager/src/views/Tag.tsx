import { Tag, TagGroup } from "@/types/tags";
import { Button } from "@/components/ui/button";
import { Field, FieldGroup, FieldLabel, FieldLegend, FieldSet, FieldTitle } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { TagState } from "@/viewmodels/tags";
import { AppState } from "@/viewmodels/app";
import { ScrollArea } from "@/components/ui/scroll-area";
import { importChannels, useDiscordStore } from "@/viewmodels/discord";
import { Checkbox } from "@/components/ui/checkbox";
import { useState } from "react";

export function TagCreate(props: {state: TagState, taggroup?: TagGroup}) {
  return (
    <div>
      <form onSubmit={async (e) => {e.preventDefault(); await props.state.addTag((document.getElementById("add-a-tag:name") as HTMLInputElement).value, props.taggroup?.id);}}>
        <FieldGroup>
          <FieldSet>
            <FieldLegend><span className="text-gray-400">Adding a</span> tag{props.taggroup && (<><span className="text-gray-400"> under tag folder </span>{props.taggroup.name}</>)}</FieldLegend>
            <FieldGroup>
              <Field>
                <FieldLabel htmlFor="add-a-tag:name">
                  Name
                </FieldLabel>
                <Input
                  id="add-a-tag:name"
                  required
                />
              </Field>
            </FieldGroup>
          </FieldSet>
          <Field orientation="horizontal">
            <Button type="submit">Create</Button>
            <Button variant="outline" type="button">Cancel</Button>
          </Field>
        </FieldGroup>
      </form>
    </div>
  )
}

export function TagEdit(props: {appState: AppState, tagState: TagState, tag: Tag}) {
  return (
    <div>
      <form onSubmit={async (e) => {console.log("submitting form"); await props.tagState.changeTagName(props.tag.id, (document.getElementById("rename-a-tag:name") as HTMLInputElement).value); e.preventDefault();}}>
        <FieldGroup>
          <FieldSet>
            <FieldLegend><span className="text-gray-400">Editing a </span>tag <span className="text-gray-400">named</span> {props.tag.name}</FieldLegend>
            <FieldGroup>
              <Field>
                <FieldLabel htmlFor="rename-a-tag:name">
                  Name
                </FieldLabel>
                <Input
                  id="rename-a-tag:name"
                  defaultValue={props.tag.name}
                  required
                />
              </Field>
            </FieldGroup>
          </FieldSet>
          <Field orientation="horizontal">
            <Button type="submit">Save</Button>
            <Button variant="outline" type="button">Cancel</Button>
            <div className="ml-auto">
              <Button variant="destructive" type="button" onClick={async () => {if (window.confirm(`Are you sure want to delete tag "${props.tag.name}"?`)) { await props.tagState.removeTag(props.tag.id); props.appState.change_type({type: "all"}); }}}>Delete this tag</Button>
            </div>
          </Field>
        </FieldGroup>
      </form>
    </div>
  )
}

export function ImportTags() {
  const discordState = useDiscordStore();
  const [selectedChannels, setSelectedChannels] = useState<Set<string>>(new Set<string>());

  return (
    <div>
      <form onSubmit={async (e) => {
        e.preventDefault();
        console.log("Selected channels: ", selectedChannels);
        const channels = [];
        for (const channel of selectedChannels) {
          channels.push(discordState.channels.find(c => c.id === channel)!);
        }
        await importChannels(channels);
      }}>
        <FieldGroup>
          <FieldSet>
            <ScrollArea className="flex-grow min-h-0">
              <FieldLegend><span className="text-gray-400">Import Tags</span></FieldLegend>
              {discordState.guilds.map(guild => (
                <FieldGroup key={guild[0].id}>
                  <FieldTitle>{guild[0].name}</FieldTitle>
                  <Field>
                    {guild[1].rootChannels.map(channel => (
                      <Field key={channel.id} className="m-0 p-0" orientation="horizontal">
                        <Checkbox
                          id={`import-${channel.id}`}
                          checked={selectedChannels.has(channel.id)}
                          onCheckedChange={(checked) => {
                            if (checked) { setSelectedChannels(prev => new Set(prev).add(channel.id)); }
                            else { setSelectedChannels(prev => {
                              const newSet = new Set(prev);
                              newSet.delete(channel.id);
                              return newSet;
                            }); }
                          }}
                        />
                        <FieldLabel>{channel.name}</FieldLabel>
                      </Field>
                    ))}
                  </Field>
                  { guild[1].categories.map(category => (
                    <Field key={category[0].id} orientation="vertical">
                      <FieldTitle>{category[0].name}</FieldTitle>
                      { category[1].map((chan) => (
                        <Field key={chan.id} orientation="horizontal">
                          <Checkbox
                            id={`import-${chan.id}`}
                            checked={selectedChannels.has(chan.id)}
                            onCheckedChange={(checked) => {
                              if (checked) { setSelectedChannels(prev => new Set(prev).add(chan.id)); }
                              else { setSelectedChannels(prev => {
                                const newSet = new Set(prev);
                                newSet.delete(chan.id);
                                return newSet;
                              }); }
                            }}
                          />
                          <FieldLabel>{chan.name}</FieldLabel>
                        </Field>
                      ))}
                    </Field>
                  ))

                  }
                </FieldGroup>
              ))}
            </ScrollArea>
          </FieldSet>
          <Field orientation="horizontal">
            <Button type="submit">Import</Button>
            <Button variant="outline" type="button">Cancel</Button>
          </Field>
        </FieldGroup>
      </form>
    </div>
  )
}