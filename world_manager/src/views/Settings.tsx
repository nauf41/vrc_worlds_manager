import { Button } from "@/components/ui/button"
import { Checkbox } from "@/components/ui/checkbox"
import { Field, FieldGroup, FieldLabel, FieldLegend, FieldSet } from "@/components/ui/field"
import { Input } from "@/components/ui/input"
import { get_config, update_config } from "@/models/db"
import { useConfigStore } from "@/viewmodels/config"
import { useState } from "react"

export function Settings() {
  const conf = useConfigStore();
  const [discordIntegration, setDiscordIntegration] = useState(conf.is_discord_bot_token_some ?? false);

  return (
    <div className="w-full max-w-md">
      <form onSubmit={async (e) => {
        e.preventDefault();

        console.info(discordIntegration, document.getElementById("settings:discordToken"));

        await update_config({
          discord_bot_token: discordIntegration ? (document.getElementById("settings:discordToken") as HTMLInputElement)?.value ?? null : null,
        });

        window.alert("You need to restart the application for the changes to take effect.");
      }}>
        <FieldGroup>
          <FieldSet>
            <FieldLegend>Settings</FieldLegend>
            <FieldGroup>
              <Field orientation="horizontal">
                <Checkbox
                  id="settings:useDiscordIntegration"
                  checked={discordIntegration}
                  onCheckedChange={(checked) => setDiscordIntegration(!!checked)}
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
