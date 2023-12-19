import React, { useState, useEffect } from "react";
import NavBar from "./NavBar";
import bgImg from "../../assets/700.jpg";
import { final_project_backend } from "../../../declarations/final_project_backend/index";


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
            let proposal  = await final_project_backend.get_proposal_list();
            console.log("Proposal list : ",proposal);
            // setProposalList(proposalList);
            // setProposalCount(proposalList.length);
        };

        if (true) {
            fetchProposals();
        }
    }, [proposalCount]);

    // Styles
    const container = "bg-[#000]  h-full   w-screen";
    const inputSectionStyle = `${currentProposal ? "" : "mt-[10%]"}`;
    // const bgImgStyle = "absolute rotate-180 object-fill h- w-full bg-repeat"
    const proposalListStyle =
        "items-end grid place-items-center grid-flow-row xl:grid-cols-2 gap-y-12 ";

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
                {/* {currentProposal && (
                    <HeroSection
                        proposalCount={proposalCount}
                        currentProposal={currentProposal}
                    />
                )} */}
                <div className={inputSectionStyle}>
                    {/* <RoundProfileSection
                        proposalList={proposalList}
                        currentProposal={currentProposal}
                        proposalCount={proposalCount}
                    /> */}
                </div>
                <div className={proposalListStyle}>
                    {/* Map Proposal list */}
                </div>
            </div>
        </div>
    );
};

export default MainPage;
