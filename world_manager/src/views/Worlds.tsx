import { MdFavoriteBorder, MdGroups, MdOutlineMeetingRoom } from "react-icons/md"
import { World as TWorld } from "../types/world";

export function Worlds(props: {worlds: TWorld[]}) {
  return (
    <div className="col-9 h-100 overflow-y-auto overflow-x-hidden">
      <div className="row g-3 m-0 p-3">
        {props.worlds.map((world) => (
          <World key={world.id} world={world} />
        ))}
      </div>
    </div>
  )
}

function World(props: {world: TWorld}) {
  return (
    <div className="col-4 col-xl-3 col-xxl-2">
      <div className="card h-100">
        <img src="/testimage.png" className="card-img-top" alt={props.world.title} />
        <div className="card-body">
          <h5 className="card-title">{props.world.title}</h5>
          <span className="card-text"><span className="text-body-tertiary">By</span> {props.world.publisher_name}</span><br />
          <span className="mx-1" style={{"whiteSpace": "nowrap"}}><MdFavoriteBorder /> {props.world.favorites}</span>
          <span className="mx-1" style={{"whiteSpace": "nowrap"}}><MdGroups /> {props.world.capacity}</span>
          <span className="mx-1" style={{"whiteSpace": "nowrap"}}><MdOutlineMeetingRoom /> {props.world.self_visits}</span>
        </div>
      </div>
    </div>
  )
}
