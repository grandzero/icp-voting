use candid::{CandidType, Decode, Deserialize, Encode};
use ic_cdk::api::call::{call, CallResult};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::result;
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;

const MAX_VALUE_SIZE: u32 = 5000;

#[derive(CandidType, Deserialize, Debug)]
enum Choice {
    #[serde(rename = "Approve")]
    Aprrove,
    #[serde(rename = "Reject")]
    Reject,
    #[serde(rename = "Pass")]
    Pass,
}
#[derive(CandidType, Deserialize, Debug)]
enum VoteError {
    AlreadyVoted,
    ProposalDoesNotExist,
    ProposalIsNotActive,
    AccessRejected,
    UpdateError,
}
#[derive(CandidType, Deserialize, Debug)]
struct Proposal {
    description: String,
    approve: u32,
    reject: u32,
    pass: u32,
    is_active: bool,
    voters: Vec<candid::Principal>,
    owner: candid::Principal,
}

#[derive(CandidType, Deserialize, Debug)]
struct PrivilegedProposal {
    description: String,
    approve: u32,
    reject: u32,
    pass: u32,
    is_active: bool,
    voters: Vec<candid::Principal>,
    owner: candid::Principal,
    nft_canister: candid::Principal,
}

#[derive(CandidType, Deserialize, Debug)]
struct CreateProposal {
    description: String,
    is_active: bool,
}

#[derive(CandidType, Deserialize, Debug)]
struct CreatePrivilegedProposal {
    description: String,
    is_active: bool,
    nft_canister: candid::Principal,
}

impl Storable for Proposal {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for PrivilegedProposal {
    const MAX_SIZE: u32 = MAX_VALUE_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

impl Storable for PrivilegedProposal {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}

impl BoundedStorable for Proposal {
    const MAX_SIZE: u32 = MAX_VALUE_SIZE;
    const IS_FIXED_SIZE: bool = false;
}

thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> = RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));
    static PROPOSALS: RefCell<StableBTreeMap<u64, Proposal, Memory>> = RefCell::new(StableBTreeMap::init(MEMORY_MANAGER.with(|mm| mm.borrow().get(MemoryId::new(0)))));
    // Privileged Proposals can only be created and voted by certain nft(dip721) holders
    static PRIVILEGED_PROPOSALS: RefCell<StableBTreeMap<u64, PrivilegedProposal, Memory>> = RefCell::new(StableBTreeMap::init(MEMORY_MANAGER.with(|mm| mm.borrow().get(MemoryId::new(1)))));
}

#[ic_cdk::query]
fn get_proposal(key: u64) -> String {
    if let Some(prop) = PROPOSALS.with(|proposals| proposals.borrow().get(&key)) {
        String::from(format!("Proposer : {}, Description: {}, Approve: {}, Reject: {}, Pass: {}, Voters: {}, Is Active: {}", prop.owner.clone().to_text() , prop.description, prop.approve, prop.reject, prop.pass, prop.voters.len(), prop.is_active))
    } else {
        String::from("Proposal does not exist")
    }
}

// Privileged Proposals Details
#[ic_cdk::query]
fn get_privileged_proposal(key: u64) -> String {
    if let Some(prop) = PRIVILEGED_PROPOSALS.with(|proposals| proposals.borrow().get(&key)) {
        String::from(format!("Proposer : {}, Description: {}, Approve: {}, Reject: {}, Pass: {}, Voters: {}, Is Active: {}, Collection: {}", prop.owner.clone().to_text() , prop.description, prop.approve, prop.reject, prop.pass, prop.voters.len(), prop.is_active, prop.nft_canister.clone().to_text()))
    } else {
        String::from("Proposal does not exist")
    }
}

#[ic_cdk::query]
fn get_proposal_count() -> u64 {
    PROPOSALS.with(|proposals| proposals.borrow().len() as u64)
}

// Privileged Proposals Count
#[ic_cdk::query]
fn get_privileged_proposal_count() -> u64 {
    PRIVILEGED_PROPOSALS.with(|proposals| proposals.borrow().len() as u64)
}

