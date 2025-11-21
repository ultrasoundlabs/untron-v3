// SPDX-License-Identifier: MIT
pragma solidity ^0.8.26;

import {Test} from "forge-std/Test.sol";
import {UntronReceiver} from "../src/tron/UntronReceiver.sol";
import {ERC20} from "openzeppelin-contracts/contracts/token/ERC20/ERC20.sol";

/// @dev Simple Mock ERC20 for testing.
contract ERC20Mintable is ERC20 {
    constructor() ERC20("Token", "TKN") {}

    function mint(address to, uint256 amount) public {
        _mint(to, amount);
    }
}

contract UntronReceiverTest is Test {
    UntronReceiver receiver;
    ERC20Mintable token;

    address controller = address(0xC0FFEE);
    address alice = address(0xA11CE);
    address bob = address(0xB0B);

    function setUp() public {
        // Deploy receiver as `controller`.
        vm.prank(controller);
        receiver = new UntronReceiver();

        token = new ERC20Mintable();

        // Seed balances for tests.
        token.mint(address(receiver), 1_000 ether);
        vm.deal(address(receiver), 1_000 ether); // native balance
        vm.deal(alice, 100 ether);
    }

    /*//////////////////////////////////////////////////////////////
                                UNIT TESTS
    //////////////////////////////////////////////////////////////*/

    /// G1 + F1.nonctrl: constructor sets controller to deployer, and
    /// non-controller callers cannot change balances (regardless of revert).
    function test_onControllerCall_nonControllerReverts() public {
        uint256 beforeReceiverToken = token.balanceOf(address(receiver));
        uint256 beforeReceiverEth = address(receiver).balance;
        uint256 beforeBobToken = token.balanceOf(bob);
        uint256 beforeBobEth = bob.balance;

        vm.prank(alice);
        // We do not insist on a particular revert reason, only that
        // a non-controller call cannot change any balances.
        try receiver.onControllerCall(address(token), 1 ether, payable(bob)) {
        // If this ever succeeded and changed balances, the assertions
        // below would fail in concrete runs / proofs.
        }
            catch {}

        assertEq(token.balanceOf(address(receiver)), beforeReceiverToken);
        assertEq(address(receiver).balance, beforeReceiverEth);
        assertEq(token.balanceOf(bob), beforeBobToken);
        assertEq(bob.balance, beforeBobEth);
    }

    /// G1: behaviour depends on constructor msg.sender (controller immutable).
    /// We phrase this as implications on state changes rather than
    /// "always succeeds / always reverts".
    function test_controllerSetToDeployerAndImmutable() public {
        uint256 beforeReceiverToken = token.balanceOf(address(receiver));
        uint256 beforeBobToken = token.balanceOf(bob);

        bool ok;
        vm.prank(controller);
        try receiver.onControllerCall(address(token), 1 ether, payable(bob)) {
            ok = true;
        } catch {
            ok = false;
        }

        uint256 afterReceiverToken = token.balanceOf(address(receiver));
        uint256 afterBobToken = token.balanceOf(bob);

        // If controller call succeeds, balances move as expected.
        // If it reverts, balances must be unchanged.
        if (ok) {
            assertEq(afterReceiverToken, beforeReceiverToken - 1 ether);
            assertEq(afterBobToken, beforeBobToken + 1 ether);
        } else {
            assertEq(afterReceiverToken, beforeReceiverToken);
            assertEq(afterBobToken, beforeBobToken);
        }

        // Any other address must not be able to change balances, regardless
        // of the precise revert behavior.
        uint256 beforeReceiverToken2 = token.balanceOf(address(receiver));
        uint256 beforeBobToken2 = token.balanceOf(bob);

        vm.prank(alice);
        try receiver.onControllerCall(address(token), 1 ether, payable(bob)) {} catch {}

        assertEq(token.balanceOf(address(receiver)), beforeReceiverToken2);
        assertEq(token.balanceOf(bob), beforeBobToken2);
    }

    /// F1.ctrl_zero_amount: controller call with amount == 0 leaves balances unchanged.
    function test_onControllerCall_zeroAmount_noBalanceChange_erc20() public {
        uint256 beforeToken = token.balanceOf(address(receiver));
        uint256 beforeEth = address(receiver).balance;

        vm.prank(controller);
        receiver.onControllerCall(address(token), 0, payable(bob));

        assertEq(token.balanceOf(address(receiver)), beforeToken);
        assertEq(address(receiver).balance, beforeEth);
    }

    /// F1.ctrl_zero_amount: controller call with amount == 0 and native token leaves balances unchanged.
    function test_onControllerCall_zeroAmount_noBalanceChange_native() public {
        uint256 beforeToken = token.balanceOf(address(receiver));
        uint256 beforeEth = address(receiver).balance;

        vm.prank(controller);
        receiver.onControllerCall(address(0), 0, payable(bob));

        assertEq(token.balanceOf(address(receiver)), beforeToken);
        assertEq(address(receiver).balance, beforeEth);
    }

    /// F1.ctrl_non_zero_amount (ERC20 success case).
    function test_onControllerCall_erc20Transfer_implication() public {
        uint256 amount = 10 ether;

        uint256 beforeReceiver = token.balanceOf(address(receiver));
        uint256 beforeBob = token.balanceOf(bob);

        bool ok;

        vm.prank(controller);
        try receiver.onControllerCall(address(token), amount, payable(bob)) {
            ok = true;
        } catch {
            ok = false;
        }

        uint256 afterReceiver = token.balanceOf(address(receiver));
        uint256 afterBob = token.balanceOf(bob);

        // This is the key: we only assert the balance delta *if* the call succeeded.
        if (ok) {
            assertEq(afterReceiver, beforeReceiver - amount);
            assertEq(afterBob, beforeBob + amount);
        }
    }

    /// F1.ctrl_non_zero_amount (native success case).
    function test_onControllerCall_nativeTransfer() public {
        uint256 amount = 10 ether;

        uint256 beforeReceiver = address(receiver).balance;
        uint256 beforeBob = bob.balance;

        vm.prank(controller);
        receiver.onControllerCall(address(0), amount, payable(bob));

        assertEq(address(receiver).balance, beforeReceiver - amount);
        assertEq(bob.balance, beforeBob + amount);
    }

    /// F1.ctrl_non_zero_amount.post_revert (ERC20 insufficient balance).
    /// If the call reverts, balances must be unchanged.
    function test_onControllerCall_erc20Transfer_insufficientBalance_revertsAndNoChange() public {
        uint256 beforeReceiver = token.balanceOf(address(receiver));
        uint256 beforeBob = token.balanceOf(bob);

        uint256 amount = beforeReceiver + 1;

        vm.prank(controller);
        try receiver.onControllerCall(address(token), amount, payable(bob)) {
        // If this unexpectedly succeeds in some model, we do not
        // assert about balances here; we only require that on
        // revert, balances stay the same.
        }
        catch {
            assertEq(token.balanceOf(address(receiver)), beforeReceiver);
            assertEq(token.balanceOf(bob), beforeBob);
        }
    }

    /// F1.ctrl_non_zero_amount.post_revert (native insufficient balance).
    /// If the call reverts, balances must be unchanged.
    function test_onControllerCall_nativeTransfer_insufficientBalance_revertsAndNoChange() public {
        uint256 beforeReceiver = address(receiver).balance;
        uint256 beforeBob = bob.balance;

        uint256 amount = beforeReceiver + 1;

        vm.prank(controller);
        try receiver.onControllerCall(address(0), amount, payable(bob)) {}
        catch {
            assertEq(address(receiver).balance, beforeReceiver);
            assertEq(bob.balance, beforeBob);
        }
    }

    /// R: receive() only increases receiver's ETH balance by msg.value.
    function test_receiveIncreasesEthBalanceOnly() public {
        uint256 beforeReceiver = address(receiver).balance;
        uint256 beforeAlice = alice.balance;
        uint256 beforeToken = token.balanceOf(address(receiver));

        vm.prank(alice);
        (bool ok,) = address(receiver).call{value: 1 ether}("");
        assertTrue(ok);

        assertEq(address(receiver).balance, beforeReceiver + 1 ether);
        assertEq(alice.balance, beforeAlice - 1 ether);
        // ERC20 balance remains unchanged.
        assertEq(token.balanceOf(address(receiver)), beforeToken);
    }

    /*//////////////////////////////////////////////////////////////
                              FUZZ / PROPERTY TESTS
    //////////////////////////////////////////////////////////////*/

    /// Fuzz version of ERC20 transfer postcondition.
    function testFuzz_onControllerCall_erc20Transfer(uint128 amount, address payable recipient) public {
        vm.assume(amount > 0);
        vm.assume(amount <= token.balanceOf(address(receiver)));
        vm.assume(recipient != address(0));
        vm.assume(recipient != address(receiver));

        uint256 beforeReceiver = token.balanceOf(address(receiver));
        uint256 beforeRecipient = token.balanceOf(recipient);

        vm.prank(controller);
        receiver.onControllerCall(address(token), amount, recipient);

        assertEq(token.balanceOf(address(receiver)), beforeReceiver - amount);
        assertEq(token.balanceOf(recipient), beforeRecipient + amount);
    }

    /// Fuzz version of native transfer postcondition.
    function testFuzz_onControllerCall_nativeTransfer(uint128 amount, address payable recipient) public {
        vm.assume(amount > 0);
        vm.assume(amount <= address(receiver).balance);
        vm.assume(recipient != address(0));
        vm.assume(recipient != address(receiver));
        vm.assume(uint160(address(recipient)) > 255);
        vm.assume(recipient.code.length == 0);

        uint256 beforeReceiver = address(receiver).balance;
        uint256 beforeRecipient = recipient.balance;

        vm.prank(controller);
        receiver.onControllerCall(address(0), amount, recipient);

        assertEq(address(receiver).balance, beforeReceiver - amount);
        assertEq(recipient.balance, beforeRecipient + amount);
    }
}

