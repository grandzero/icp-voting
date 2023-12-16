use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap, Storable};
use std::{borrow::Cow, cell::RefCell};

type Memory = VirtualMemory<DefaultMemoryImpl>;

const MAX_VALUE_SIZE: u32 = 5000;

#[derive(CandidType, Deserialize, Debug)]
enum Choice {
    Aprrove,
    Reject,
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
struct CreateProposal {
    description: String,
    is_active: bool,
}

impl Storable for Proposal {
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
}

#[ic_cdk::query]
fn get_proposal(key: u64) -> Option<Proposal> {
    PROPOSALS.with(|proposals| proposals.borrow().get(&key))
}

#[ic_cdk::query]
fn get_proposal_count() -> u64 {
    PROPOSALS.with(|proposals| proposals.borrow().len() as u64)
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
        PROPOSALS.with(|proposals| proposals.borrow_mut().insert(key, proposal))
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
