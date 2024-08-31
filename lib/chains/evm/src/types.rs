use alloy::sol;

sol!(
	#[allow(missing_docs)]
	#[sol(rpc)]
	Bridge,
	"contracts/Bridge.json"
);

sol!(
	#[allow(missing_docs)]
	#[sol(rpc)]
	Erc20Handler,
	"contracts/ERC20Handler.json"
);
