import React from "react";
import ReactDOM from "react-dom/client";
import App from "./views/App";
import "./styles.css";
import { init as world_init } from "./viewmodels/world";
import { init as tag_init } from "./viewmodels/tags";
import { init as settings_init } from "./viewmodels/config";
import { init as discord_init } from "./viewmodels/discord";

await world_init();
await tag_init();
await settings_init();
await discord_init();

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
