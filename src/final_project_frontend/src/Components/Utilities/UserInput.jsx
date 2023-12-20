import React from "react";
import styled from "styled-components";
const InputContainer = styled.div`
  display: flex;
  flex-direction: column;
  gap: 6px;
  padding: 30px;
  border-radius: 20px;
  position: relative;

  ::before {
    content: "";
    position: absolute;
    width: 100%;
    height: 100%;
    top: 0;
    left: 0;
    z-index: -1;
    filter: blur(25px);
    border-radius: 20px;
    background-color: #e499ff;
    background-image: radial-gradient(
        at 47% 69%,
        hsla(17, 62%, 65%, 1) 0px,
        transparent 50%
      ),
      radial-gradient(at 9% 32%, hsla(222, 75%, 60%, 1) 0px, transparent 50%);
  }
`;
const Label = styled.label``;

const UserInput = ({ children }) => {
  return (
    <InputContainer className="md:w-[35rem] w-[20em]  mb-8 bg-gradient-to-r from-[#6B46C1] to-[#38B2AC]">
      <Label className="mb-2 text-[20px] text-white ml-1 font-mono uppercase font-bold">
        Create Proposal
      </Label>

      {children}
    </InputContainer>
  );
};

export default UserInput;
