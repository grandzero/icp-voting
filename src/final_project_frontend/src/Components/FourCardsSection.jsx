import React from "react";
import { useState, useEffect } from "react";
import ProposalListItems from "./Utilities/ProposalListItems";

const FourCardsSection = ({ proposalList }) => {
  const [proposals, setProposals] = useState(proposalList);

  useEffect(() => {
    if (proposalList.length > 0) {
      setProposals(proposalList);
    }
  }, [proposalList]);
  return (
    <div className="mt-4 p-4 grid lg:grid-cols-3 md:grid-cols-2 grid-cols-1 gap-5">
      {proposals?.reverse().map((item, index) => (
        <div key={index} className="flex justify-center">
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
