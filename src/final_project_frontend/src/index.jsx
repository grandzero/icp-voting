import * as React from "react";
import '../assets/main.css';
import "./index.css"
import ReactDOM from 'react-dom/client';
import { render } from "react-dom";
import { final_project_backend } from "../../declarations/final_project_backend";
import MainPage from "./Components/MainPage";

const App = () => {
  return (
    <div >
      <MainPage />
    </div>
  );
};

ReactDOM.createRoot(document.getElementById('root')).render(<App />);
