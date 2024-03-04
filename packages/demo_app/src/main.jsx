import App from "./App";
import { InternetIdentityProvider } from "ic-use-internet-identity";
import { PassportProvider } from "./passport/PassportProvider";
import React from "react";
import ReactDOM from "react-dom/client";

ReactDOM.createRoot(document.getElementById("root")).render(
  <React.StrictMode>
    <InternetIdentityProvider>
      <PassportProvider>
        <App />
      </PassportProvider>
    </InternetIdentityProvider>
  </React.StrictMode>
);
