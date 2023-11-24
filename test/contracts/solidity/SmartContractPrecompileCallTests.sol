// SPDX-License-Identifier: GPL-3.0-only
pragma solidity >=0.8.3;

import "./Batch.sol";

/// @notice Smart contract to test precompile calls
contract SmartContractPrecompileCallTest {
    function callBatch(address to, bytes[] memory callData) external {
        address[] memory toAddress = new address[](1);
        toAddress[0] = to;
        uint256[] memory value = new uint256[](1);
        value[0] = 0;
        uint64[] memory gasLimit = new uint64[](1);
        gasLimit[0] = 0;
        BATCH_CONTRACT.batchAll(toAddress, value, callData, gasLimit);
    }
}