#[ic_cdk::update]
fn create_proposal(key: u64, proposal: CreateProposal) -> Option<Proposal> {
    PROPOSALS.with(|proposals| {
        let mut proposals = proposals.borrow_mut();
        if proposals.contains_key(&key) {
            return None;
        }
        let proposal = Proposal {
            description: proposal.description,
            approve: 0,
            reject: 0,
            pass: 0,
            is_active: proposal.is_active,
            voters: Vec::new(),
            owner: ic_cdk::caller(),
        };
        proposals.insert(key, proposal)
    })
}
// Create Privileged Proposals
#[ic_cdk::update]
async fn create_privileged_proposal(
    key: u64,
    proposal: CreatePrivilegedProposal,
) -> Option<PrivilegedProposal> {
    // Check if user has nft that allows him to create privileged proposals

    let user_principal =  ic_cdk::caller()/* the principal you want to pass as argument */;
    let result: CallResult<(u64,)> =
        call(proposal.nft_canister, "balanceOfDip721", (user_principal,)).await;

    if let result::Result::Ok((balance,)) = result {
        if balance == 0 {
            return None;
        }
    }
    PRIVILEGED_PROPOSALS.with(|proposals| {
        let mut proposals = proposals.borrow_mut();
        if proposals.contains_key(&key) {
            return None;
        }
        let proposal = PrivilegedProposal {
            description: proposal.description,
            approve: 0,
            reject: 0,
            pass: 0,
            is_active: proposal.is_active,
            voters: Vec::new(),
            owner: ic_cdk::caller(),
            nft_canister: proposal.nft_canister,
        };
        proposals.insert(key, proposal)
    })
}

#[ic_cdk::update]
fn edit_proposal(key: u64, proposal: CreateProposal) -> Result<(), VoteError> {
    PROPOSALS.with(|proposals| {
        let old_proposal_opt = proposals.borrow().get(&key);
        let old_proposal: Proposal = match old_proposal_opt {
            Some(old_proposal) => old_proposal,
            None => return Err(VoteError::ProposalDoesNotExist),
        };
        if old_proposal.owner != ic_cdk::caller() {
            return Err(VoteError::AccessRejected);
        }
        let new_proposal = Proposal {
            description: proposal.description,
            approve: old_proposal.approve,
            reject: old_proposal.reject,
            pass: old_proposal.pass,
            is_active: proposal.is_active,
            voters: old_proposal.voters,
            owner: old_proposal.owner,
        };
        if let Some(_) = proposals.borrow_mut().insert(key, new_proposal) {
            Ok(())
        } else {
            Err(VoteError::UpdateError)
        }
    })
}

#[ic_cdk::update]
async fn edit_privileged_proposal(
    key: u64,
    proposal: CreatePrivilegedProposal,
) -> Result<(), VoteError> {
    PRIVILEGED_PROPOSALS.with(|proposals| {
        let old_proposal_opt = proposals.borrow().get(&key);
        let old_proposal: PrivilegedProposal = match old_proposal_opt {
            Some(old_proposal) => old_proposal,
            None => return Err(VoteError::ProposalDoesNotExist),
        };
        if old_proposal.owner != ic_cdk::caller() {
            return Err(VoteError::AccessRejected);
        }
        let new_proposal = PrivilegedProposal {
            description: proposal.description,
            approve: old_proposal.approve,
            reject: old_proposal.reject,
            pass: old_proposal.pass,
            is_active: proposal.is_active,
            voters: old_proposal.voters,
            owner: old_proposal.owner,
            nft_canister: old_proposal.nft_canister,
        };
        if let Some(_) = proposals.borrow_mut().insert(key, new_proposal) {
            Ok(())
        } else {
            Err(VoteError::UpdateError)
        }
    })
}

