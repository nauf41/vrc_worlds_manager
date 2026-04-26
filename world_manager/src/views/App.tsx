import { TopBar } from "./TopBar";
import { SideBar } from "./SideBar";
import { Worlds } from "./Worlds";
import { CategorySettings } from "./CategorySettings";
import { Settings } from "./Settings";

function App() {
  return (
    <main className="d-flex flex-column vh-100 overflow-hidden">
      <TopBar />
      <div className="container-fluid flex-grow-1 d-flex flex-column min-vh-0 overflow-hidden px-0">
        <div className="row flex-grow-1 min-vh-0 g-0 overflow-hidden">
          <SideBar />
          <Worlds />
        </div>
      </div>
    </main>
  );
}

export default App;
