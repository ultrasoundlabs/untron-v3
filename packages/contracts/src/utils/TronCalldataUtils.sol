// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

/// @title TronCalldataUtils
/// @notice Gas-optimized pure helpers for parsing a small set of recognized Tron/EVM ABI calldata patterns.
/// @dev
/// This library intentionally does not perform general-purpose ABI decoding. It only recognizes:
/// - TRC-20 `transfer(address,uint256)` and `transferFrom(address,address,uint256)`
/// - `isEventChainTip(bytes32)` either as a direct call or embedded as an element within `multicall(bytes[])`
///
/// Addresses are returned in a Tron "raw" 21-byte format: `0x41 || bytes20(evmAddress)`.
/// @author Ultrasound Labs
library TronCalldataUtils {
    /// @dev Errors duplicated from UntronV3 to preserve revert selectors.
    error NotATrc20Transfer();
    error TronInvalidTrc20DataLength();
    error TronInvalidCalldataLength();
    error NoEventChainTipInMulticall();

    /* solhint-disable gas-small-strings */
    // this is a false alarm. keccak over string literal is evaluated at compile time.

    /// @dev TRC-20 function selectors.
    bytes4 internal constant SELECTOR_TRANSFER = bytes4(keccak256("transfer(address,uint256)"));
    bytes4 internal constant SELECTOR_TRANSFER_FROM = bytes4(keccak256("transferFrom(address,address,uint256)"));

    /* solhint-enable gas-small-strings */

    /// @notice Convert an EVM `address` into a Tron-style 21-byte "raw address".
    /// @dev The returned value is `0x41 || bytes20(a)` (no base58check encoding/decoding).
    /// @param a The EVM address to convert.
    /// @return tron The Tron raw address (21 bytes, `0x41` prefix + 20-byte EVM address).
    function evmToTronAddress(address a) internal pure returns (bytes21 tron) {
        // Layout: [0x41][20-byte address]
        tron = bytes21((uint168(0x41) << 160) | uint168(uint160(a)));
    }

    /// @notice Decode calldata for `isEventChainTip(bytes32)` and return the `tip` argument.
    /// @dev Expects `data` to be the full ABI-encoded calldata: `selector (4) || tip (32)`.
    /// @param data ABI-encoded calldata for `isEventChainTip(bytes32)`.
    /// @return tip The `bytes32` argument passed to `isEventChainTip`.
    function decodeIsEventChainTip(bytes memory data) internal pure returns (bytes32 tip) {
        uint256 dataEnd = data.length;
        if (dataEnd != 4 + 32) revert TronInvalidCalldataLength();
        // solhint-disable-next-line no-inline-assembly
        assembly ("memory-safe") {
            // `data` points to the bytes object; skip the 32-byte length slot then skip 4-byte selector.
            tip := mload(add(data, 0x24))
        }
    }

    /// @notice Decode isEventChainTip(bytes32) embedded within multicall(bytes[]).
    /// @dev
    /// Expects `data` to be full ABI-encoded calldata for `multicall(bytes[])`:
    /// - `data[0:4]`   = multicall selector (ignored by this function)
    /// - `data[4:36]`  = offset to `bytes[]` tail (relative to byte 4, i.e. start of arguments)
    ///
    /// For `bytes[]`, Solidity ABI encodes per-element offsets relative to the start of the offsets table
    /// (i.e. `arrayStart + 32`), not relative to `arrayStart`.
    ///
    /// Scans each element of the `bytes[]` and returns the first one whose selector equals
    /// `selectorIsEventChainTip`. The element must be exactly `4 + 32` bytes long.
    /// @param data ABI-encoded calldata for `multicall(bytes[])`.
    /// @param selectorIsEventChainTip The selector for isEventChainTip(bytes32).
    /// @return tip The `bytes32` tip embedded in the first matching `isEventChainTip(bytes32)` element.
    function decodeMulticallEventChainTip(bytes memory data, bytes4 selectorIsEventChainTip)
        internal
        pure
        returns (bytes32 tip)
    {
        uint256 dataEnd = data.length;
        if (dataEnd < 4 + 32) revert TronInvalidCalldataLength();

        // ABI decoding for: multicall(bytes[])
        // calldata = selector (4) | head (32: offset to bytes[]) | ...dynamic...
        // The head offset is relative to the start of the arguments area (i.e. immediately after selector).
        uint256 arrayStart = 4 + _readU256(data, 4);
        if (arrayStart + 32 > dataEnd) revert TronInvalidCalldataLength();

        uint256 n = _readU256(data, arrayStart);
        uint256 offsetsStart = arrayStart + 32;
        if (offsetsStart + 32 * n > dataEnd) revert TronInvalidCalldataLength();

        for (uint256 i = 0; i < n; ++i) {
            (bool ok, bytes32 tip_) =
                _tryDecodeEventChainTipElement(data, offsetsStart, dataEnd, i, selectorIsEventChainTip);
            if (ok) return tip_;
        }

        revert NoEventChainTipInMulticall();
    }

    /// @notice Tries to decode an event chain tip element from the given data.
    /// @param data The calldata.
    /// @param offsetsStart The start of the offsets array.
    /// @param dataEnd The end of the calldata.
    /// @param i The index of the element to decode.
    /// @param selectorIsEventChainTip The selector of the event chain tip.
    /// @return ok Whether the decoding was successful.
    /// @return tip The decoded event chain tip.
    function _tryDecodeEventChainTipElement(
        bytes memory data,
        uint256 offsetsStart,
        uint256 dataEnd,
        uint256 i,
        bytes4 selectorIsEventChainTip
    ) private pure returns (bool ok, bytes32 tip) {
        uint256 elementRel = _readU256(data, offsetsStart + 32 * i);
        // Element offsets are relative to `offsetsStart` (i.e., immediately after the length slot).
        uint256 elementStart = offsetsStart + elementRel;
        if (elementStart + 32 > dataEnd) revert TronInvalidCalldataLength();

        uint256 elementLen = _readU256(data, elementStart);
        if (elementLen < 4) return (false, bytes32(0));

        uint256 elementDataStart = elementStart + 32;
        uint256 elementDataEnd = elementDataStart + elementLen;
        if (elementDataEnd > dataEnd) revert TronInvalidCalldataLength();

        bytes4 innerSel;
        // solhint-disable-next-line no-inline-assembly
        assembly ("memory-safe") {
            // `bytes4` values are left-aligned on the stack; loading the first word preserves
            // the selector in the high 4 bytes.
            innerSel := mload(add(data, add(0x20, elementDataStart)))
        }
        if (innerSel != selectorIsEventChainTip) return (false, bytes32(0));
        if (elementLen != 4 + 32) revert TronInvalidCalldataLength();

        tip = bytes32(_readU256(data, elementDataStart + 4));
        return (true, tip);
    }

    /// @notice Decode TRC-20 `transfer` / `transferFrom` calldata.
    /// @dev
    /// - For `transfer(address,uint256)`, the `fromTron` value is taken from `senderTron` since the
    ///   calldata does not carry the sender.
    /// - For `transferFrom(address,address,uint256)`, both `fromTron` and `toTron` are taken from calldata.
    /// @param data ABI-encoded calldata for a TRC-20 transfer function.
    /// @param senderTron The Tron raw address (`0x41 || 20 bytes`) representing the caller/sender.
    /// @return fromTron The decoded Tron raw "from" address.
    /// @return toTron The decoded Tron raw "to" address.
    /// @return amount The decoded transfer amount.
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

    /// @notice Decode TRC-20 `transfer(address,uint256)` calldata arguments.
    /// @dev Expects exact calldata length: `4 + 32*2`.
    /// @param data ABI-encoded calldata for `transfer(address,uint256)`.
    /// @return toTron The Tron raw recipient address (`0x41 || 20 bytes`).
    /// @return amount The transfer amount.
    function decodeTrc20TransferArgs(bytes memory data) internal pure returns (bytes21 toTron, uint256 amount) {
        uint256 dataEnd = data.length;
        if (dataEnd != 4 + 32 * 2) revert TronInvalidTrc20DataLength();
        bytes32 word1;
        bytes32 word2;
        // solhint-disable-next-line no-inline-assembly
        assembly ("memory-safe") {
            word1 := mload(add(data, 0x24)) // 0x20 (data) + 4 (selector)
            word2 := mload(add(data, 0x44)) // 0x20 (data) + 36
        }
        address toAddr = address(uint160(uint256(word1)));
        toTron = evmToTronAddress(toAddr);
        amount = uint256(word2);
    }

    /// @notice Decode TRC-20 `transferFrom(address,address,uint256)` calldata arguments.
    /// @dev Expects exact calldata length: `4 + 32*3`.
    /// @param data ABI-encoded calldata for `transferFrom(address,address,uint256)`.
    /// @return fromTron The Tron raw source address (`0x41 || 20 bytes`).
    /// @return toTron The Tron raw destination address (`0x41 || 20 bytes`).
    /// @return amount The transfer amount.
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
        // solhint-disable-next-line no-inline-assembly
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

    /// @notice Read a dynamic `bytes`-encoded region (`len || data`) from `data`.
    /// @dev
    /// This helper treats the region starting at `offset` as an ABI-encoded `bytes`:
    /// - `data[offset : offset+32]` = length (uint256)
    /// - `data[offset+32 : offset+32+len]` = bytes payload
    ///
    /// The returned `start`/`end` are byte offsets into `data`, and `newCursor` equals `end`.
    /// Callers must ensure `offset` itself is meaningful for their layout; this function only
    /// bounds-checks against `limit`.
    /// @param data The byte array to read from.
    /// @param offset The byte offset where the dynamic region begins (points at the length word).
    /// @param limit The maximum allowed end offset (typically `data.length`).
    /// @return start The start offset (equals `offset`).
    /// @return end The first offset after the region (`offset + 32 + len`).
    /// @return newCursor Cursor advanced to `end` (useful for sequential parsing).
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

    /// @notice Read a 32-byte word from `data` at the given byte `offset`.
    /// @dev This performs an unchecked `mload`; callers should ensure `offset + 32 <= data.length`.
    /// @param data The byte array to read from.
    /// @param offset The byte offset into `data` where the word begins.
    /// @return v The loaded 32-byte word interpreted as `uint256`.
    function _readU256(bytes memory data, uint256 offset) private pure returns (uint256 v) {
        // solhint-disable-next-line no-inline-assembly
        assembly ("memory-safe") {
            v := mload(add(data, add(0x20, offset)))
        }
    }
}