#[ic_cdk::update]
fn end_proposal(key: u64) -> Result<(), VoteError> {
    PROPOSALS.with(|proposals| {
        let old_proposal_opt = proposals.borrow().get(&key);
        let mut old_proposal: Proposal = match old_proposal_opt {
            Some(old_proposal) => old_proposal,
            None => return Err(VoteError::ProposalDoesNotExist),
        };
        if old_proposal.owner != ic_cdk::caller() {
            return Err(VoteError::AccessRejected);
        }
        old_proposal.is_active = false;
        if let Some(_) = proposals.borrow_mut().insert(key, old_proposal) {
            Ok(())
        } else {
            Err(VoteError::UpdateError)
        }
    })
}

// End privileged Proposal
#[ic_cdk::update]
fn end_privileged_proposal(key: u64) -> Result<(), VoteError> {
    PRIVILEGED_PROPOSALS.with(|proposals| {
        let old_proposal_opt = proposals.borrow().get(&key);
        let mut old_proposal: PrivilegedProposal = match old_proposal_opt {
            Some(old_proposal) => old_proposal,
            None => return Err(VoteError::ProposalDoesNotExist),
        };
        if old_proposal.owner != ic_cdk::caller() {
            return Err(VoteError::AccessRejected);
        }
        old_proposal.is_active = false;
        if let Some(_) = proposals.borrow_mut().insert(key, old_proposal) {
            Ok(())
        } else {
            Err(VoteError::UpdateError)
        }
    })
}

#[ic_cdk::update]
fn vote(key: u64, choice: Choice) -> Result<(), VoteError> {
    PROPOSALS.with(|proposals| {
        let old_proposal_opt = proposals.borrow().get(&key);
        let mut old_proposal: Proposal = match old_proposal_opt {
            Some(old_proposal) => old_proposal,
            None => return Err(VoteError::ProposalDoesNotExist),
        };
        if !old_proposal.is_active {
            return Err(VoteError::ProposalIsNotActive);
        }
        if old_proposal.voters.contains(&ic_cdk::caller()) {
            return Err(VoteError::AlreadyVoted);
        }
        old_proposal.voters.push(ic_cdk::caller());
        match choice {
            Choice::Aprrove => old_proposal.approve += 1,
            Choice::Reject => old_proposal.reject += 1,
            Choice::Pass => old_proposal.pass += 1,
        };
        if let Some(_) = proposals.borrow_mut().insert(key, old_proposal) {
            Ok(())
        } else {
            Err(VoteError::UpdateError)
        }
    })
}
// Vote for privileged proposal
#[ic_cdk::update]
async fn vote_privileged(key: u64, choice: Choice) -> Result<(), VoteError> {
    let privileged_proposal: Option<PrivilegedProposal> =
        PRIVILEGED_PROPOSALS.with(|proposals| proposals.borrow().get(&key));
    if let Some(proposal) = privileged_proposal {
        let user_principal =  ic_cdk::caller()/* the principal you want to pass as argument */;
        let result: CallResult<(u64,)> =
            call(proposal.nft_canister, "balanceOfDip721", (user_principal,)).await;
        if let result::Result::Ok((balance,)) = result {
            if balance == 0 {
                return Err(VoteError::AccessRejected);
            }
        }
    } else {
        return Err(VoteError::ProposalDoesNotExist);
    }

    PRIVILEGED_PROPOSALS.with(|proposals| {
        let old_proposal_opt = proposals.borrow().get(&key);
        let mut old_proposal: PrivilegedProposal = match old_proposal_opt {
            Some(old_proposal) => old_proposal,
            None => return Err(VoteError::ProposalDoesNotExist),
        };
        if !old_proposal.is_active {
            return Err(VoteError::ProposalIsNotActive);
        }
        if old_proposal.voters.contains(&ic_cdk::caller()) {
            return Err(VoteError::AlreadyVoted);
        }

        old_proposal.voters.push(ic_cdk::caller());
        match choice {
            Choice::Aprrove => old_proposal.approve += 1,
            Choice::Reject => old_proposal.reject += 1,
            Choice::Pass => old_proposal.pass += 1,
        };
        if let Some(_) = proposals.borrow_mut().insert(key, old_proposal) {
            Ok(())
        } else {
            Err(VoteError::UpdateError)
        }
    })
}
