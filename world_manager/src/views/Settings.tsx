import { Button } from "@/components/ui/button"
import { Checkbox } from "@/components/ui/checkbox"
import { Field, FieldGroup, FieldLabel, FieldLegend, FieldSet } from "@/components/ui/field"
import { Input } from "@/components/ui/input"

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
