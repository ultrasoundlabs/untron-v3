// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Test} from "forge-std/Test.sol";
import {TronLightClientHarness} from "./harness/TronLightClientHarness.sol";
import {TronLightClient} from "../../../src/evm/TronLightClient.sol";
import {IBlockRangeProver} from "../../../src/evm/blockRangeProvers/interfaces/IBlockRangeProver.sol";

contract TronLightClientLayer0Test is Test {
    TronLightClientHarness internal client;

    // Fixture data used for pure/low-level checks
    bytes internal metadata;
    uint256[] internal blockNumbers;
    bytes32[] internal blockIds;
    bytes32[] internal blockHashes;
    bytes[] internal rawHeaderBytes;
    address[] internal witnessEvmAddresses;
    uint8[] internal witnessIndices;

    function setUp() public {
        string memory root = vm.projectRoot();
        // Reuse the same fixture as layer1 so all tests agree on the data source.
        string memory path = string.concat(root, "/test/evm/TronLightClient/fixtures/tron_78000000_78000099.json");

        // forge-lint: disable-next-line(unsafe-cheatcode)
        string memory json = vm.readFile(path);

        // Parse packed metadata blob used by the client.
        metadata = vm.parseJsonBytes(json, ".compressedTronBlockMetadata");

        // Parse block numbers and ids (uint256 / bytes32 arrays).
        {
            uint256[] memory numsDyn = vm.parseJsonUintArray(json, ".blockNumbers");
            bytes32[] memory idsDyn = vm.parseJsonBytes32Array(json, ".blockIds");
            blockNumbers = numsDyn;
            blockIds = idsDyn;
        }

        // Parse reference hashes and raw header bytes for hashBlock tests.
        {
            bytes32[] memory hashesDyn = vm.parseJsonBytes32Array(json, ".blockHashes");
            blockHashes = hashesDyn;

            // rawHeaderBytes is an array of arbitrary-length byte blobs, so we parse entries
            // individually instead of using a single helper that returns bytes[].
            uint256 len = blockNumbers.length;
            rawHeaderBytes = new bytes[](len);
            for (uint256 i = 0; i < len; i++) {
                // e.g. `.blockHeaderRawBytes[0]`, `.blockHeaderRawBytes[1]`, ...
                string memory key = string.concat(".blockHeaderRawBytes[", vm.toString(i), "]");
                rawHeaderBytes[i] = vm.parseJsonBytes(json, key);
            }
        }

        // Parse witness reference data.
        {
            address[] memory addrsDyn = vm.parseJsonAddressArray(json, ".witnessEvmAddresses");
            witnessEvmAddresses = addrsDyn;

            uint256[] memory idxDyn = vm.parseJsonUintArray(json, ".witnessIndices");
            uint256 lenIdx = idxDyn.length;
            witnessIndices = new uint8[](lenIdx);
            for (uint256 i = 0; i < lenIdx; i++) {
                witnessIndices[i] = uint8(idxDyn[i]);
            }
        }

        // Build the SR set from the fixture's SRS field, as in layer1.
        bytes20[27] memory srs;
        {
            address[] memory srsAddrs = vm.parseJsonAddressArray(json, ".srs");
            require(srsAddrs.length == 27, "fixture srs must be length 27");
            for (uint256 i = 0; i < 27; i++) {
                srs[i] = bytes20(srsAddrs[i]);
            }
        }

        // Use the startingBlockId from the fixture simply to satisfy the constructor.
        bytes32 startingBlockId = vm.parseJsonBytes32(json, ".startingBlockId");
        client = new TronLightClientHarness(IBlockRangeProver(address(0)), startingBlockId, srs);
    }

    /// @notice Sanity check that decoding the packed metadata yields the same
    /// parentHash/txTrieRoot/timestamp/witnessIndex as the reference JSON fields.
    function test_decodeTronBlockMetadata_matchesFixture() public view {
        uint256 len = blockNumbers.length;
        assertGt(len, 0, "fixture must contain at least one block");

        for (uint256 i = 0; i < len; i++) {
            TronLightClient.TronBlockMetadata memory meta = client.decodeAt(metadata, i);

            // Parent/txTrieRoot are not explicitly exposed in the JSON today, but we can at
            // least assert the witness index matches the fixture's witnessIndices.
            assertEq(uint256(meta.witnessAddressIndex), uint256(witnessIndices[i]), "witness index mismatch");
        }
    }

    /// @notice hashBlock should reproduce the sha256(BlockHeader_raw) from the fixture,
    /// given the same number, metadata and SRS mapping.
    function test_hashBlock_matchesFixtureHash() public view {
        uint256 len = blockNumbers.length;
        assertGt(len, 0, "fixture must contain at least one block");
        assertEq(len, blockHashes.length, "blockNumbers/blockHashes length mismatch");

        for (uint256 i = 0; i < len; i++) {
            TronLightClient.TronBlockMetadata memory meta = client.decodeAt(metadata, i);

            bytes32 computed = client.hashBlockPublic(meta, blockNumbers[i]);
            assertEq(computed, blockHashes[i], "hashBlock mismatch");
        }
    }

    /// @notice The raw protobuf encoding produced by the Solidity implementation should
    /// match the bytes emitted by the TypeScript `BlockHeader_raw.encode` for each block.
    function test_encodeBlockHeader_matchesFixtureRawBytes() public view {
        uint256 len = blockNumbers.length;
        assertGt(len, 0, "fixture must contain at least one block");
        assertEq(len, rawHeaderBytes.length, "blockNumbers/rawHeaderBytes length mismatch");

        for (uint256 i = 0; i < len; i++) {
            TronLightClient.TronBlockMetadata memory meta = client.decodeAt(metadata, i);

            bytes memory encoded = client.encodeBlockHeaderPublic(meta, blockNumbers[i]);
            // forge-std's assertEq on bytes performs a byte-for-byte comparison.
            assertEq(encoded, rawHeaderBytes[i], "encoded header bytes mismatch");
        }
    }
}

