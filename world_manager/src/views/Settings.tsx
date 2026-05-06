import { Button } from "@/components/ui/button"
import { Checkbox } from "@/components/ui/checkbox"
import { Field, FieldDescription, FieldGroup, FieldLabel, FieldLegend, FieldSet } from "@/components/ui/field"
import { Input } from "@/components/ui/input"
import { useState } from "react"

export function Settings() {
  return (
    <div className="w-full max-w-md">
      <form>
        <FieldGroup>
          <FieldSet>
            <FieldLegend>Settings</FieldLegend>
            <FieldGroup>
              <Field orientation="horizontal">
                <Checkbox
                  id="settings:useDiscordIntegration"
                />
                <FieldLabel htmlFor="settings:useDiscordIntegration">
                  Use Discord Integration
                </FieldLabel>
              </Field>
              <Field>
                <FieldLabel htmlFor="settings:discordToken">
                  Token
                </FieldLabel>
                <Input
                  id="settings:discordToken"
                  type="password"
                />
                </Field>
            </FieldGroup>
          </FieldSet>
          <Field orientation="horizontal">
            <Button type="submit">Save</Button>
            <Button variant="outline" type="button">
              Cancel
            </Button>
          </Field>
        </FieldGroup>
      </form>
    </div>
  )
}


{/* <div className="col-9 h-100 overflow-y-auto overflow-x-hidden d-flex flex-column">
      <div className="m-3">
        <input type="checkbox" className="form-check-input" id="useDiscordLink" checked={whetherUseDiscordLink} onChange={(e) => {setWhetherUseDiscordLink(e.target.checked)}} />&nbsp;
        <label htmlFor="useDiscordLink" className="form-check-label">Use Discord Integration</label>
      </div>
      <div className="m-3 ms-5">
        <label htmlFor="folderName" className="form-label">Token</label>
        <input type="password" className="form-control" id="folderName" defaultValue="unclassified" />
        <span className="form-text">The value entered in this field will not be accessible after saving.</span>
      </div>
      <div className="m-3">
        <button className="btn btn-link">Open app folder in File Explorer</button>
      </div>

      <div className="align-self-end me-3 mt-auto mb-3">
        <button className="btn btn-primary mx-1">Save</button>
        <button className="btn btn-secondary mx-1">Discard</button>
        <button className="btn btn-primary mx-1">Apply</button>
      </div>
    </div> */}