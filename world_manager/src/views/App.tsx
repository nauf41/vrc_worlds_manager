import { TopBar } from "./TopBar";
import { SideBar } from "./SideBar";
import { Worlds } from "./Worlds";
import { useAppStore } from "../viewmodels/app";
import { CategorySettings } from "./CategorySettings";
import { AddWorldForm } from "./AddWorldForm";
import { Settings } from "./Settings";
import { useTagsStore } from "../viewmodels/tags";

function App() {
  const appState = useAppStore();
  useTagsStore.getState().update();

  return (
    <main className="d-flex flex-column vh-100 overflow-hidden">
      <TopBar state={appState} />
      <div className="container-fluid flex-grow-1 d-flex flex-column min-vh-0 overflow-hidden px-0">
        <div className="row flex-grow-1 min-vh-0 g-0 overflow-hidden">
          <SideBar state={appState} />
          { appState.now.type === 'edit_category' && (
            <CategorySettings />
          )}
          { appState.now.type === 'add_world' && (
            <AddWorldForm />
          )}
          { appState.now.type === 'settings' && (
            <Settings />
          )}
          { appState.now.type === 'dashboard' && (
             <div className="col-9 h-100 overflow-y-auto overflow-x-hidden d-flex flex-column">
              <h1 className="m-3">Dashboard</h1>
            </div>
          )}
          { (appState.now.type === 'all-worlds' || appState.now.type === 'recent-worlds' || appState.now.type === 'unclassified-worlds' || appState.now.type === 'tag') && (
            <Worlds worlds={appState.now.worlds} />
          )}
        </div>
      </div>
    </main>
  );
}

export default App;
