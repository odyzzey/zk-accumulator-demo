use methods::{VOTE_ID, VOTE_PATH};
use risc0_zkvm::{Prover, Receipt};
use risc0_zkvm::serde::to_vec;
use types::{PointVote, ContractPoint};

fn main() {
    /* Data Layer starts
        * Code is stored on the data layer or, really, anywhere the prover can access
        * The only thing that needs to be stored on the main chain is the */

    let method_code = std::fs::read(VOTE_PATH) 
        .expect("Method code should be present at the specified path; did you use the correct *_PATH constant?");
    // Data Layer ends


    /* Execution Layer starts
        * The execution layer is a network of provers peered with one another.
        * In this demo we sequentially simulate 2 provers. 
        * A production network would have provers run in parallel. */

    let mut prover = Prover::new(&method_code, VOTE_ID).expect(
        "Prover should be constructed from valid method source code and corresponding method ID",
    ); // prover makes itself available to receive transactions
   
    for i in 0..10 {
        let vote = PointVote::new(i, i, 1);
        add_vote(&mut prover, vote);
    }

    let receipt = close_vote_and_prove(&mut prover); // transactions received while closed are relayed to a peer prover 

    prover = Prover::new(&method_code, VOTE_ID).expect(
        "Prover should be constructed from valid method source code and corresponding method ID",
    );  // second prover also bundles votes at the same time
    
    for i in 0..10 {
        let vote = PointVote::new(i * 2, i * 2, 1);
        add_vote(&mut prover, vote);
    }

    let receipt2 = close_vote_and_prove(&mut prover);

    // Execution Layer ends


    /* Settlement Layer starts
        * Provers finally relay their receipts to the main network and intiate transactions.
        * State transition is handled at the smart contract level
        * Verification would happen in a zk verification module on a Cosmos chain. */

    let mut contract = ContractPoint::new();
    // verify votes/receipt from the first prover
    receipt.verify(VOTE_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct method ID?",
    ); // note again that only the method_id is needed on the verifier side--not the code itself

    // settle and transition contract state
    contract = settle_vote(&contract, &receipt);
    println!("Transaction 1: \n\t Receipt: {:?} \n\t {:?}", &receipt.journal, &contract);

    // verify votes/receipt from the second prover
    receipt2.verify(VOTE_ID).expect(
        "Code you have proven should successfully verify; did you specify the correct method ID?",
    );

    contract = settle_vote(&contract, &receipt2);
    println!("Transaction 2: \n\t Receipt: {:?} \n\t {:?}", &receipt2.journal, &contract);
    // Settlement Layer ends
}

fn add_vote(prover: &mut Prover, vote: PointVote) {
    prover.add_input_u32_slice(&to_vec(&1).expect("Should be able to serialize"));

    prover.add_input_u32_slice(&to_vec(&vote.get_x()).expect("x error"));
    prover.add_input_u32_slice(&to_vec(&vote.get_y()).expect("y error"));
    prover.add_input_u32_slice(&to_vec(&vote.get_weight()).expect("weight error"));
}

fn close_vote_and_prove(prover: &mut Prover) -> Receipt {
    prover.add_input_u32_slice(&to_vec(&0).expect("Should be able to serialize"));
    prover.run().expect("Code 1) had an error or 2) overflowed the cycle limit.")
}

fn settle_vote(contract: &ContractPoint, receipt: &Receipt) -> ContractPoint {
    let journal = receipt.journal.to_vec();
    let transaction_vote = PointVote::new(journal[0].try_into().unwrap(), journal[1].try_into().unwrap(), journal[2]);
    contract.add(transaction_vote)
}
