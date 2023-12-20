import React, { useState, useEffect } from "react";
import NavBar from "./NavBar";
import bgImg from "../../assets/700.jpg";
import { final_project_backend } from "../../../declarations/final_project_backend/index";
import RoundProfileSection from "./RoundProfileSection";
import FourCardsSection from "./FourCardsSection";
import HeroSection from "./HeroSection";

const MainPage = () => {
  const [currentProposal, setCurrentProposal] = useState();
  const [proposalList, setProposalList] = useState([]);
  const [proposalCount, setProposalCount] = useState(0);
  // Get Proposal List Function
  useEffect(() => {
    console.log("Entered useeffect");
    console.log(final_project_backend);
    const fetchProposals = async () => {
      // const arg= {val_test: 1, val: [1]} ;
      let proposals = await final_project_backend.get_proposal_list();
      console.log("Proposal list : ", proposals);
      setProposalList(proposals);
      setProposalCount(proposals.length);
      setCurrentProposal(proposals[proposals.length - 1]);
    };

    if (true) {
      fetchProposals();
    }
  }, [proposalCount]);

  // Styles
  const container = "bg-[#000]  h-full   w-screen";
  const inputSectionStyle = `${currentProposal ? "" : "mt-[10%]"}`;
  // const bgImgStyle = "absolute rotate-180 object-fill h- w-full bg-repeat"
  const proposalListStyle = "";

  return (
    <div className={container}>
      <div
        style={{
          backgroundImage: `url(${bgImg})`,
          backgroundRepeat: "no-repeat",
          backgroundSize: "cover",
        }}
      >
        <NavBar />

        {currentProposal && (
          <HeroSection
            proposalCount={proposalCount}
            currentProposal={currentProposal}
          />
        )}
        <div className={inputSectionStyle}>
          <RoundProfileSection
            proposalList={proposalList}
            currentProposal={currentProposal}
            proposalCount={proposalCount}
          />
        </div>
        <div className={proposalListStyle}>
          <FourCardsSection proposalList={proposalList} />
        </div>
      </div>
    </div>
  );
};

export default MainPage;
