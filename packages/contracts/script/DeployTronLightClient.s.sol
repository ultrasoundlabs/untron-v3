// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import "forge-std/Script.sol";
import "forge-std/StdJson.sol";
import "forge-std/console2.sol";

import {TronLightClient} from "../src/evm/TronLightClient.sol";
import {IBlockRangeProver} from "../src/evm/blockRangeProvers/interfaces/IBlockRangeProver.sol";

contract DeployTronLightClientScript is Script {
    using stdJson for string;

    function run() external returns (TronLightClient lc) {
        // ---- required env vars ----
        uint256 deployerPk = vm.envUint("PRIVATE_KEY");

        // ---- read config json ----
        string memory json = vm.readFile("script/tlc.json");

        address proverAddr = json.readAddress(".blockRangeProver");
        bytes32 initialBlockHash = json.readBytes32(".initialBlockHash"); // NOTE: must be the TRON blockId (height||tail), not just sha256(header)
        bytes32 initialTxTrieRoot = json.readBytes32(".initialTxTrieRoot");
        uint256 ts = json.readUint(".initialTimestamp"); // seconds
        require(ts <= type(uint32).max, "initialTimestamp > uint32");
        // forge-lint: disable-next-line(unsafe-typecast)
        uint32 initialTimestamp = uint32(ts);

        bytes32 srDataHash = json.readBytes32(".srDataHash");

        // srs + witnessDelegatees as address arrays in JSON, converted to bytes20[27]
        address[] memory srsAddr = json.readAddressArray(".srs");
        address[] memory delAddr = json.readAddressArray(".witnessDelegatees");

        require(srsAddr.length == 27, "srs must have 27 items");
        require(delAddr.length == 27, "witnessDelegatees must have 27 items");

        bytes20[27] memory srs;
        bytes20[27] memory witnessDelegatees;

        for (uint256 i = 0; i < 27; i++) {
            srs[i] = bytes20(srsAddr[i]);
            witnessDelegatees[i] = bytes20(delAddr[i]);
        }

        // ---- deploy ----
        vm.startBroadcast(deployerPk);

        lc = new TronLightClient(
            IBlockRangeProver(proverAddr),
            initialBlockHash,
            initialTxTrieRoot,
            initialTimestamp,
            srs,
            witnessDelegatees,
            srDataHash
        );

        vm.stopBroadcast();

        console2.log("TronLightClient deployed at:", address(lc));
        console2.logBytes32(lc.latestProvenBlock());
        console2.logBytes32(lc.SR_DATA_HASH());
    }
}
