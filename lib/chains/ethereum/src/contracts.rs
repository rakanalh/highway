use ethers_contract::abigen;

abigen!(
	IBridge,
	r#"[
        event Deposit(uint8 indexed destinationChainID, bytes32 indexed resourceID, uint64 indexed depositNonce)
	    event Proposal(uint8 indexed originChainID,uint64 indexed depositNonce,uint8 indexed status,bytes32 resourceID,bytes32 dataHash)
        event ProposalVote(uint8 indexed originChainID,uint64 indexed depositNonce,uint8 indexed status,bytes32 resourceID)
    ]"#,
);

abigen!(
	IErc20Handler,
	r#"[
        struct DepositRecord {address _tokenAddress; uint8 _lenDestinationRecipientAddress; uint8 _destinationChainID; bytes32 _resourceID; bytes   _destinationRecipientAddress; address _depositer; uint    _amount;}

        function getDepositRecord(uint64 depositNonce, uint8 destId) external view returns (DepositRecord memory)
    ]"#,
);
