import React, { useState, useEffect } from "react";
import logo from "../../assets/logo.png";
import { useAuth } from "../Contexts/AuthContext";

const NavBar = () => {
  const { login, logout, isAuthenticated } = useAuth();

  const container =
    "z-40 backdrop-blur-2xl p-4 grid place-items-center grid-flow-col shadow-2xl sticky top-0";
  const navBarText = `text-white cursor-pointer font-bold font-roboto text-[18px] md:text-[32px] flex items-center gap-4 `;
  const logoSection = "grid grid-flow-col auto-cols-max gap-4";
  const underlineStyle = `${"underline"}`;
  return (
    <div className={container}>
      <div className={logoSection}>
        <div className={navBarText}>
          <img
            className="w-10 h-10 bg-blend-screen"
            src={logo}
            alt="logo Img"
          />
          <div onClick={() => {}} className={navBarText + underlineStyle}>
            ICP Voting
          </div>
        </div>
      </div>
      <div className="flex items-center">
        <div onClick={isAuthenticated ? logout : login}>
          <button className="text-white bg-purple-600 hover:bg-purple-700 px-4 py-2 rounded focus:outline-none focus:ring focus:ring-purple-400 focus:ring-opacity-75 shadow-lg hover:shadow-2xl transition duration-300 ease-in-out transform hover:-translate-y-1 border border-white">
            {isAuthenticated ? "Logout" : "Login"}
          </button>
        </div>
      </div>
    </div>
  );
};

export default NavBar;
