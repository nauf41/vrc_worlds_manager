import { MdFavoriteBorder, MdGroups, MdOutlineLabel, MdOutlineMeetingRoom } from "react-icons/md"

export function Worlds() {
  return (
    <div className="col-9 h-100 overflow-y-auto overflow-x-hidden">
      <div className="row g-3 m-0 p-3">
      <World />
      <World />
      <World />
      <World />
      <World />
      <World />
      <World />
      <World />
      <World />
      </div>
    </div>
  )
}

function World() {
  return (
    <div className="col-4">
      <div className="card h-100">
      <img src="/public/testimage.png" className="card-img-top" alt="" />
      <div className="card-body">
        <h5 className="card-title">集合はいつもの場所で</h5>
        <span className="card-text"><span className="text-body-tertiary">By</span> John Doe</span><br />
        <span className="mx-1" style={{"whiteSpace": "nowrap"}}><MdFavoriteBorder /> 1.4k</span>
        <span className="mx-1" style={{"whiteSpace": "nowrap"}}><MdGroups /> 10</span>
        <span className="mx-1" style={{"whiteSpace": "nowrap"}}><MdOutlineMeetingRoom /> 5</span>
        <span className="mx-1" style={{"whiteSpace": "nowrap"}}><MdOutlineLabel /> Adventure <span className="text-body-tertiary">+2</span></span>
      </div>
      </div>
    </div>
  )
}