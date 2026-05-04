import { useState } from "react"
import { MdCheck } from "react-icons/md";
import { AppState, NowSelected } from "../viewmodels/app";
import { useTagsStore } from "../viewmodels/tags";

export function CategorySettings(props: {state: AppState, updateState: (state: NowSelected) => void}) {
  const [whetherUseDiscordLink, setWhetherUseDiscordLink] = useState<boolean>(false);

  return (
    <div className="col-9 h-100 overflow-y-auto overflow-x-hidden d-flex flex-column">
      <div className="m-3">
        <label htmlFor="folderName" className="form-label">Folder Name</label>
        <input type="text" className="form-control" id="folderName" value={props.state.now.type === "edit_category" ? props.state.now.form.name : ""} onChange={(e) => {
          if (props.state.now.type === "edit_category") {
            props.updateState({...props.state.now, form: {...props.state.now.form, name: e.target.value}});
          }
        }} />
        <span className="form-text">This field can't be left empty.</span>
      </div>
      <div className="m-3">
          <input type="checkbox" className="form-check-input" id="useDiscordLink" checked={whetherUseDiscordLink} onChange={(e) => {setWhetherUseDiscordLink(e.target.checked)}} />&nbsp;
          <label htmlFor="useDiscordLink" className="form-check-label">Use Discord Integration</label>
      </div>
      { whetherUseDiscordLink &&
        <div className="m-3 ms-5">
          <label htmlFor="folderName" className="form-label">Discord Channel Id</label>
          <div className="container-fluid p-0 d-flex gap-2">
            <input type="text row-8" className="form-control" id="discordTarget" />
            <MdCheck className="row-1 align-self-center" size="1.3em" />
            <button className="row-3 btn btn-primary">Check</button>
          </div>
          <span className="form-text">This field can't be left empty.</span>
        </div>
      }
      <div className="align-self-end me-3 mt-auto mb-3">
        <button className="btn btn-danger mx-1" onClick={() => props.state.now.type === "edit_category" && useTagsStore.getState().remove(props.state.now.category_id).then(() => props.state.change_type({type: "dashboard"}))}>Remove this category</button>
        <button className="btn btn-primary mx-1" onClick={() => props.state.now.type === "edit_category" && useTagsStore.getState().change(props.state.now.category_id, {id: props.state.now.category_id, name: props.state.now.form.name}).then(() => props.state.change_type({type: "dashboard"}))}>Save</button>
        <button className="btn btn-secondary mx-1" onClick={() => props.state.change_type({type: "dashboard"})}>Discard</button>
      </div>
    </div>
  )
}
