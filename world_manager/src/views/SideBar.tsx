import { MdApps, MdLabelOutline, MdOutlineDashboard, MdOutlineInbox, MdOutlineMoreVert, MdOutlineSchedule } from "react-icons/md";

export function SideBar() {
  return (
    <div className="col-3 h-100 overflow-auto list-group list-group-flush" style={{borderRight: "1px solid #ccc"}}>
      <li className="list-group-item list-group-item-action"><MdOutlineDashboard /> Dashboard</li>
      <li className="list-group-item list-group-item-action"><MdApps /> All<MdOutlineMoreVert className="float-end" style={{height: "100%"}} /></li>
      <li className="list-group-item list-group-item-action list-group-item-light"><MdOutlineInbox /> Unclassified<MdOutlineMoreVert className="float-end" style={{height: "100%"}} /></li>
      {
        ["Folder-1", "Folder-2", "Folder-3"].map((item, index) => (
          <li key={index} className="list-group-item list-group-item-action"><MdLabelOutline /> {item}<MdOutlineMoreVert className="float-end" style={{height: "100%"}} /></li>
        ))
      }
      <li className="list-group-item list-group-item-action"><MdOutlineSchedule /> Pending Updates</li>
    </div>
  )
}