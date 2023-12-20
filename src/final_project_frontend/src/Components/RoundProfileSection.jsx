import React, { useState, useEffect } from "react";
import UserInput from "./Utilities/UserInput";
import ShadowButton from "./Utilities/ShadowButton";
import styled from "styled-components";

const Input = styled.input``;

const Description = styled.p`
  font-size: 0.6em;
  font-weight: bold;
  text-align: center;
  color: rgba(0, 0, 0, 0.5);
`;
const RoundProfileSection = ({
  proposalList,
  text,
  currentProposal,
  proposalCount,
}) => {
  const [loading, setLoading] = useState(false);
  const [proposal, setProposal] = useState("");
  const [isPrivileged, setIsPrivileged] = useState(false);
  const StyledCheckbox = ({ checked, onChange }) => {
    const checkboxStyle =
      "form-checkbox h-5 w-5 text-purple-600 rounded focus:ring-transparent border-white shadow-lg";

    return (
      <label className="flex items-center space-x-3 mt-3 ml-2">
        <input
          type="checkbox"
          checked={checked}
          onChange={onChange}
          className={checkboxStyle}
        />
        <span className="text-white text-[18px]">Privileged</span>
      </label>
    );
  };

  const handlePrivilegeChange = (e) => {
    setIsPrivileged(e.target.checked);
  };

  const handleChange = (e) => {
    setProposal(e.target.value);
  };
  const handleProposeSend = async () => {};

  // Styles
  const textStyle =
    "text-white text-[18px] p-4 grid place-items-center gap-y-4 mt-4";
  const inputStyle =
    "grid lg:grid-flow-col md:auto-cols-1fr md:gap-x-[15em] place-items-center  items-center ";
  const container = `mt-4 ${currentProposal ? "" : "h-screen"} ${
    proposalList.length > 1 ? "" : "h-[60vh]"
  } md:mt-10 relative w-full `;
  return (
    <div className={container}>
      <div className="my-24 grid place-items-center mb-[12%]">
        <div className={inputStyle}>
          <UserInput value={proposal} onChange={handleChange}>
            <Input
              onChange={handleChange}
              value={proposal}
              className="rounded-[60px] p-3 "
              placeholder="Enter your proposal here..."
              name="text"
              type="text"
            />

            {isPrivileged && (
              <Input
                onChange={handleChange}
                value={proposal}
                className="rounded-[60px] p-3 "
                placeholder="Enter nft canister principal here..."
                name="text"
                type="text"
              />
            )}
            <StyledCheckbox
              checked={isPrivileged}
              onChange={handlePrivilegeChange}
            />
            <ShadowButton
              loading={loading}
              onClick={() => handleProposeSend()}
            />
          </UserInput>
        </div>
        <p className={textStyle}>{text}</p>
      </div>
    </div>
  );
};

export default RoundProfileSection;
