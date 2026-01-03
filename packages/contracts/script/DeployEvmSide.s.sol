// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import "forge-std/Script.sol";
import "forge-std/StdJson.sol";
import "forge-std/console2.sol";

import {StatefulTronTxReader} from "../src/evm/StatefulTronTxReader.sol";
import {MockStatefulTronTxReader} from "../src/evm/mocks/MockStatefulTronTxReader.sol";
import {UntronV3} from "../src/evm/UntronV3.sol";
import {CCTPV2Bridger} from "../src/evm/bridgers/USDC/CCTPV2Bridger.sol";
import {USDT0Bridger} from "../src/evm/bridgers/USDT0/USDT0Bridger.sol";
import {UntronDeployer} from "./UntronDeployer.sol";

/// @notice Deploys all EVM-side Untron V3 contracts and wires bridgers into UntronV3.
/// @dev Env (required unless noted):
/// - PRIVATE_KEY
/// - CONTROLLER_ADDRESS (Tron-side UntronController in 20-byte EVM form)
/// - TRON_RECEIVER_IMPL (Tron-side UntronController.RECEIVER_IMPL() in 20-byte EVM form)
/// - TLC_CONFIG_PATH (optional; defaults to `<projectRoot>/script/tlc.json`) - source of SR epoch data
/// - TOKEN_MESSENGER_V2 (Circle CCTP V2 TokenMessengerV2 on this chain)
/// - USDC (USDC token on this chain)
/// - USDT0 (USDT0 token on this chain)
/// - OFT (LayerZero OFT module for USDT0 on this chain)
/// - USDT (optional; defaults to USDT0) - UntronV3 accounting token on this chain
/// - OWNER (optional; defaults to deployer) - final owner for UntronV3 + bridgers
/// - BRIDGER_CONFIG_PATH (optional; defaults to `<projectRoot>/script/bridgers.json`)
/// - UNTRON_CREATE2_PREFIX (optional; defaults to 0x41) - CREATE2 prefix used by UntronV3 to predict receiver addresses
/// - STATEFUL_TRON_TX_READER (optional; if set, uses existing and skips deploying StatefulTronTxReader)
/// - USE_MOCK_STATEFUL_TRON_TX_READER (optional; if true, deploys and uses MockStatefulTronTxReader)
/// - OUTPUT_PATH (optional; if set, writes a JSON with deployed addresses)
contract DeployEvmSideScript is UntronDeployer {
    using stdJson for string;

    struct EnvConfig {
        uint256 deployerPk;
        address deployer;
        address controllerAddress;
        address tronReceiverImpl;
        bytes1 untronCreate2Prefix;
        address tronTxReader;
        bool useMockTronTxReader;
        address tokenMessengerV2;
        address usdc;
        address usdt0;
        address oft;
        address usdt;
        address finalOwner;
    }

    struct TronReaderConfig {
        bytes20[27] srs;
        bytes20[27] witnessDelegatees;
    }

    struct BridgerConfig {
        uint256[] cctpChainIds;
        uint32[] circleDomains;
        uint256[] usdt0ChainIds;
        uint32[] eids;
    }

    struct Deployed {
        address tronTxReader;
        address untron;
        address cctpV2Bridger;
        address usdt0Bridger;
    }

    function _deployEvmSideNoBroadcast(
        EnvConfig memory env,
        BridgerConfig memory bridgers,
        TronReaderConfig memory readerCfg
    )
        internal
        returns (Deployed memory deployed, UntronV3 untron, CCTPV2Bridger cctpV2Bridger, USDT0Bridger usdt0Bridger)
    {
        address tronTxReaderAddr = env.tronTxReader;
        bool deployTronTxReader = tronTxReaderAddr == address(0);

        if (deployTronTxReader) {
            if (env.useMockTronTxReader) {
                tronTxReaderAddr = address(new MockStatefulTronTxReader());
            } else {
                tronTxReaderAddr = address(new StatefulTronTxReader(readerCfg.srs, readerCfg.witnessDelegatees));
            }
        }

        untron = _deployUntronV3WithCreate2Prefix(env.controllerAddress, env.untronCreate2Prefix, env.tronReceiverImpl);
        _setUntronTronReader(untron, tronTxReaderAddr);
        _setUntronUsdt(untron, env.usdt);

        cctpV2Bridger = _deployCctpV2Bridger(
            address(untron), env.tokenMessengerV2, env.usdc, bridgers.cctpChainIds, bridgers.circleDomains
        );
        usdt0Bridger = _deployUsdt0Bridger(address(untron), env.usdt0, env.oft, bridgers.usdt0ChainIds, bridgers.eids);

        _setBridgerRoutes(untron, env.usdc, bridgers.cctpChainIds, address(cctpV2Bridger));
        _setBridgerRoutes(untron, env.usdt0, bridgers.usdt0ChainIds, address(usdt0Bridger));
        _transferOwnershipsIfNeeded(env.deployer, env.finalOwner, untron, cctpV2Bridger, usdt0Bridger);

        deployed = Deployed({
            tronTxReader: tronTxReaderAddr,
            untron: address(untron),
            cctpV2Bridger: address(cctpV2Bridger),
            usdt0Bridger: address(usdt0Bridger)
        });
    }

    function _readEnv() internal view returns (EnvConfig memory env) {
        env.deployerPk = vm.envUint("PRIVATE_KEY");
        env.deployer = vm.addr(env.deployerPk);

        env.controllerAddress = vm.envAddress("CONTROLLER_ADDRESS");
        env.tronReceiverImpl = vm.envAddress("TRON_RECEIVER_IMPL");
        env.untronCreate2Prefix = bytes1(0x41);
        try vm.envUint("UNTRON_CREATE2_PREFIX") returns (uint256 v) {
            require(v <= type(uint8).max, "UNTRON_CREATE2_PREFIX overflow");
            // forge-lint: disable-next-line(unsafe-typecast)
            env.untronCreate2Prefix = bytes1(uint8(v));
        } catch {}
        try vm.envAddress("STATEFUL_TRON_TX_READER") returns (address v) {
            env.tronTxReader = v;
        } catch {}
        try vm.envBool("USE_MOCK_STATEFUL_TRON_TX_READER") returns (bool v) {
            env.useMockTronTxReader = v;
        } catch {}
        env.tokenMessengerV2 = vm.envAddress("TOKEN_MESSENGER_V2");
        env.usdc = vm.envAddress("USDC");
        env.usdt0 = vm.envAddress("USDT0");
        env.oft = vm.envAddress("OFT");
        env.usdt = _usdtOrUsdt0();
        env.finalOwner = _finalOwnerOrDeployer(env.deployer);

        require(env.controllerAddress != address(0), "CONTROLLER_ADDRESS is zero");
        require(env.tronReceiverImpl != address(0), "TRON_RECEIVER_IMPL is zero");
        require(env.tokenMessengerV2 != address(0), "TOKEN_MESSENGER_V2 is zero");
        require(env.usdc != address(0), "USDC is zero");
        require(env.usdt0 != address(0), "USDT0 is zero");
        require(env.oft != address(0), "OFT is zero");
        require(env.usdt != address(0), "USDT is zero");
        require(env.finalOwner != address(0), "OWNER is zero");
    }

    function _readTronReaderConfig() internal view returns (TronReaderConfig memory cfg) {
        string memory tlcJson = vm.readFile(_tlcConfigPath());

        address[] memory srsAddr = tlcJson.readAddressArray(".srs");
        address[] memory delAddr = tlcJson.readAddressArray(".witnessDelegatees");
        require(srsAddr.length == 27, "srs must have 27 items");
        require(delAddr.length == 27, "witnessDelegatees must have 27 items");

        for (uint256 i = 0; i < 27; i++) {
            cfg.srs[i] = bytes20(srsAddr[i]);
            cfg.witnessDelegatees[i] = bytes20(delAddr[i]);
        }
    }

    function _readBridgers() internal view returns (BridgerConfig memory cfg) {
        string memory bridgerJson = vm.readFile(_bridgerConfigPath());

        cfg.cctpChainIds = bridgerJson.readUintArray(".cctpV2.supportedChainIds");
        cfg.circleDomains = _toUint32Array(bridgerJson.readUintArray(".cctpV2.circleDomains"));

        cfg.usdt0ChainIds = bridgerJson.readUintArray(".usdt0.supportedChainIds");
        cfg.eids = _toUint32Array(bridgerJson.readUintArray(".usdt0.eids"));
    }

    function _writeOutputIfNeeded(EnvConfig memory env, Deployed memory deployed) internal {
        string memory outputPath = _outputPath();
        if (bytes(outputPath).length == 0) return;

        string memory json = vm.serializeAddress("contracts", "StatefulTronTxReader", deployed.tronTxReader);
        json = vm.serializeAddress("contracts", "UntronV3", deployed.untron);
        json = vm.serializeAddress("contracts", "CCTPV2Bridger", deployed.cctpV2Bridger);
        json = vm.serializeAddress("contracts", "USDT0Bridger", deployed.usdt0Bridger);
        json = vm.serializeAddress("contracts", "USDC", env.usdc);
        json = vm.serializeAddress("contracts", "USDT0", env.usdt0);
        json = vm.serializeAddress("contracts", "USDT", env.usdt);
        json = vm.serializeAddress("contracts", "TOKEN_MESSENGER_V2", env.tokenMessengerV2);
        json = vm.serializeAddress("contracts", "OFT", env.oft);
        json = vm.serializeAddress("contracts", "OWNER", env.finalOwner);
        json = vm.serializeAddress("contracts", "DEPLOYER", env.deployer);
        vm.writeJson(json, outputPath);
        console2.log("Wrote output JSON:", outputPath);
    }

    function run() external virtual {
        EnvConfig memory env = _readEnv();
        BridgerConfig memory bridgers = _readBridgers();

        TronReaderConfig memory readerCfg;
        if (!env.useMockTronTxReader && env.tronTxReader == address(0)) {
            readerCfg = _readTronReaderConfig();
        }

        vm.startBroadcast(env.deployerPk);
        (Deployed memory deployed,,,) = _deployEvmSideNoBroadcast(env, bridgers, readerCfg);

        vm.stopBroadcast();

        console2.log("Config tlc.json:", _tlcConfigPath());
        console2.log("Config bridgers.json:", _bridgerConfigPath());
        console2.log("Deployer:", env.deployer);
        console2.log("Final owner:", env.finalOwner);
        console2.log("CONTROLLER_ADDRESS:", env.controllerAddress);
        console2.log("TRON_RECEIVER_IMPL:", env.tronReceiverImpl);
        console2.log("USDT (accounting):", env.usdt);
        console2.log("StatefulTronTxReader:", deployed.tronTxReader);
        console2.log("Using mock tron reader:", env.useMockTronTxReader);
        console2.log("UntronV3:", deployed.untron);
        console2.log("CCTPV2Bridger:", deployed.cctpV2Bridger);
        console2.log("USDT0Bridger:", deployed.usdt0Bridger);
        console2.log("Configured CCTP routes:", bridgers.cctpChainIds.length);
        console2.log("Configured USDT0 routes:", bridgers.usdt0ChainIds.length);

        _writeOutputIfNeeded(env, deployed);
    }
}