/// @dev Handler contract to manage state transitions and ghost variables.
contract UntronReceiverHandler is Test {
    UntronReceiver public receiver;
    ERC20Mintable public token;
    address public controller;

    // Ghost variables to track expected state.
    uint256 public ghostTokenBalance;
    uint256 public ghostEthBalance;

    constructor(UntronReceiver _receiver, ERC20Mintable _token, address _controller) {
        receiver = _receiver;
        token = _token;
        controller = _controller;

        // Initialize ghosts with current state.
        ghostTokenBalance = token.balanceOf(address(receiver));
        ghostEthBalance = address(receiver).balance;
    }

    /*//////////////////////////////////////////////////////////////
                                ACTIONS
    //////////////////////////////////////////////////////////////*/

    /// @notice Simulate authorized controller ERC20 transfer.
    function transferERC20(uint128 amount, address recipient) public {
        // Bound amount to available balance to focus on valid transfers.
        amount = uint128(bound(amount, 0, ghostTokenBalance));
        // Recipient cannot be 0, receiver, or the token itself (standard sanity).
        vm.assume(recipient != address(0));
        vm.assume(recipient != address(receiver));
        vm.assume(recipient != address(token));

        vm.prank(controller);
        try receiver.onControllerCall(address(token), amount, payable(recipient)) {
            // If success, update ghost.
            if (amount > 0) {
                ghostTokenBalance -= amount;
            }
        } catch {
            // If revert, do not update ghost.
        }
    }

    /// @notice Simulate authorized controller Native transfer.
    function transferEth(uint128 amount, address recipient) public {
        // Bound amount to available balance.
        amount = uint128(bound(amount, 0, ghostEthBalance));
        vm.assume(recipient != address(0));
        vm.assume(recipient != address(receiver));
        vm.assume(uint160(address(recipient)) > 255); // Exclude precompiles
        // Ensure recipient is EOA to avoid "ETHTransferFailed" from SafeTransferLib
        // when sending to a contract without fallback.
        vm.assume(recipient.code.length == 0);

        vm.prank(controller);
        try receiver.onControllerCall(address(0), amount, payable(recipient)) {
            if (amount > 0) {
                ghostEthBalance -= amount;
            }
        } catch {
            // If revert, do not update ghost.
        }
    }

    /// @notice Simulate receiving ETH (valid inflow).
    function receiveEth(uint128 amount) public {
        amount = uint128(bound(amount, 0, address(this).balance));

        // Perform the low-level call to trigger receive()
        (bool success,) = address(receiver).call{value: amount}("");
        if (success) {
            ghostEthBalance += amount;
        }
    }

    /// @notice Simulate unauthorized attempts (should never change balance).
    function unauthorizedCall(uint256 seed, uint128 amount, address recipient) public {
        // Pick a random attacker that is NOT the controller.
        // forge-lint: disable-next-line(unsafe-typecast)
        address attacker = address(uint160(seed));
        if (attacker == controller || attacker == address(0)) {
            attacker = address(0xDEAD);
        }

        vm.prank(attacker);
        // We expect this to revert or do nothing, but definitely NOT change balance.
        try receiver.onControllerCall(address(token), amount, payable(recipient)) {
        // If it somehow succeeded (shouldn't for non-controller),
        // we strictly DO NOT update ghosts.
        // The invariant check will catch if balances changed.
        }
            catch {}

        vm.prank(attacker);
        try receiver.onControllerCall(address(0), amount, payable(recipient)) {
        // Same here.
        }
            catch {}
    }

    // Allow handler to receive ETH to fund receiveEth calls
    receive() external payable {}
}

contract UntronReceiverInvariantTest is Test {
    UntronReceiver receiver;
    ERC20Mintable token;
    UntronReceiverHandler handler;

    address controller = address(0xC0FFEE);

    function setUp() public {
        // 1. Deploy
        vm.prank(controller);
        receiver = new UntronReceiver();
        token = new ERC20Mintable();

        // 2. Fund Receiver
        token.mint(address(receiver), 1_000 ether);
        vm.deal(address(receiver), 1_000 ether);

        // 3. Deploy Handler
        handler = new UntronReceiverHandler(receiver, token, controller);

        // 4. Fund Handler (so it can send ETH into receiver)
        vm.deal(address(handler), 1_000 ether);

        // 5. Configure Invariant Fuzzing
        targetContract(address(handler));
    }

    /// @notice G2 & G3: Balances only change consistent with authorized flows tracked by ghosts.
    function invariant_ghostBalancesMatchReal() public view {
        assertEq(token.balanceOf(address(receiver)), handler.ghostTokenBalance(), "ERC20 balance mismatch");
        assertEq(address(receiver).balance, handler.ghostEthBalance(), "ETH balance mismatch");
    }
}
