import React from "react";
import { useState, useEffect } from "react";
import ProposalListItems from "./Utilities/ProposalListItems";

const FourCardsSection = ({ proposalList }) => {
    const [proposals, setProposals] = useState(proposalList);

    const getProposals = async () => {};

    useEffect(() => {
        if (proposalList.length > 0) {
            setProposals(proposalList);
        }
    }, [proposalList]);
    return (
        <div className="mt-4 p-4 grid md:grid-cols-3 grid-cols-2  gap-5">
            {proposals?.reverse().map((item, index) => (
                <div key={index} className="">
                    <ProposalListItems
                        index={proposals.length - 1 - index}
                        proposal={item}
                    />
                </div>
            ))}
        </div>
    );
};

export default FourCardsSection;
