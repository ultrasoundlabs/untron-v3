// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import "forge-std/console2.sol";

import {UntronScriptBase} from "./UntronScriptBase.sol";
import {UntronController} from "../src/tron/UntronController.sol";
import {MockERC20} from "../src/evm/mocks/MockERC20.sol";

/// @notice Dev helper: deploys a minimal Tron-side stack on a clean chain (e.g. anvil).
/// @dev This is intended for local multi-chain/indexer testing where a second anvil instance stands in for Tron.
/// Env:
/// - PRIVATE_KEY (required)
/// - OWNER (optional; defaults to deployer) - UntronController owner (must be deployer for this script, then transferred)
/// - EXECUTOR (optional; defaults to deployer)
/// - LP (optional; defaults to deployer)
/// - TRON_CREATE2_PREFIX (optional; default 0x41; use 0xff to match EVM/anvil CREATE2 behavior)
/// - MINT_CONTROLLER_USDT (optional; default 0) - mint USDT to controller for quick pulls/bridges
/// - MINT_LP_USDT (optional; default 1_000_000e6) - mint USDT to LP for swap liquidity testing
/// - OUTPUT_PATH (optional; writes deployment JSON)
contract DeployMockAnvilControllerSideScript is UntronScriptBase {
    function run() external {
        uint256 deployerPk = vm.envUint("PRIVATE_KEY");
        address deployer = vm.addr(deployerPk);

        address finalOwner = _finalOwnerOrDeployer(deployer);

        address executor = deployer;
        try vm.envAddress("EXECUTOR") returns (address v) {
            executor = v;
        } catch {}

        address lp = deployer;
        try vm.envAddress("LP") returns (address v) {
            lp = v;
        } catch {}

        bytes1 create2Prefix = bytes1(0x41);
        try vm.envUint("TRON_CREATE2_PREFIX") returns (uint256 v) {
            require(v <= type(uint8).max, "TRON_CREATE2_PREFIX overflow");
            // forge-lint: disable-next-line(unsafe-typecast)
            create2Prefix = bytes1(uint8(v));
        } catch {}

        uint256 mintControllerUsdt = 0;
        try vm.envUint("MINT_CONTROLLER_USDT") returns (uint256 v) {
            mintControllerUsdt = v;
        } catch {}

        uint256 mintLpUsdt = 1_000_000e6;
        try vm.envUint("MINT_LP_USDT") returns (uint256 v) {
            mintLpUsdt = v;
        } catch {}

        vm.startBroadcast(deployerPk);

        UntronController controller = new UntronController(create2Prefix);
        MockERC20 usdt = new MockERC20("Mock Tron USDT", "tUSDT", 6);

        controller.setExecutor(executor);
        controller.setLp(lp);
        controller.setUsdt(address(usdt));

        if (mintControllerUsdt != 0) {
            usdt.mint(address(controller), mintControllerUsdt);
        }
        if (mintLpUsdt != 0) {
            usdt.mint(lp, mintLpUsdt);
        }

        if (finalOwner != deployer) {
            controller.setOwner(finalOwner);
        }

        vm.stopBroadcast();

        console2.log("Deployer:", deployer);
        console2.log("Final owner:", finalOwner);
        console2.log("Executor:", executor);
        console2.log("LP:", lp);
        console2.log("TRON_CREATE2_PREFIX:", uint256(uint8(create2Prefix)));
        console2.log("UntronController:", address(controller));
        console2.log("RECEIVER_IMPL:", controller.RECEIVER_IMPL());
        console2.log("Mock Tron USDT:", address(usdt));

        _writeOutputIfNeeded(
            deployer,
            finalOwner,
            executor,
            lp,
            create2Prefix,
            address(controller),
            controller.RECEIVER_IMPL(),
            address(usdt)
        );
    }

    function _writeOutputIfNeeded(
        address deployer,
        address finalOwner,
        address executor,
        address lp,
        bytes1 create2Prefix,
        address controller,
        address receiverImpl,
        address usdt
    ) internal {
        string memory outputPath = _outputPath();
        if (bytes(outputPath).length == 0) return;

        // Keep output JSON "flat" (single root object), because forge's `vm.serialize*` returns the object for a
        // given key, not a merged root across multiple object keys.
        string memory json = vm.serializeAddress("contracts", "UntronController", controller);
        json = vm.serializeAddress("contracts", "TRON_RECEIVER_IMPL", receiverImpl);
        json = vm.serializeAddress("contracts", "USDT", usdt);
        json = vm.serializeAddress("contracts", "OWNER", finalOwner);
        json = vm.serializeAddress("contracts", "EXECUTOR", executor);
        json = vm.serializeAddress("contracts", "LP", lp);
        json = vm.serializeAddress("contracts", "DEPLOYER", deployer);
        json = vm.serializeUint("contracts", "TRON_CREATE2_PREFIX", uint256(uint8(create2Prefix)));
        vm.writeJson(json, outputPath);
        console2.log("Wrote output JSON:", outputPath);
    }
}
