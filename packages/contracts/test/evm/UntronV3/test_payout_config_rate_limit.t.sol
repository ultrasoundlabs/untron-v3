// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Test} from "forge-std/Test.sol";
import {UntronV3} from "../../../src/evm/UntronV3.sol";

contract UntronV3PayoutConfigHarness is UntronV3 {
    constructor(address controllerAddress, bytes1 create2Prefix, address tronReader_)
        UntronV3(controllerAddress, create2Prefix, tronReader_)
    {}
}

contract UntronV3PayoutConfigRateLimitTest is Test {
    UntronV3PayoutConfigHarness internal untron;

    address internal constant DUMMY_USDT = address(0x1000);
    address internal constant CONTROLLER = address(0xCAFE);

    function setUp() public {
        untron = new UntronV3PayoutConfigHarness(CONTROLLER, 0xff, address(0));
        untron.setUsdt(DUMMY_USDT);
        untron.setRealtor(address(this), true);
    }

    function testSetPayoutConfigIsRateLimited() public {
        address lessee = address(0xBEEF);
        uint256 leaseId = _createLeaseForLessee(lessee);

        untron.setLesseePayoutConfigRateLimit(2, 1 hours);

        vm.startPrank(lessee);
        untron.setPayoutConfig(leaseId, block.chainid, DUMMY_USDT, address(0x1111));
        untron.setPayoutConfig(leaseId, block.chainid, DUMMY_USDT, address(0x2222));
        vm.expectRevert(UntronV3.PayoutConfigRateLimitExceeded.selector);
        untron.setPayoutConfig(leaseId, block.chainid, DUMMY_USDT, address(0x3333));
        vm.stopPrank();

        vm.warp(block.timestamp + 1 hours);

        vm.prank(lessee);
        untron.setPayoutConfig(leaseId, block.chainid, DUMMY_USDT, address(0x4444));
    }

    function testSetPayoutConfigWithSigIsRateLimited() public {
        uint256 lesseeKey = 0xBEEF;
        address lessee = vm.addr(lesseeKey);
        uint256 leaseId = _createLeaseForLessee(lessee);

        untron.setLesseePayoutConfigRateLimit(2, 1 hours);

        UntronV3.PayoutConfig memory c1 = UntronV3.PayoutConfig({
            targetChainId: block.chainid, targetToken: DUMMY_USDT, beneficiary: address(0x1111)
        });
        UntronV3.PayoutConfig memory c2 = UntronV3.PayoutConfig({
            targetChainId: block.chainid, targetToken: DUMMY_USDT, beneficiary: address(0x2222)
        });
        UntronV3.PayoutConfig memory c3 = UntronV3.PayoutConfig({
            targetChainId: block.chainid, targetToken: DUMMY_USDT, beneficiary: address(0x3333)
        });

        uint256 deadline = block.timestamp + 1 days;

        untron.setPayoutConfigWithSig(leaseId, c1, deadline, _signPayoutConfigUpdate(lesseeKey, leaseId, c1, deadline));
        untron.setPayoutConfigWithSig(leaseId, c2, deadline, _signPayoutConfigUpdate(lesseeKey, leaseId, c2, deadline));

        vm.prank(lessee);
        vm.expectRevert(UntronV3.PayoutConfigRateLimitExceeded.selector);
        untron.setPayoutConfig(leaseId, block.chainid, DUMMY_USDT, address(0x9999));

        bytes memory sig3 = _signPayoutConfigUpdate(lesseeKey, leaseId, c3, deadline);
        vm.expectRevert(UntronV3.PayoutConfigRateLimitExceeded.selector);
        untron.setPayoutConfigWithSig(leaseId, c3, deadline, sig3);
    }

    function testSetPayoutConfigRateLimitDisabledAllowsUnlimitedUpdates() public {
        address lessee = address(0xBEEF);
        uint256 leaseId = _createLeaseForLessee(lessee);

        untron.setLesseePayoutConfigRateLimit(0, 0);

        vm.startPrank(lessee);
        untron.setPayoutConfig(leaseId, block.chainid, DUMMY_USDT, address(0x1111));
        untron.setPayoutConfig(leaseId, block.chainid, DUMMY_USDT, address(0x2222));
        untron.setPayoutConfig(leaseId, block.chainid, DUMMY_USDT, address(0x3333));
        vm.stopPrank();
    }

    function _createLeaseForLessee(address lessee) internal returns (uint256 leaseId) {
        bytes32 salt = keccak256(abi.encodePacked("salt", lessee));
        uint32 leaseFeePpm = 0;
        uint64 flatFee = 0;
        uint64 nukeableAfter = uint64(block.timestamp + 1 days);

        leaseId = untron.createLease(
            salt, lessee, nukeableAfter, leaseFeePpm, flatFee, block.chainid, DUMMY_USDT, address(0xB0B)
        );
    }

    function _signPayoutConfigUpdate(
        uint256 lesseeKey,
        uint256 leaseId,
        UntronV3.PayoutConfig memory config,
        uint256 deadline
    ) internal view returns (bytes memory signature) {
        uint256 nonce = untron.leaseNonces(leaseId);
        bytes32 structHash = keccak256(
            abi.encode(
                keccak256(
                    "PayoutConfigUpdate(uint256 leaseId,uint256 targetChainId,address targetToken,address beneficiary,uint256 nonce,uint256 deadline)"
                ),
                leaseId,
                config.targetChainId,
                config.targetToken,
                config.beneficiary,
                nonce,
                deadline
            )
        );

        bytes32 domainSeparator = keccak256(
            abi.encode(
                keccak256("EIP712Domain(string name,string version,uint256 chainId,address verifyingContract)"),
                keccak256(bytes("Untron")),
                keccak256(bytes("1")),
                block.chainid,
                address(untron)
            )
        );

        bytes32 digest = keccak256(abi.encodePacked("\x19\x01", domainSeparator, structHash));

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(lesseeKey, digest);
        signature = abi.encodePacked(r, s, v);
    }
}
