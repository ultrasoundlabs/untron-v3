# Detailed breakdown of Untron V3 smart contracts

## UntronReceiver

UntronReceiver is a smart contract that has a single controller (the deployer). Only the controller can instruct it to transfer ERC-20 tokens or native coins from itself to arbitrary recipients.

#### Global assumption A1:
For any supported token T, we assume:
- T does not rebase or otherwise change balances of arbitrary holders without their participation.
- Calls to transfer/from this contract either revert and leave all balances unchanged, or succeed and change balances exactly by the requested amount, without additional fees or burns.
- T cannot unilaterally confiscate this contract's balances.

If A1 is violated, UntronReceiver’s code does not introduce any additional attack surface: any unexpected loss of funds is inherent to the token’s logic itself, not to UntronReceiver.

#### Global assumption A2 - TokenUtils.transfer correctness under A1:
- On success, TokenUtils.transfer(token, recipient, amount) moves exactly amount of the specified asset from UntronReceiver to recipient.
- On failure (e.g. insufficient balance, non-compliant token, revert in the recipient), it reverts and leaves balances unchanged.

#### Global invariant G1 - Controller immutability
For all states reachable after the constructor finishes:
- CONTROLLER == constructor.msg.sender.

#### Global invariant G2 - Only controller can decrease balances
Under assumption A1, for any completed call and any asset (ERC-20 or native):
- If balance_after(UntronReceiver, asset) < balance_before(UntronReceiver, asset), then msg.sender == CONTROLLER.

#### Global invariant G3 – Authorized outgoing transfers

Under assumptions A1 and A2:
- Any state where balance_after(UntronReceiver, asset) < balance_before(UntronReceiver, asset) must come from executing TokenUtils.transfer(token, recipient, amount) inside onControllerCall.

### Storage
```solidity
address internal immutable CONTROLLER;
```

#### Meaning
- CONTROLLER stores the only address allowed to trigger outgoing transfers via onControllerCall.
- Because it is immutable and only assigned in the constructor, it can never change after construction.

### Errors

```solidity
error NotController();
```

### Constructor

```solidity
constructor() {
    CONTROLLER = msg.sender;
}
```

#### Preconditions C.pre
- None beyond normal EVM rules; any address can deploy.
- Storage is in its initial zeroed state.

#### Postconditions C.post
Let d be the deployer address (the msg.sender of the constructor call):
- After the constructor finishes:
    - CONTROLLER == d.
    - Combined with immutable, this establishes global invariant G1.

### Functions

```solidity
function onControllerCall(
    address token,
    uint256 amount,
    address payable recipient
) external;
```

**Case 1 - Non-controller caller**

#### Preconditions F1.nonctrl.pre
- msg.sender != CONTROLLER.

#### Postconditions F1.nonctrl.post
- The call reverts with NotController().
- No storage variables are changed.
- No ERC-20 or native balances of UntronReceiver are changed by this call.

**Case 2 - Controller caller, amount == 0**

#### Preconditions F.ctrl_zero_amount.pre
- msg.sender == CONTROLLER.
- amount == 0.
- No assumptions needed on token or recipient for correctness.

#### Postconditions F1.ctrl_zero_amount.post
- The call returns successfully (does not revert).
- TokenUtils.transfer is not called.
- No storage variables are changed.
- No ERC-20 or native balances of UntronReceiver are changed by this call.

**Case 3 – Controller caller, amount > 0**

#### Preconditions F1.ctrl_non_zero_amount.pre
- msg.sender == CONTROLLER.
- amount > 0.

#### Postconditions F1.ctrl_non_zero_amount.post_success (if call does not revert)

If token != address(0) (ERC-20 case):
- erc20Balance(token, UntronReceiver)_after = erc20Balance(token, UntronReceiver)_before - amount.
- erc20Balance(token, recipient)_after = erc20Balance(token, recipient)_before + amount.

If token == address(0) (native case):
- ethBalance(UntronReceiver)_after = ethBalance(UntronReceiver)_before - amount.
- ethBalance(recipient)_after = ethBalance(recipient)_before + amount.

In both subcases:
- No storage variables of UntronReceiver are modified.

#### Postconditions F1.ctrl_non_zero_amount.post_revert (if call reverts due to TokenUtils.transfer)
- All balances and storage are unchanged compared to the state before the call.

```
receive() external payable {}
```

#### Preconditions R.pre
- The function is invoked by sending native coin to the contract with empty calldata.
- No constraints on msg.sender.
- msg.value can be zero or positive.

#### Postconditions R.post
- The call returns successfully and does not revert (ignoring gas issues).
- No storage variables are changed.
- The only state change is:
- ethBalance(UntronReceiver)_after = ethBalance(UntronReceiver)_before + msg.value.
