import { MdApps, MdHistory, MdLabelOutline, MdOutlineDashboard, MdOutlineInbox, MdOutlineMoreVert } from "react-icons/md";
import { AppState } from "../viewmodels/app";
import { useTagsStore } from "../viewmodels/tags";

export function SideBar(props: {state: AppState}) {
  const tags = useTagsStore();

  return (
    <div className="col-3 h-100 overflow-auto list-group list-group-flush" style={{borderRight: "1px solid #ccc"}}>
      <li className={"list-group-item list-group-item-action" + (props.state.now.type === "dashboard" ? " list-group-item-light" : "")} onClick={() => props.state.now.type === "dashboard" || props.state.change_type({type: "dashboard"})}><MdOutlineDashboard /> Dashboard</li>
      <li className={"list-group-item list-group-item-action" + (props.state.now.type === "all-worlds" ? " list-group-item-light" : "")} onClick={() => props.state.now.type === "all-worlds" || props.state.change_type({type: "all-worlds"})}><MdApps /> All<MdOutlineMoreVert className="float-end" style={{height: "100%"}} /></li>
      <li className={"list-group-item list-group-item-action" + (props.state.now.type === "recent-worlds" ? " list-group-item-light" : "")} onClick={() => props.state.now.type === "recent-worlds" || props.state.change_type({type: "recent-worlds"})}><MdHistory /> Recently Visited<MdOutlineMoreVert className="float-end" style={{height: "100%"}} /></li>
      <li className={"list-group-item list-group-item-action" + (props.state.now.type === "unclassified-worlds" ? " list-group-item-light" : "")} onClick={() => props.state.now.type === "unclassified-worlds" || props.state.change_type({type: "unclassified-worlds"})}><MdOutlineInbox /> Unclassified<MdOutlineMoreVert className="float-end" style={{height: "100%"}} /></li>
      {
        tags.tags.map((item, index) => (
          <li key={index} className={"list-group-item list-group-item-action" + ((props.state.now.type === "tag" && props.state.now.tag.id === item.id) ? " list-group-item-light" : "")} onClick={() => (props.state.now.type === "tag" && props.state.now.tag.id === item.id) || props.state.change_type({type: "tag", tag_id: item.id})}>
            <MdLabelOutline /> {item.name}
            <MdOutlineMoreVert className="float-end" onClick={() => {
              props.state.change_type({type: "edit_category", category_id: item.id, form: {name: item.name}});
            }} />
          </li>
        ))
      }
    </div>
  )
}
