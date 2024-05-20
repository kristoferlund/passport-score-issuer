import "./styles/window.scss";

import App from "./App";
import { InternetIdentityProvider } from "ic-use-internet-identity";
import React from "react";
import ReactDOM from "react-dom/client";
import { VcProvider } from "./vc/VcProvider";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <InternetIdentityProvider>
      <VcProvider>
        <App />
      </VcProvider>
    </InternetIdentityProvider>
  </React.StrictMode>
);
