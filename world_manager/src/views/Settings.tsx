import { useState } from "react"

export function Settings() {
  const [whetherUseDiscordLink, setWhetherUseDiscordLink] = useState<boolean>(false);

  return (
    <div className="col-9 h-100 overflow-y-auto overflow-x-hidden d-flex flex-column">
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
    </div>
  )
}
