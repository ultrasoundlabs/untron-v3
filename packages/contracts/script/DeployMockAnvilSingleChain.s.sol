// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import "forge-std/console2.sol";

import {DeployEvmSideScript} from "./DeployEvmSide.s.sol";
import {MockERC20} from "../src/evm/mocks/MockERC20.sol";
import {MockTokenMessengerV2} from "../src/evm/mocks/MockTokenMessengerV2.sol";
import {MockOFT} from "../src/evm/mocks/MockOFT.sol";

contract _DummyTronSide {}

/// @notice Dev helper: deploys a fully-mocked EVM-side stack on a clean chain (e.g. anvil).
/// @dev Deploys mock tokens + mock bridge dependencies, then reuses DeployEvmSide's wiring via `_deployEvmSideNoBroadcast`.
/// Env:
/// - PRIVATE_KEY (required)
/// - OWNER (optional; defaults to deployer)
/// - OUTPUT_PATH (optional; writes deployment JSON)
/// - MINT_TEST_TOKENS (optional; default true) - mints some USDC/USDT0 to deployer for quick testing
/// - MINT_USDC (optional; default 1_000_000e6)
/// - MINT_USDT0 (optional; default 1_000_000e6)
contract DeployMockAnvilSingleChainScript is DeployEvmSideScript {
    function run() external override {
        uint256 deployerPk = vm.envUint("PRIVATE_KEY");
        address deployer = vm.addr(deployerPk);
        address finalOwner = _finalOwnerOrDeployer(deployer);

        vm.startBroadcast(deployerPk);

        (
            address controller,
            address receiverImpl,
            MockERC20 usdc,
            MockERC20 usdt0,
            address tokenMessengerV2,
            address oft
        ) = _deployMocks();

        EnvConfig memory env = _makeEnv(
            deployerPk,
            deployer,
            finalOwner,
            controller,
            receiverImpl,
            tokenMessengerV2,
            address(usdc),
            address(usdt0),
            oft
        );
        BridgerConfig memory bridgers = _defaultBridgerConfig();
        TronReaderConfig memory readerCfg; // unused for mock reader

        (Deployed memory deployed,,,) = _deployEvmSideNoBroadcast(env, bridgers, readerCfg);

        _mintIfConfigured(usdc, usdt0, deployer);

        vm.stopBroadcast();

        console2.log("Deployer:", deployer);
        console2.log("Final owner:", finalOwner);
        console2.log("Mock USDC:", address(usdc));
        console2.log("Mock USDT0:", address(usdt0));
        console2.log("Mock TokenMessengerV2:", tokenMessengerV2);
        console2.log("Mock OFT:", oft);
        console2.log("Mock tron reader deployed:", deployed.tronTxReader);
        console2.log("UntronV3:", deployed.untron);
        console2.log("CCTPV2Bridger:", deployed.cctpV2Bridger);
        console2.log("USDT0Bridger:", deployed.usdt0Bridger);

        _writeOutputIfNeeded(env, deployed);
    }

    function _deployMocks()
        internal
        returns (
            address controller,
            address receiverImpl,
            MockERC20 usdc,
            MockERC20 usdt0,
            address tokenMessengerV2,
            address oft
        )
    {
        controller = address(new _DummyTronSide());
        receiverImpl = address(new _DummyTronSide());

        usdc = new MockERC20("Mock USDC", "mUSDC", 6);
        usdt0 = new MockERC20("Mock USDT0", "mUSDT0", 6);

        tokenMessengerV2 = address(new MockTokenMessengerV2());
        oft = address(new MockOFT(address(usdt0), address(0)));
    }

    function _makeEnv(
        uint256 deployerPk,
        address deployer,
        address finalOwner,
        address controller,
        address receiverImpl,
        address tokenMessengerV2,
        address usdc,
        address usdt0,
        address oft
    ) internal pure returns (EnvConfig memory env) {
        env.deployerPk = deployerPk;
        env.deployer = deployer;
        env.controllerAddress = controller;
        env.tronReceiverImpl = receiverImpl;
        env.tronTxReader = address(0);
        env.useMockTronTxReader = true;
        env.tokenMessengerV2 = tokenMessengerV2;
        env.usdc = usdc;
        env.usdt0 = usdt0;
        env.oft = oft;
        env.usdt = usdt0;
        env.finalOwner = finalOwner;
    }

    function _defaultBridgerConfig() internal view returns (BridgerConfig memory bridgers) {
        bridgers.cctpChainIds = new uint256[](1);
        bridgers.cctpChainIds[0] = block.chainid;
        bridgers.circleDomains = new uint32[](1);
        bridgers.circleDomains[0] = 0;

        bridgers.usdt0ChainIds = new uint256[](1);
        bridgers.usdt0ChainIds[0] = block.chainid;
        bridgers.eids = new uint32[](1);
        bridgers.eids[0] = 1;
    }

    function _mintIfConfigured(MockERC20 usdc, MockERC20 usdt0, address deployer) internal {
        bool mint = true;
        try vm.envBool("MINT_TEST_TOKENS") returns (bool v) {
            mint = v;
        } catch {}
        if (!mint) return;

        uint256 mintUsdc = 1_000_000e6;
        uint256 mintUsdt0 = 1_000_000e6;
        try vm.envUint("MINT_USDC") returns (uint256 v) {
            mintUsdc = v;
        } catch {}
        try vm.envUint("MINT_USDT0") returns (uint256 v) {
            mintUsdt0 = v;
        } catch {}

        usdc.mint(deployer, mintUsdc);
        usdt0.mint(deployer, mintUsdt0);
    }
}
