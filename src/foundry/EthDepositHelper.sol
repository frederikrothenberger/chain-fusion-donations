// SPDX-License-Identifier: Apache-2.0

pragma solidity 0.8.18;

/**
 * @title A helper smart contract for ETH <-> ckETH conversion.
 * @notice This smart contract deposits incoming ETH to the ckETH minter account and emits deposit events.
 */
contract EthDeposit {
    address payable private immutable chainFusionStakingAddress;

    event ReceivedEth(address indexed from, uint256 value);

    /**
     * @dev Set cketh_minter_main_address.
     */
    constructor(address _chainFusionStakingAddress) {
        chainFusionStakingAddress = payable(_chainFusionStakingAddress);
    }

    /**
     * @dev Return ckETH minter main address.
     * @return address of ckETH minter main address.
     */
    function getMinterAddress() public view returns (address) {
        return chainFusionStakingAddress;
    }

    /**
     * @dev Emits the `ReceivedEth` event if the transfer succeeds.
     */
    function deposit() public payable {
        emit ReceivedEth(msg.sender, msg.value);
        chainFusionStakingAddress.transfer(msg.value);
    }
}
