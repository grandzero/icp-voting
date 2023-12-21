import React, { createContext, useState, useContext, useEffect } from "react";
import { AuthClient } from "@dfinity/auth-client";
import { toast } from "react-hot-toast";
let authClient = await AuthClient.create();
const AuthContext = createContext(null);

export const useAuth = () => useContext(AuthContext);

export const AuthProvider = ({ children }) => {
  const [isAuthenticated, setIsAuthenticated] = useState(false);
  const [identity, setIdentity] = useState(null);

  useEffect(() => {
    (async () => {
      const identity = await authClient.getIdentity();
      if (identity?.constructor?.name === "DelegationIdentity") {
        toast.success("Successfully authenticated with Internet Identity", {
          duration: 2000,
          position: "bottom-right",
        });
        setIsAuthenticated(true);
        setIdentity(identity);
      }
    })();
  }, []);

  async function login() {
    // console.log("AuthClient : ", authClient);
    authClient = await authClient.login({
      onSuccess: () => {
        toast.success("Successfully authenticated with Internet Identity", {
          duration: 2000,
          position: "bottom-right",
        });
        setIsAuthenticated(true);
      },
      onError: (err) => {
        console.log(err);
        toast.error("Could not authenticate!", {
          duration: 2000,
          position: "bottom-right",
        });
        setIsAuthenticated(false);
      },
    });
  }

  async function logout() {
    await authClient.logout();

    setIsAuthenticated(false);
    // The user is now logged out.
  }

  return (
    <AuthContext.Provider value={{ isAuthenticated, login, logout, identity }}>
      {children}
    </AuthContext.Provider>
  );
};
