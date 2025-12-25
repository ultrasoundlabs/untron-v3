// SPDX-License-Identifier: MIT
pragma solidity ^0.8.27;

import "forge-std/Script.sol";
import "forge-std/StdJson.sol";
import "forge-std/console2.sol";

import {TronLightClient} from "../src/evm/TronLightClient.sol";
import {TronTxReader} from "../src/evm/TronTxReader.sol";
import {UntronV3} from "../src/evm/UntronV3.sol";
import {CCTPV2Bridger} from "../src/evm/bridgers/USDC/CCTPV2Bridger.sol";
import {USDT0Bridger} from "../src/evm/bridgers/USDT0/USDT0Bridger.sol";
import {UntronDeployer} from "./UntronDeployer.sol";

/// @notice Deploys all EVM-side Untron V3 contracts and wires bridgers into UntronV3.
/// @dev Env (required unless noted):
/// - PRIVATE_KEY
/// - CONTROLLER_ADDRESS (Tron-side UntronController in 20-byte EVM form)
/// - TOKEN_MESSENGER_V2 (Circle CCTP V2 TokenMessengerV2 on this chain)
/// - USDC (USDC token on this chain)
/// - USDT0 (USDT0 token on this chain)
/// - OFT (LayerZero OFT module for USDT0 on this chain)
/// - USDT (optional; defaults to USDT0) - UntronV3 accounting token on this chain
/// - OWNER (optional; defaults to deployer) - final owner for UntronV3 + bridgers
/// - TLC_CONFIG_PATH (optional; defaults to `<projectRoot>/script/tlc.json`)
/// - BRIDGER_CONFIG_PATH (optional; defaults to `<projectRoot>/script/bridgers.json`)
/// - BLOCK_RANGE_PROVER (optional; overrides `tlc.json` `.blockRangeProver`)
/// - SR_DATA_HASH (optional; overrides `tlc.json` `.srDataHash`)
/// - OUTPUT_PATH (optional; if set, writes a JSON with deployed addresses)
contract DeployEvmSideScript is UntronDeployer {
    using stdJson for string;

    struct EnvConfig {
        uint256 deployerPk;
        address deployer;
        address controllerAddress;
        address tokenMessengerV2;
        address usdc;
        address usdt0;
        address oft;
        address usdt;
        address finalOwner;
    }

    struct TlcConfig {
        address proverAddr;
        bytes32 initialBlockHash;
        bytes32 initialTxTrieRoot;
        uint32 initialTimestamp;
        bytes20[27] srs;
        bytes20[27] witnessDelegatees;
        bytes32 srDataHash;
    }

    struct BridgerConfig {
        uint256[] cctpChainIds;
        uint32[] circleDomains;
        uint256[] usdt0ChainIds;
        uint32[] eids;
    }

    struct Deployed {
        address tronLightClient;
        address tronReader;
        address untron;
        address cctpV2Bridger;
        address usdt0Bridger;
    }

    function _readEnv() internal view returns (EnvConfig memory env) {
        env.deployerPk = vm.envUint("PRIVATE_KEY");
        env.deployer = vm.addr(env.deployerPk);

        env.controllerAddress = vm.envAddress("CONTROLLER_ADDRESS");
        env.tokenMessengerV2 = vm.envAddress("TOKEN_MESSENGER_V2");
        env.usdc = vm.envAddress("USDC");
        env.usdt0 = vm.envAddress("USDT0");
        env.oft = vm.envAddress("OFT");
        env.usdt = _usdtOrUsdt0();
        env.finalOwner = _finalOwnerOrDeployer(env.deployer);

        require(env.controllerAddress != address(0), "CONTROLLER_ADDRESS is zero");
        require(env.tokenMessengerV2 != address(0), "TOKEN_MESSENGER_V2 is zero");
        require(env.usdc != address(0), "USDC is zero");
        require(env.usdt0 != address(0), "USDT0 is zero");
        require(env.oft != address(0), "OFT is zero");
        require(env.usdt != address(0), "USDT is zero");
        require(env.finalOwner != address(0), "OWNER is zero");
    }

    function _readTlc() internal view returns (TlcConfig memory cfg) {
        string memory tlcJson = vm.readFile(_tlcConfigPath());

        cfg.proverAddr = _overrideAddress("BLOCK_RANGE_PROVER", tlcJson.readAddress(".blockRangeProver"));
        cfg.initialBlockHash = tlcJson.readBytes32(".initialBlockHash"); // Tron blockId (height||tail)
        cfg.initialTxTrieRoot = tlcJson.readBytes32(".initialTxTrieRoot");

        uint256 ts = tlcJson.readUint(".initialTimestamp"); // seconds
        require(ts <= type(uint32).max, "initialTimestamp > uint32");
        // forge-lint: disable-next-line(unsafe-typecast)
        cfg.initialTimestamp = uint32(ts);

        cfg.srDataHash = _overrideBytes32("SR_DATA_HASH", tlcJson.readBytes32(".srDataHash"));

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

    function _writeOutputIfNeeded(EnvConfig memory env, TlcConfig memory tlc, Deployed memory deployed) internal {
        string memory outputPath = _outputPath();
        if (bytes(outputPath).length == 0) return;

        string memory json = vm.serializeAddress("contracts", "TronLightClient", deployed.tronLightClient);
        json = vm.serializeAddress("contracts", "TronTxReader", deployed.tronReader);
        json = vm.serializeAddress("contracts", "UntronV3", deployed.untron);
        json = vm.serializeAddress("contracts", "CCTPV2Bridger", deployed.cctpV2Bridger);
        json = vm.serializeAddress("contracts", "USDT0Bridger", deployed.usdt0Bridger);
        json = vm.serializeAddress("contracts", "USDC", env.usdc);
        json = vm.serializeAddress("contracts", "USDT0", env.usdt0);
        json = vm.serializeAddress("contracts", "USDT", env.usdt);
        json = vm.serializeAddress("contracts", "TOKEN_MESSENGER_V2", env.tokenMessengerV2);
        json = vm.serializeAddress("contracts", "OFT", env.oft);
        json = vm.serializeAddress("contracts", "BLOCK_RANGE_PROVER", tlc.proverAddr);
        json = vm.serializeAddress("contracts", "OWNER", env.finalOwner);
        json = vm.serializeAddress("contracts", "DEPLOYER", env.deployer);
        vm.writeJson(json, outputPath);
        console2.log("Wrote output JSON:", outputPath);
    }

    function run() external {
        EnvConfig memory env = _readEnv();
        TlcConfig memory tlc = _readTlc();
        BridgerConfig memory bridgers = _readBridgers();

        vm.startBroadcast(env.deployerPk);

        TronLightClient tronLightClient = _deployTronLightClient(
            tlc.proverAddr,
            tlc.initialBlockHash,
            tlc.initialTxTrieRoot,
            tlc.initialTimestamp,
            tlc.srs,
            tlc.witnessDelegatees,
            tlc.srDataHash
        );
        TronTxReader tronReader = _deployTronTxReader(address(tronLightClient));

        UntronV3 untron = _deployUntronV3(env.controllerAddress);
        _setUntronTronReader(untron, address(tronReader));
        _setUntronUsdt(untron, env.usdt);

        CCTPV2Bridger cctpV2Bridger = _deployCctpV2Bridger(
            address(untron), env.tokenMessengerV2, env.usdc, bridgers.cctpChainIds, bridgers.circleDomains
        );
        USDT0Bridger usdt0Bridger =
            _deployUsdt0Bridger(address(untron), env.usdt0, env.oft, bridgers.usdt0ChainIds, bridgers.eids);

        _setBridgerRoutes(untron, env.usdc, bridgers.cctpChainIds, address(cctpV2Bridger));
        _setBridgerRoutes(untron, env.usdt0, bridgers.usdt0ChainIds, address(usdt0Bridger));
        _transferOwnershipsIfNeeded(env.deployer, env.finalOwner, untron, cctpV2Bridger, usdt0Bridger);

        vm.stopBroadcast();

        Deployed memory deployed = Deployed({
            tronLightClient: address(tronLightClient),
            tronReader: address(tronReader),
            untron: address(untron),
            cctpV2Bridger: address(cctpV2Bridger),
            usdt0Bridger: address(usdt0Bridger)
        });

        console2.log("Config tlc.json:", _tlcConfigPath());
        console2.log("Config bridgers.json:", _bridgerConfigPath());
        console2.log("Deployer:", env.deployer);
        console2.log("Final owner:", env.finalOwner);
        console2.log("CONTROLLER_ADDRESS:", env.controllerAddress);
        console2.log("USDT (accounting):", env.usdt);
        console2.log("TronLightClient:", deployed.tronLightClient);
        console2.log("TronTxReader:", deployed.tronReader);
        console2.log("UntronV3:", deployed.untron);
        console2.log("CCTPV2Bridger:", deployed.cctpV2Bridger);
        console2.log("USDT0Bridger:", deployed.usdt0Bridger);
        console2.log("Configured CCTP routes:", bridgers.cctpChainIds.length);
        console2.log("Configured USDT0 routes:", bridgers.usdt0ChainIds.length);

        _writeOutputIfNeeded(env, tlc, deployed);
    }
}
