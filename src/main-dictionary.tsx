import React from "react";
import ReactDOM from "react-dom/client";
import DictionaryWindowApp from "./features/dictionary-window/DictionaryWindowApp";
import "./index.css";

ReactDOM.createRoot(document.getElementById("root") as HTMLElement).render(
  <React.StrictMode>
    <DictionaryWindowApp />
  </React.StrictMode>,
);
