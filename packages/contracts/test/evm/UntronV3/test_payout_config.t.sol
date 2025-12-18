// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import {UntronV3} from "../../../src/evm/UntronV3.sol";

import {UntronV3TestBase} from "./UntronV3TestBase.t.sol";

contract UntronV3PayoutConfigTest is UntronV3TestBase {
    function testOnlyLesseeCanUpdatePayoutConfigDirectly() public {
        address lessee = address(0xBEEF);
        uint256 leaseId = _createLease(
            keccak256("salt_payout_direct"),
            lessee,
            uint64(block.timestamp + 1 days),
            0,
            0,
            block.chainid,
            address(_usdt),
            address(0xB0B)
        );

        vm.prank(address(0xCA11));
        vm.expectRevert(UntronV3.NotLessee.selector);
        _untron.setPayoutConfig(leaseId, block.chainid, address(_usdt), address(0x1111));
    }

    function testPayoutConfigWithSigHappyPathAndReplayProtection() public {
        uint256 lesseeKey = 0xBEEF;
        address lessee = vm.addr(lesseeKey);

        uint256 leaseId = _createLease(
            keccak256("salt_payout_sig"),
            lessee,
            uint64(block.timestamp + 1 days),
            0,
            0,
            block.chainid,
            address(_usdt),
            address(0xB0B)
        );

        UntronV3.PayoutConfig memory cfg = UntronV3.PayoutConfig({
            targetChainId: block.chainid, targetToken: address(_usdt), beneficiary: address(0x1111)
        });
        uint256 deadline = block.timestamp + 1 days;
        bytes memory sig = _signPayoutConfigUpdate(lesseeKey, leaseId, cfg, deadline);

        _untron.setPayoutConfigWithSig(leaseId, cfg, deadline, sig);
        assertEq(_untron.leaseNonces(leaseId), 1);

        // Replay with same signature should fail (nonce mismatch changes digest).
        vm.expectRevert(UntronV3.InvalidSignature.selector);
        _untron.setPayoutConfigWithSig(leaseId, cfg, deadline, sig);
    }

    function testPayoutConfigWithSigFailureModes() public {
        uint256 lesseeKey = 0xBEEF;
        address lessee = vm.addr(lesseeKey);
        uint256 leaseId = _createLease(
            keccak256("salt_payout_sig_fail"),
            lessee,
            uint64(block.timestamp + 1 days),
            0,
            0,
            block.chainid,
            address(_usdt),
            address(0xB0B)
        );

        UntronV3.PayoutConfig memory cfgLocalUsdt = UntronV3.PayoutConfig({
            targetChainId: block.chainid, targetToken: address(_usdt), beneficiary: address(0x1111)
        });

        // Wrong signer.
        uint256 wrongKey = 0xCAFE;
        bytes memory wrongSig = _signPayoutConfigUpdate(wrongKey, leaseId, cfgLocalUsdt, block.timestamp + 1 days);
        vm.expectRevert(UntronV3.InvalidSignature.selector);
        _untron.setPayoutConfigWithSig(leaseId, cfgLocalUsdt, block.timestamp + 1 days, wrongSig);

        // Expired deadline.
        vm.expectRevert(UntronV3.SignatureExpired.selector);
        _untron.setPayoutConfigWithSig(leaseId, cfgLocalUsdt, block.timestamp - 1, hex"");

        // Unroutable: missing swap rate.
        UntronV3.PayoutConfig memory cfgNeedsSwap = UntronV3.PayoutConfig({
            targetChainId: block.chainid, targetToken: address(_tokenX), beneficiary: address(0x2222)
        });
        vm.expectRevert(UntronV3.RateNotSet.selector);
        _untron.setPayoutConfigWithSig(leaseId, cfgNeedsSwap, block.timestamp + 1 days, hex"");

        // Unroutable: missing bridger.
        uint256 otherChainId = block.chainid + 1;
        UntronV3.PayoutConfig memory cfgNeedsBridge = UntronV3.PayoutConfig({
            targetChainId: otherChainId, targetToken: address(_usdt), beneficiary: address(0x3333)
        });
        vm.expectRevert(UntronV3.NoBridger.selector);
        _untron.setPayoutConfigWithSig(leaseId, cfgNeedsBridge, block.timestamp + 1 days, hex"");

        // Deprecated chain.
        _untron.setChainDeprecated(otherChainId, true);
        vm.expectRevert(UntronV3.ChainDeprecated.selector);
        _untron.setPayoutConfigWithSig(leaseId, cfgNeedsBridge, block.timestamp + 1 days, hex"");
    }

    function testPayoutConfigRateLimitConfigValidity() public {
        vm.expectRevert(UntronV3.PayoutConfigRateLimitConfigInvalid.selector);
        _untron.setLesseePayoutConfigRateLimit(0, 1);

        vm.expectRevert(UntronV3.PayoutConfigRateLimitConfigInvalid.selector);
        _untron.setLesseePayoutConfigRateLimit(1, 0);

        vm.expectRevert(UntronV3.PayoutConfigRateLimitConfigInvalid.selector);
        _untron.setLesseePayoutConfigRateLimit(uint256(type(uint32).max) + 1, 1);

        vm.expectRevert(UntronV3.PayoutConfigRateLimitConfigInvalid.selector);
        _untron.setLesseePayoutConfigRateLimit(1, uint256(type(uint32).max) + 1);
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
