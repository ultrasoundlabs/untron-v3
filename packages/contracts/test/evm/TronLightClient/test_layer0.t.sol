// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Test} from "forge-std/Test.sol";
import {TronLightClientHarness} from "./harness/TronLightClientHarness.sol";

import {IBlockRangeProver} from "../../../src/evm/blockRangeProvers/interfaces/IBlockRangeProver.sol";

contract TronLightClientLayer0Test is Test {
    TronLightClientHarness internal _client;

    // Fixture data used for pure/low-level checks
    bytes internal _metadata;
    uint256[] internal _blockNumbers;
    bytes32[] internal _blockIds;
    bytes32[] internal _blockHashes;
    bytes[] internal _rawHeaderBytes;
    address[] internal _witnessEvmAddresses;
    uint8[] internal _witnessIndices;

    function setUp() public {
        string memory root = vm.projectRoot();
        // Reuse the same fixture as layer1 so all tests agree on the data source.
        string memory path = string.concat(root, "/test/evm/fixtures/tron_78000000_78000099.json");

        // forge-lint: disable-next-line(unsafe-cheatcode)
        string memory json = vm.readFile(path);

        // Parse packed metadata blob used by the client.
        _metadata = vm.parseJsonBytes(json, ".compressedTronBlockMetadata");

        // Parse block numbers and ids (uint256 / bytes32 arrays).
        {
            uint256[] memory numsDyn = vm.parseJsonUintArray(json, ".blockNumbers");
            bytes32[] memory idsDyn = vm.parseJsonBytes32Array(json, ".blockIds");
            _blockNumbers = numsDyn;
            _blockIds = idsDyn;
        }

        // Parse reference hashes and raw header bytes for hashBlock tests.
        {
            bytes32[] memory hashesDyn = vm.parseJsonBytes32Array(json, ".blockHashes");
            _blockHashes = hashesDyn;

            // rawHeaderBytes is an array of arbitrary-length byte blobs, so we parse entries
            // individually instead of using a single helper that returns bytes[].
            uint256 len = _blockNumbers.length;
            _rawHeaderBytes = new bytes[](len);
            for (uint256 i = 0; i < len; i++) {
                // e.g. `.blockHeaderRawBytes[0]`, `.blockHeaderRawBytes[1]`, ...
                string memory key = string.concat(".blockHeaderRawBytes[", vm.toString(i), "]");
                _rawHeaderBytes[i] = vm.parseJsonBytes(json, key);
            }
        }

        // Parse witness reference data.
        {
            address[] memory addrsDyn = vm.parseJsonAddressArray(json, ".witnessEvmAddresses");
            _witnessEvmAddresses = addrsDyn;

            uint256[] memory idxDyn = vm.parseJsonUintArray(json, ".witnessIndices");
            uint256 lenIdx = idxDyn.length;
            _witnessIndices = new uint8[](lenIdx);
            for (uint256 i = 0; i < lenIdx; i++) {
                _witnessIndices[i] = uint8(idxDyn[i]);
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

        // Build the witness delegatees set from the fixture's witnessDelegatees field, as in layer1.
        bytes20[27] memory witnessDelegatees;
        {
            address[] memory delegateeAddrs = vm.parseJsonAddressArray(json, ".witnessDelegatees");
            require(delegateeAddrs.length == 27, "fixture witnessDelegatees must be length 27");
            for (uint256 i = 0; i < 27; i++) {
                witnessDelegatees[i] = bytes20(delegateeAddrs[i]);
            }
        }

        // Use the startingBlockId and startingBlockTxTrieRoot from the fixture to satisfy the constructor.
        bytes32 startingBlockId = vm.parseJsonBytes32(json, ".startingBlockId");
        bytes32 startingBlockTxTrieRoot = vm.parseJsonBytes32(json, ".startingBlockTxTrieRoot");
        // Retrieve the starting block timestamp (seconds) from the fixture.
        // The fixture stores this as a string; convert it to uint32 for the constructor.
        uint256 startingTimestampSec = vm.parseJsonUint(json, ".startingBlockTimestamp");
        _client = new TronLightClientHarness(
            IBlockRangeProver(address(0)),
            startingBlockId,
            startingBlockTxTrieRoot,
            // casting to 'uint32' is safe because startingTimestampSec is guaranteed to fit within 32 bits
            // forge-lint: disable-next-line(unsafe-typecast)
            uint32(startingTimestampSec & 0xFFFFFFFF),
            srs,
            witnessDelegatees,
            bytes32(0) // TODO: fix
        );
    }

    /// @notice Sanity check that decoding the packed metadata yields the same
    /// parentHash/txTrieRoot/timestamp/witnessIndex as the reference JSON fields.
    function test_decodeTronBlockMetadata_matchesFixture() public view {
        uint256 len = _blockNumbers.length;
        assertGt(len, 0, "fixture must contain at least one block");

        for (uint256 i = 0; i < len; i++) {
            (,,, uint8 witnessIndex) = _client.decodeAt(_metadata, i);

            // Parent/txTrieRoot are not explicitly exposed in the JSON today, but we can at
            // least assert the witness index matches the fixture's witnessIndices.
            assertEq(uint256(witnessIndex), uint256(_witnessIndices[i]), "witness index mismatch");
        }
    }

    /// @notice hashBlock should reproduce the sha256(BlockHeader_raw) from the fixture,
    /// given the same number, metadata and SRS mapping.
    function test_hashBlock_matchesFixtureHash() public view {
        uint256 len = _blockNumbers.length;
        assertGt(len, 0, "fixture must contain at least one block");
        assertEq(len, _blockHashes.length, "blockNumbers/blockHashes length mismatch");

        for (uint256 i = 0; i < len; i++) {
            (bytes32 parentHash, bytes32 txTrieRoot, uint32 timestamp, uint8 witnessIndex) =
                _client.decodeAt(_metadata, i);

            bytes32 computed =
                _client.hashBlockPublic(parentHash, txTrieRoot, timestamp, witnessIndex, _blockNumbers[i]);
            assertEq(computed, _blockHashes[i], "hashBlock mismatch");
        }
    }

    /// @notice The raw protobuf encoding produced by the Solidity implementation should
    /// match the bytes emitted by the TypeScript `BlockHeader_raw.encode` for each block.
    function test_encodeBlockHeader_matchesFixtureRawBytes() public view {
        uint256 len = _blockNumbers.length;
        assertGt(len, 0, "fixture must contain at least one block");
        assertEq(len, _rawHeaderBytes.length, "blockNumbers/rawHeaderBytes length mismatch");

        for (uint256 i = 0; i < len; i++) {
            (bytes32 parentHash, bytes32 txTrieRoot, uint32 timestamp, uint8 witnessIndex) =
                _client.decodeAt(_metadata, i);

            bytes memory encoded =
                _client.encodeBlockHeaderPublic(parentHash, txTrieRoot, timestamp, witnessIndex, _blockNumbers[i]);
            // forge-std's assertEq on bytes performs a byte-for-byte comparison.
            assertEq(encoded, _rawHeaderBytes[i], "encoded header bytes mismatch");
        }
    }
}
