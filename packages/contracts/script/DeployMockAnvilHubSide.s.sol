// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import "forge-std/console2.sol";

import {DeployEvmSideScript} from "./DeployEvmSide.s.sol";
import {MockERC20} from "../src/evm/mocks/MockERC20.sol";
import {MockTokenMessengerV2} from "../src/evm/mocks/MockTokenMessengerV2.sol";
import {MockOFT} from "../src/evm/mocks/MockOFT.sol";

/// @notice Dev helper: deploys a fully-mocked EVM-side stack on a clean chain, wired to a real Tron-side controller.
/// @dev Expects `CONTROLLER_ADDRESS` and `TRON_RECEIVER_IMPL` from the Tron-side deployment output.
/// Env:
/// - PRIVATE_KEY (required)
/// - CONTROLLER_ADDRESS (required; Tron-side UntronController in 20-byte EVM form)
/// - TRON_RECEIVER_IMPL (required; Tron-side UntronController.RECEIVER_IMPL() in 20-byte EVM form)
/// - OWNER (optional; defaults to deployer)
/// - OUTPUT_PATH (optional; writes deployment JSON)
/// - MINT_TEST_TOKENS (optional; default true)
/// - MINT_USDC (optional; default 1_000_000e6)
/// - MINT_USDT0 (optional; default 1_000_000e6)
contract DeployMockAnvilHubSideScript is DeployEvmSideScript {
    function run() external override {
        uint256 deployerPk = vm.envUint("PRIVATE_KEY");
        address deployer = vm.addr(deployerPk);
        address finalOwner = _finalOwnerOrDeployer(deployer);

        address controller = vm.envAddress("CONTROLLER_ADDRESS");
        address receiverImpl = vm.envAddress("TRON_RECEIVER_IMPL");
        require(controller != address(0), "CONTROLLER_ADDRESS is zero");
        require(receiverImpl != address(0), "TRON_RECEIVER_IMPL is zero");

        vm.startBroadcast(deployerPk);

        (MockERC20 usdc, MockERC20 usdt0, address tokenMessengerV2, address oft) = _deployMocks();

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
        console2.log("CONTROLLER_ADDRESS:", controller);
        console2.log("TRON_RECEIVER_IMPL:", receiverImpl);
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

    function _deployMocks() internal returns (MockERC20 usdc, MockERC20 usdt0, address tokenMessengerV2, address oft) {
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
        env.untronCreate2Prefix = bytes1(0xff);
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
        // Include a synthetic "remote" chain id so mocked fills can exercise bridging paths without reverting.
        // `apps/research/src/scripts/generateBothSidesActivity.ts` uses `remoteChainId = hubChainId + 1000`.
        uint256 remoteChainId = block.chainid + 1000;

        bridgers.cctpChainIds = new uint256[](2);
        bridgers.cctpChainIds[0] = block.chainid;
        bridgers.cctpChainIds[1] = remoteChainId;
        bridgers.circleDomains = new uint32[](2);
        bridgers.circleDomains[0] = 0;
        bridgers.circleDomains[1] = 1;

        bridgers.usdt0ChainIds = new uint256[](2);
        bridgers.usdt0ChainIds[0] = block.chainid;
        bridgers.usdt0ChainIds[1] = remoteChainId;
        bridgers.eids = new uint32[](2);
        bridgers.eids[0] = 1;
        bridgers.eids[1] = 2;
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
