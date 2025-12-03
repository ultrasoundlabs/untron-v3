// test/TronLightClientFixture.t.sol
// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Test} from "forge-std/Test.sol";
import {TronLightClient} from "../../../src/evm/TronLightClient.sol";
import {IBlockRangeProver} from "../../../src/evm/blockRangeProvers/interfaces/IBlockRangeProver.sol";

contract TronLightClientFixtureTest is Test {
    TronLightClient internal client;

    // Fixture data
    bytes32 internal startingBlockId;
    bytes32 internal endingBlockId;
    bytes internal metadata;
    bytes internal sigs;
    // SR owner accounts (Tron witnesses).
    bytes20[27] internal srs;
    // Delegatee signing keys for each SR index.
    bytes20[27] internal witnessDelegatees;
    uint256[] internal blockNumbers;
    bytes32[] internal blockIds;

    function setUp() public {
        // Read JSON fixture
        string memory root = vm.projectRoot();
        string memory path = string.concat(
            root,
            // TODO: figure out how to iterate over all files in the fixtures folder
            "/test/evm/TronLightClient/fixtures/tron_78000000_78000099.json"
        );

        // forge-lint: disable-next-line(unsafe-cheatcode)
        string memory json = vm.readFile(path);

        // Parse primitive fields
        startingBlockId = vm.parseJsonBytes32(json, ".startingBlockId");
        endingBlockId = vm.parseJsonBytes32(json, ".endingBlockId");

        metadata = vm.parseJsonBytes(json, ".compressedTronBlockMetadata");
        sigs = vm.parseJsonBytes(json, ".compressedSignatures");

        // Parse blockNumbers and blockIds for additional checks
        {
            uint256[] memory numsDyn = vm.parseJsonUintArray(json, ".blockNumbers");
            bytes32[] memory idsDyn = vm.parseJsonBytes32Array(json, ".blockIds");
            blockNumbers = numsDyn;
            blockIds = idsDyn;
        }

        // Parse SRS as addresses (SR owner accounts), then cast to bytes20[27]
        address[] memory srsAddrs = vm.parseJsonAddressArray(json, ".srs");
        require(srsAddrs.length == 27, "fixture srs must be length 27");

        address[] memory delegateeAddrs = vm.parseJsonAddressArray(json, ".witnessDelegatees");
        require(delegateeAddrs.length == 27, "fixture witnessDelegatees must be length 27");

        for (uint256 i = 0; i < 27; i++) {
            srs[i] = bytes20(srsAddrs[i]);
            witnessDelegatees[i] = bytes20(delegateeAddrs[i]);
        }

        client = new TronLightClient(IBlockRangeProver(address(0)), startingBlockId, srs, witnessDelegatees);
    }

    function test_proveBlocks_happyPath_fixture() public {
        // Call proveBlocks with the real Tron data
        client.proveBlocks(startingBlockId, metadata, sigs);

        // latestProvenBlock should be the endingBlockId from the fixture
        assertEq(client.latestProvenBlock(), endingBlockId, "latestProvenBlock mismatch");

        // The last block in the proven range should be present in the ring buffer.
        uint256 numLast = blockNumbers[blockNumbers.length - 1];

        assertEq(client.getBlockId(numLast), blockIds[blockIds.length - 1], "getBlockId(last) mismatch");

        // We intentionally DO NOT store every intermediate block in storage; only
        // anchor blocks (e.g. the initial and ending ones) are persisted. A
        // mid-range block lookup should therefore fail with BlockNotRelayed.
        uint256 idxMid = blockNumbers.length / 2;
        uint256 numMid = blockNumbers[idxMid];
        vm.expectRevert(TronLightClient.BlockNotRelayed.selector);
        client.getBlockId(numMid);

        // The starting anchor used in the constructor should also be present;
        // for this fixture it is the parent of blockNumbers[0].
        uint256 parentNum = blockNumbers[0] - 1;
        assertEq(client.getBlockId(parentNum), startingBlockId, "getBlockId(parent) mismatch");
    }

    function test_proveBlocks_revertsOnInvalidSignature() public {
        // Corrupt a single byte in the signatures to trigger InvalidWitnessSigner
        bytes memory badSigs = sigs;
        badSigs[0] = bytes1(uint8(badSigs[0]) ^ 0x01);

        vm.expectRevert(TronLightClient.InvalidWitnessSigner.selector);
        client.proveBlocks(startingBlockId, metadata, badSigs);
    }
}
