import { SideBar } from "./SideBar";
import { Worlds } from "./Worlds";
import { useAppStore } from "../viewmodels/app";
import { CategorySettings } from "./CategorySettings";
import { Settings } from "./Settings";
import { useTagsStore } from "../viewmodels/tags";
import "../viewmodels/app";
import "../viewmodels/tags";

function App() {
  const appState = useAppStore();
  useTagsStore.getState().update();

  return (
    <main className="flex flex-col bg-background text-foreground h-screen">
      <div className="grid grid-cols-12 flex-1">
        <SideBar state={appState} />
        <div className="col-span-8">
          { appState.now.type === 'edit_category' && (
            <CategorySettings state={appState} updateState={appState.update_with} />
          )}
          { appState.now.type === 'settings' && (
            <Settings />
          )}
          { appState.now.type === 'dashboard' && (
            <div className="">
              <h1 className="">Dashboard</h1>
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
