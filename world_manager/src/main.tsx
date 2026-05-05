import React from "react";
import ReactDOM from "react-dom/client";
import App from "./views/App";
import "./styles.css";
import { init as world_init } from "./viewmodels/world";
import { init as tag_init } from "./viewmodels/tags";

world_init();
tag_init();

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
