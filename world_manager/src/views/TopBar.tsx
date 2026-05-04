import { MdAdd, MdGridView, MdList, MdOutlineNewLabel, MdOutlineSettings } from "react-icons/md";
import { AppState } from "../viewmodels/app";
import { createTag } from "../models/db";

export function TopBar(props: {state: AppState}) {
  const appState = props.state;

  return (
    <div className="d-flex gap-2 p-2 flex-shrink-0" style={{borderBottom: "1px solid #ccc"}}>
      <button className="btn btn-dark" onClick={() => {createTag("Unnamed Category").then(r => appState.change_type({type: "edit_category", form: {name: "Unnamed Category"}, category_id: r.id}))}}><MdOutlineNewLabel /> Add new category</button>
      <button className="btn btn-dark" onClick={() => appState.change_type({type: "settings"})}><MdOutlineSettings /> Settings</button>

      <div className="ms-auto">
        <input type="radio" className="btn-check" name="view-type" id="grid-view" autoComplete="off" checked={appState.display === "grid"} onChange={(e) => {if (e.target.checked && appState.display !== "grid") appState.change_display("grid")}}></input>
        <label className="btn" htmlFor="grid-view"><MdGridView /></label>
        <input type="radio" className="btn-check" name="view-type" id="list-view" autoComplete="off" checked={appState.display === "list"} onChange={(e) => {if (e.target.checked && appState.display !== "list") appState.change_display("list")}}></input>
        <label className="btn" htmlFor="list-view"><MdList /></label>
      </div>
    </div>
  )
}
