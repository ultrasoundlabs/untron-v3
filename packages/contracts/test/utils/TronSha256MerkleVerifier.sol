// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Test} from "forge-std/Test.sol";
import {TronSha256MerkleVerifier} from "../../src/utils/TronSha256MerkleVerifier.sol";

contract TronSha256MerkleVerifierHarness {
    function verify(bytes32 root, bytes32 leaf, bytes32[] calldata proof, uint256 index) external pure returns (bool) {
        return TronSha256MerkleVerifier.verify(root, leaf, proof, index);
    }
}

contract TronSha256MerkleVerifierTest is Test {
    TronSha256MerkleVerifierHarness _harness;

    function setUp() public {
        _harness = new TronSha256MerkleVerifierHarness();
    }

    function _fixtureProof() internal pure returns (bytes32 root, bytes32 leaf, bytes32[] memory proof, uint256 index) {
        root = 0xe28d36f49e5b7f3f47b25c3d5db79197095683b727926dfa242ef181cca0e93a;
        leaf = 0x24036bec63e9b6b606c1fb5cb4c69819910d41856b108338c287df17cc2b1425;

        proof = new bytes32[](9);
        proof[0] = 0xdffbb6cfdb5a2b295a82926130b6bc148145cbfe27d132b4dcc2f9d4a71e684b;
        proof[1] = 0x5a3c3c6446c219e32b5b62cf83471969e106a37c8bdf5d00d5f558b45dd574a8;
        proof[2] = 0x8b173d5f8f2767f9b3c653fa11c8187c0c8219de4bb0c21421b9fcfd63be3a74;
        proof[3] = 0x4612f8909b7020008a329b682cd460647543310201398602298eb11d95915563;
        proof[4] = 0xed7815f3dcaca321885c11bc9e415465b6ef462d082d5ce0b5ea4728809ff55c;
        proof[5] = 0xdbe409cc4b49dea854d85b8e6e7bfed05db22aab6595db07305333822c40957e;
        proof[6] = 0x695a8aabc045ce096acf85eb5c36b70be756c6db6f53ae28030bc6d152cf2931;
        proof[7] = 0xeac3c407e13e5d5857caefc374a4597f95dc5550dc1c009d92a4a28e356e789e;
        proof[8] = 0xf328ece22ac0d2027070a0aca79ff5aed36a94e4f5820e360564f4f4fb8c0f21;

        // For this case, the index produced by the script is 0.
        index = 0;
    }

    function test_verify_AcceptsValidFixtureProof() public view {
        (bytes32 root, bytes32 leaf, bytes32[] memory proof, uint256 index) = _fixtureProof();

        bool ok = _harness.verify(root, leaf, proof, index);
        assertTrue(ok, "valid Tron tx Merkle proof should verify");
    }

    function test_verify_RejectsWrongLeaf() public view {
        (bytes32 root, bytes32 leaf, bytes32[] memory proof, uint256 index) = _fixtureProof();

        // Flip the lowest bit to get an obviously wrong leaf.
        leaf = bytes32(uint256(leaf) ^ uint256(1));

        bool ok = _harness.verify(root, leaf, proof, index);
        assertFalse(ok, "Merkle proof with wrong leaf must fail verification");
    }
}
