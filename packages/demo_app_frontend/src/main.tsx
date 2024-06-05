import "./styles.scss";

import App from "./App";
import DemoAppBackendProvider from "./demo_app_backend/DemoAppBackendProvider";
import { InternetIdentityProvider } from "ic-use-internet-identity";
import React from "react";
import ReactDOM from "react-dom/client";
import { VcProvider } from "./vc/VcProvider";

ReactDOM.createRoot(document.getElementById("root")!).render(
  <React.StrictMode>
    <InternetIdentityProvider>
      <DemoAppBackendProvider>
        <VcProvider>
          <App />
        </VcProvider>
      </DemoAppBackendProvider>
    </InternetIdentityProvider>
  </React.StrictMode>
);
