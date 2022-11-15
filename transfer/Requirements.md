Write generic transfer contract
instatiate with me as owner/an owner.

have an execute message where an account/addr can send funds to the instantiated contract which has a specific owner, and specify two account/addr as beneficiary (so this is not during instatiation) ....so... address 1 and 2 as beneficiary, and a sent contract amount/funds evenly split between two beneficiaries
The beneficiaries can withdraw when and how much they like till their balance is zero

store funds in the contract for every account that is non zero. Meaning whenever an account hits zero/is withdrawn to zero, it should be removed from storage/deleted

support an execute message where an account can withdraw its funds - either some or all

support a read query to get the owner of the smart contract. Is it the owner of an intantiated contract? or the owner of the smart contract....for each instantiated contract, there should be a read query to get the owner of the contract

read query to get the withdrawable coins of any account

I/and beneficiaries should be able to check each beneficiary balance for all (owner) and for beneficiary (for each beneficiary) for a contract with a certain owner


implement a fee structure for the tranfer contract, where each send incurs fees that are collectable by the contract owner.

