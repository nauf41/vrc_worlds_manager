import { Button } from "@/components/ui/button";
import { Field, FieldGroup, FieldLabel, FieldLegend, FieldSet } from "@/components/ui/field";
import { Input } from "@/components/ui/input";
import { TagGroup } from "@/types/tags";
import { AppState } from "@/viewmodels/app";
import { TagState } from "@/viewmodels/tags";

export function TagGroupCreate(props: {state: TagState}) {
  return (
    <div>
      <form onSubmit={(e) => {e.preventDefault(); props.state.addTagGroup((document.getElementById("add-a-tag-group:name") as HTMLInputElement).value);}}>
        <FieldGroup>
          <FieldSet>
            <FieldLegend><span className="text-gray-400">Adding a</span> tag folder</FieldLegend>
            <FieldGroup>
              <Field>
                <FieldLabel htmlFor="add-a-tag-group:name">
                  Name
                </FieldLabel>
                <Input
                  id="add-a-tag-group:name"
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

export function TagGroupEdit(props: {appState: AppState, tagState: TagState, taggroup: TagGroup}) {
  return (
    <div>
      <form onSubmit={(e) => {e.preventDefault(); props.tagState.changeTagGroupName(props.taggroup.id, (document.getElementById("rename-a-tag-group:name") as HTMLInputElement).value);}}>
        <FieldGroup>
          <FieldSet>
            <FieldLegend><span className="text-gray-400">Editing a</span> tag folder <span className="text-gray-400">named</span> {props.taggroup.name}</FieldLegend>
            <FieldGroup>
              <Field>
                <FieldLabel htmlFor="rename-a-tag-group:name">
                  Name
                </FieldLabel>
                <Input
                  id="rename-a-tag-group:name"
                  defaultValue={props.taggroup.name}
                  required
                />
              </Field>
            </FieldGroup>
          </FieldSet>
          <Field orientation="horizontal">
            <Button type="submit">Save</Button>
            <Button variant="outline" type="button">Cancel</Button>
            <div className="ml-auto">
              <Button variant="destructive" type="button" onClick={async () => {if (window.confirm(`Are you sure want to delete tag group "${props.taggroup.name}"?`)) { await props.tagState.removeTagGroup(props.taggroup.id); props.appState.change_type({type: "all"}); }}}>Delete this tag group</Button>
            </div>
          </Field>
        </FieldGroup>
      </form>
    </div>
  )
}
