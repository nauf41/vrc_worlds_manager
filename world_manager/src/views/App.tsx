import { SideBar } from "./SideBar";
import { Worlds } from "./Worlds";
import { useAppStore } from "../viewmodels/app";
import { Settings } from "./Settings";
import { TagCreate, TagEdit } from "./Tag";
import { TagGroupCreate, TagGroupEdit } from "./TagGroup";
import { useTagStore } from "@/viewmodels/tags";

function App() {
  const appState = useAppStore();
  const tagState = useTagStore();

  return (
    <main className="flex flex-col bg-background text-foreground h-screen">
      <div className="grid grid-cols-12 flex-1">
        <SideBar state={appState} />
        <div className="col-span-8 p-3">
          { appState.now.type === "settings" && (
            <Settings />
          )}
          { appState.now.type === "create-tag" && (
            <TagCreate state={tagState} taggroup={appState.now.under} />
          )}
          { appState.now.type === "edit-tag" && (
            <TagEdit appState={appState} tagState={tagState} tag={appState.now.tag} />
          )}
          { appState.now.type === "create-tag-group" && (
            <TagGroupCreate state={tagState} />
          )}
          { appState.now.type === "edit-tag-group" && (
            <TagGroupEdit appState={appState} tagState={tagState} taggroup={appState.now.taggroup} />
          ) }
          { (appState.now.type === "non-tagged" || appState.now.type === "all-favorited" || appState.now.type === "tagged" || appState.now.type === "all") && (
            <Worlds />
          )}
        </div>
      </div>
    </main>
  );
}

export default App;
