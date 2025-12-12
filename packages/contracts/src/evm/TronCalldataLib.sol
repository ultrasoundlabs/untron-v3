// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

/// @title TronCalldataLib
/// @notice Pure helpers for parsing recognized Tron calldata patterns.
library TronCalldataLib {
    // Errors duplicated from UntronV3 to preserve revert selectors.
    error NotATrc20Transfer();
    error TronInvalidTrc20DataLength();
    error TronInvalidCalldataLength();
    error NoEventChainTipInMulticall();

    // TRC-20 function selectors.
    bytes4 internal constant SELECTOR_TRANSFER = bytes4(keccak256("transfer(address,uint256)"));
    bytes4 internal constant SELECTOR_TRANSFER_FROM = bytes4(keccak256("transferFrom(address,address,uint256)"));

    /// @dev Convert an EVM address into Tron-style 21-byte address (0x41 prefix + 20-byte address).
    function evmToTronAddress(address a) internal pure returns (bytes21) {
        return bytes21((uint168(0x41) << 160) | uint168(uint160(a)));
    }

    function decodeIsEventChainTip(bytes memory data) internal pure returns (bytes32 tip) {
        uint256 dataEnd = data.length;
        if (dataEnd != 4 + 32) revert TronInvalidCalldataLength();
        assembly ("memory-safe") {
            tip := mload(add(data, 0x24)) // selector (4) + tip (32)
        }
    }

    /// @notice Decode isEventChainTip(bytes32) embedded within multicall(bytes[]).
    /// @param selectorIsEventChainTip The selector for isEventChainTip(bytes32).
    function decodeMulticallEventChainTip(bytes memory data, bytes4 selectorIsEventChainTip)
        internal
        pure
        returns (bytes32 tip)
    {
        uint256 dataEnd = data.length;
        if (dataEnd < 4 + 32) revert TronInvalidCalldataLength();

        // ABI decoding for: multicall(bytes[])
        // calldata = selector (4) | head (32: offset to bytes[]) | ...dynamic...
        uint256 arrayRel = _readU256(data, 4);
        uint256 arrayStart = 4 + arrayRel;
        if (arrayStart + 32 > dataEnd) revert TronInvalidCalldataLength();

        uint256 n = _readU256(data, arrayStart);
        uint256 offsetsStart = arrayStart + 32;
        uint256 offsetsEnd = offsetsStart + 32 * n;
        if (offsetsEnd > dataEnd) revert TronInvalidCalldataLength();

        for (uint256 i = 0; i < n; i++) {
            uint256 elementRel = _readU256(data, offsetsStart + 32 * i);
            // Element offsets are relative to `offsetsStart` (i.e., immediately after the length slot).
            uint256 elementStart = offsetsStart + elementRel;
            if (elementStart + 32 > dataEnd) revert TronInvalidCalldataLength();

            uint256 elementLen = _readU256(data, elementStart);
            uint256 elementDataStart = elementStart + 32;
            uint256 elementDataEnd = elementDataStart + elementLen;
            if (elementDataEnd > dataEnd) revert TronInvalidCalldataLength();

            if (elementLen < 4) continue;

            bytes4 innerSel;
            assembly ("memory-safe") {
                // `bytes4` values are left-aligned on the stack; loading the first word preserves
                // the selector in the high 4 bytes.
                innerSel := mload(add(data, add(0x20, elementDataStart)))
            }

            if (innerSel != selectorIsEventChainTip) continue;
            if (elementLen != 4 + 32) revert TronInvalidCalldataLength();

            tip = bytes32(_readU256(data, elementDataStart + 4));
            return tip;
        }

        revert NoEventChainTipInMulticall();
    }

    /// @dev Decode TRC-20 transfer / transferFrom calldata (first recognizable transfer).
    function decodeTrc20FromCalldata(bytes memory data, bytes21 senderTron)
        internal
        pure
        returns (bytes21 fromTron, bytes21 toTron, uint256 amount)
    {
        if (data.length < 4) revert TronInvalidCalldataLength();
        // forge-lint: disable-next-line(unsafe-typecast)
        bytes4 sig = bytes4(data);
        if (sig == SELECTOR_TRANSFER) {
            (toTron, amount) = decodeTrc20TransferArgs(data);
            fromTron = senderTron;
        } else if (sig == SELECTOR_TRANSFER_FROM) {
            (fromTron, toTron, amount) = decodeTrc20TransferFromArgs(data);
        } else {
            revert NotATrc20Transfer();
        }
    }

    function decodeTrc20TransferArgs(bytes memory data) internal pure returns (bytes21 toTron, uint256 amount) {
        uint256 dataEnd = data.length;
        if (dataEnd != 4 + 32 * 2) revert TronInvalidTrc20DataLength();
        bytes32 word1;
        bytes32 word2;
        assembly ("memory-safe") {
            word1 := mload(add(data, 0x24)) // 0x20 (data) + 4 (selector)
            word2 := mload(add(data, 0x44)) // 0x20 (data) + 36
        }
        address toAddr = address(uint160(uint256(word1)));
        toTron = evmToTronAddress(toAddr);
        amount = uint256(word2);
    }

    function decodeTrc20TransferFromArgs(bytes memory data)
        internal
        pure
        returns (bytes21 fromTron, bytes21 toTron, uint256 amount)
    {
        uint256 dataEnd = data.length;
        if (dataEnd != 4 + 32 * 3) revert TronInvalidTrc20DataLength();
        bytes32 w1;
        bytes32 w2;
        bytes32 w3;
        assembly ("memory-safe") {
            w1 := mload(add(data, 0x24)) // from
            w2 := mload(add(data, 0x44)) // to
            w3 := mload(add(data, 0x64)) // amount
        }
        address fromAddr = address(uint160(uint256(w1)));
        address toAddr2 = address(uint160(uint256(w2)));
        fromTron = evmToTronAddress(fromAddr);
        toTron = evmToTronAddress(toAddr2);
        amount = uint256(w3);
    }

    function _readDyn(bytes memory data, uint256 offset, uint256 limit)
        private
        pure
        returns (uint256 start, uint256 end, uint256 newCursor)
    {
        start = offset;
        uint256 len = _readU256(data, start);
        end = start + 32 + len;
        if (end > limit) revert TronInvalidCalldataLength();
        return (start, end, end);
    }

    function _readU256(bytes memory data, uint256 offset) private pure returns (uint256 v) {
        assembly ("memory-safe") {
            v := mload(add(data, add(0x20, offset)))
        }
    }
}
