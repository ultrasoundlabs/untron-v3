// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {Test} from "forge-std/Test.sol";
import {UntronV3} from "../../../src/evm/UntronV3.sol";

contract UntronV3PayoutConfigHarness is UntronV3 {
    constructor(address controllerAddress, bytes1 create2Prefix, address receiverImplOverride)
        UntronV3(controllerAddress, create2Prefix, receiverImplOverride)
    {}
}

contract UntronV3PayoutConfigRateLimitTest is Test {
    UntronV3PayoutConfigHarness internal _untron;

    address internal constant _DUMMY_USDT = address(0x1000);
    address internal constant _CONTROLLER = address(0xCAFE);
    address internal constant _RECEIVER_IMPL_OVERRIDE = address(0xBEEF);

    function setUp() public {
        _untron = new UntronV3PayoutConfigHarness(_CONTROLLER, 0xff, _RECEIVER_IMPL_OVERRIDE);
        _untron.setUsdt(_DUMMY_USDT);
        _untron.setRealtor(address(this), true);
    }

    function testSetPayoutConfigIsRateLimited() public {
        address lessee = address(0xBEEF);
        (uint256 leaseId,) = _createLeaseForLessee(lessee);

        _untron.setLesseePayoutConfigRateLimit(2, 1 hours);

        vm.startPrank(lessee);
        _untron.setPayoutConfig(leaseId, block.chainid, _DUMMY_USDT, address(0x1111));
        _untron.setPayoutConfig(leaseId, block.chainid, _DUMMY_USDT, address(0x2222));
        vm.expectRevert(UntronV3.PayoutConfigRateLimitExceeded.selector);
        _untron.setPayoutConfig(leaseId, block.chainid, _DUMMY_USDT, address(0x3333));
        vm.stopPrank();

        vm.warp(block.timestamp + 1 hours);

        vm.prank(lessee);
        _untron.setPayoutConfig(leaseId, block.chainid, _DUMMY_USDT, address(0x4444));
    }

    function testSetPayoutConfigWithSigIsRateLimited() public {
        uint256 lesseeKey = 0xBEEF;
        address lessee = vm.addr(lesseeKey);
        (uint256 leaseId,) = _createLeaseForLessee(lessee);

        _untron.setLesseePayoutConfigRateLimit(2, 1 hours);

        UntronV3.PayoutConfig memory c1 = UntronV3.PayoutConfig({
            targetChainId: block.chainid, targetToken: _DUMMY_USDT, beneficiary: address(0x1111)
        });
        UntronV3.PayoutConfig memory c2 = UntronV3.PayoutConfig({
            targetChainId: block.chainid, targetToken: _DUMMY_USDT, beneficiary: address(0x2222)
        });
        UntronV3.PayoutConfig memory c3 = UntronV3.PayoutConfig({
            targetChainId: block.chainid, targetToken: _DUMMY_USDT, beneficiary: address(0x3333)
        });

        uint256 deadline = block.timestamp + 1 days;

        _untron.setPayoutConfigWithSig(leaseId, c1, deadline, _signPayoutConfigUpdate(lesseeKey, leaseId, c1, deadline));
        _untron.setPayoutConfigWithSig(leaseId, c2, deadline, _signPayoutConfigUpdate(lesseeKey, leaseId, c2, deadline));

        vm.prank(lessee);
        vm.expectRevert(UntronV3.PayoutConfigRateLimitExceeded.selector);
        _untron.setPayoutConfig(leaseId, block.chainid, _DUMMY_USDT, address(0x9999));

        bytes memory sig3 = _signPayoutConfigUpdate(lesseeKey, leaseId, c3, deadline);
        vm.expectRevert(UntronV3.PayoutConfigRateLimitExceeded.selector);
        _untron.setPayoutConfigWithSig(leaseId, c3, deadline, sig3);
    }

    function testSetPayoutConfigRateLimitDisabledAllowsUnlimitedUpdates() public {
        address lessee = address(0xBEEF);
        (uint256 leaseId,) = _createLeaseForLessee(lessee);

        _untron.setLesseePayoutConfigRateLimit(0, 0);

        vm.startPrank(lessee);
        _untron.setPayoutConfig(leaseId, block.chainid, _DUMMY_USDT, address(0x1111));
        _untron.setPayoutConfig(leaseId, block.chainid, _DUMMY_USDT, address(0x2222));
        _untron.setPayoutConfig(leaseId, block.chainid, _DUMMY_USDT, address(0x3333));
        vm.stopPrank();
    }

    function _createLeaseForLessee(address lessee) internal returns (uint256 leaseId, uint256 leaseNumber) {
        bytes32 salt = keccak256(abi.encodePacked("salt", lessee));
        uint32 leaseFeePpm = 0;
        uint64 flatFee = 0;
        uint64 nukeableAfter = uint64(block.timestamp + 1 days);
        (leaseId, leaseNumber) = _untron.createLease(
            salt, lessee, nukeableAfter, leaseFeePpm, flatFee, block.chainid, _DUMMY_USDT, address(0xB0B)
        );
    }

    function _signPayoutConfigUpdate(
        uint256 lesseeKey,
        uint256 leaseId,
        UntronV3.PayoutConfig memory config,
        uint256 deadline
    ) internal view returns (bytes memory signature) {
        uint256 nonce = _untron.leaseNonces(leaseId);
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
                address(_untron)
            )
        );

        bytes32 digest = keccak256(abi.encodePacked("\x19\x01", domainSeparator, structHash));

        (uint8 v, bytes32 r, bytes32 s) = vm.sign(lesseeKey, digest);
        signature = abi.encodePacked(r, s, v);
    }
}
