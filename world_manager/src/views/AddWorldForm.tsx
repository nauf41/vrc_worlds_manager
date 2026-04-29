export function AddWorldForm() {
  return (
    <div className="col-9 h-100 overflow-y-auto overflow-x-hidden d-flex flex-column">
      <div className="m-3">
        <label htmlFor="worldUrl" className="form-label">World URL <span color="red">*</span></label>
        <input type="text" className="form-control" id="worldUrl" defaultValue="" />
      </div>
      <div className="m-3">
        <label htmlFor="worldName" className="form-label">World Name</label>
        <input type="text" className="form-control" id="worldName" defaultValue="" />
      </div>
      <div className="m-3">
        <label htmlFor="worldDescription" className="form-label">World Description</label>
        <input type="text" className="form-control" id="worldDescription" defaultValue="" />
      </div>
      <div className="m-3">
        <label htmlFor="worldCapacity" className="form-label">World Capacity</label>
        <input type="number" className="form-control" id="worldCapacity" defaultValue="" />
      </div>

      <div className="align-self-end me-3 mt-auto mb-3">
        <button className="btn btn-danger mx-1">Remove this category</button>
        <button className="btn btn-primary mx-1">Save</button>
        <button className="btn btn-secondary mx-1">Discard</button>
        <button className="btn btn-primary mx-1">Apply</button>
      </div>
    </div>
  )
}
