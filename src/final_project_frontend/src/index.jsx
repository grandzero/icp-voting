import * as React from "react";
import "../assets/main.css";
import "./index.css";
import ReactDOM from "react-dom/client";
import { render } from "react-dom";
import { final_project_backend } from "../../declarations/final_project_backend";
import MainPage from "./Components/MainPage";
import { AuthProvider } from "./Contexts/AuthContext";
import { Toaster } from "react-hot-toast";
const App = () => {
  return (
    <div>
      <MainPage />
    </div>
  );
};

ReactDOM.createRoot(document.getElementById("root")).render(
  <AuthProvider>
    <Toaster position="top-center" reverseOrder={false} />
    <App />
  </AuthProvider>
);
