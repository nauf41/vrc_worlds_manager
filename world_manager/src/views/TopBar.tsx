import { MdAdd, MdGridView, MdList, MdOutlineNewLabel, MdOutlineSettings } from "react-icons/md";

export function TopBar() {
  return (
    <div className="d-flex gap-2 p-2 flex-shrink-0" style={{borderBottom: "1px solid #ccc"}}>
      <button className="btn btn-dark"><MdOutlineNewLabel /> Add new category</button>
      <button className="btn btn-dark"><MdAdd /> Add new world</button>
      <button className="btn btn-dark"><MdOutlineSettings /> Settings</button>

      <div className="ms-auto">
        <input type="radio" className="btn-check" name="view-type" id="grid-view" autoComplete="off"></input>
        <label className="btn" htmlFor="grid-view"><MdGridView /></label>
        <input type="radio" className="btn-check" name="view-type" id="list-view" autoComplete="off"></input>
        <label className="btn" htmlFor="list-view"><MdList /></label>
      </div>
    </div>
  )
}
