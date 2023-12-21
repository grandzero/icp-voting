import React, { useState } from "react";

const styles = {
  marginTop: "20px",
  padding: "20px 50px",
  border: "none",
  fontSize: "17px",
  color: "#fff",
  borderRadius: "20px",
  letterSpacing: "2px",
  fontWeight: "700",
  textTransform: "uppercase",
  transition: "0.5s",
  transitionProperty: "box-shadow",
  background: "rgb(35, 35, 35)", // Changed to a darker shade for contrast
  boxShadow: "0 0 25px rgb(35, 35, 35)", // Adjusted shadow color
};

const hoverStyles = {
  boxShadow:
    "0 0 5px rgb(35, 35, 35), 0 0 25px rgb(35, 35, 35), 0 0 50px rgb(35, 35, 35), 0 0 100px rgb(35, 35, 35)",
};

const ShadowButton = ({ onClick, loading, isAuthenticated }) => {
  const [isHovered, setIsHovered] = useState(false);

  const handleMouseEnter = () => {
    setIsHovered(true);
  };

  const handleMouseLeave = () => {
    setIsHovered(false);
  };

  return (
    <button
      onClick={onClick}
      style={isHovered ? { ...styles, ...hoverStyles } : styles}
      onMouseEnter={() => handleMouseEnter()}
      onMouseLeave={() => handleMouseLeave()}
      disabled={!isAuthenticated}
    >
      {loading ? "Sending Proposal..." : "Send Proposal"}
    </button>
  );
};

export default ShadowButton;
