import { Tag, TagGroup } from "@/types/tags";
import { Button } from "@/components/ui/button";
import { Field, FieldGroup, FieldLabel, FieldLegend, FieldSet } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { TagState } from "@/viewmodels/tags";
import { AppState } from "@/viewmodels/app";

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