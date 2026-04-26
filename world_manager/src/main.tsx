import React from "react";
import ReactDOM from "react-dom/client";
import App from "./views/App";
import "bootstrap";
import "bootstrap/dist/css/bootstrap.min.css";
import "./styles.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <App />
  </React.StrictMode>,
);
