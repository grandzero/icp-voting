#!/bin/bash
dfx canister call b77ix-eeaaa-aaaaa-qaada-cai create_privileged_proposal \
'(
    0,
    record {
        description = "This is a privileged proposal";
        is_active = true;
        nft_canister = principal "asrmz-lmaaa-aaaaa-qaaeq-cai";
    }
)'