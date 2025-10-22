// SPDX-License-Identifier: GPL-3.0-or-later
pragma solidity ^0.8.24;

import "forge-std/Test.sol";
import {StdInvariant} from "forge-std/StdInvariant.sol";

import {TokenUtilsHarness} from "./harness/TokenUtilsHarness.sol";
import {TokenAmount} from "../src/utils/TokenUtils.sol";

// ---------- Minimal, standards-compliant mock ERC20 ----------

contract MockERC20 {
    string public name = "Mock";
    string public symbol = "MOCK";
    uint8 public decimals = 18;

    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    event Transfer(address indexed from, address indexed to, uint256 amount);
    event Approval(address indexed owner, address indexed spender, uint256 amount);

    function mint(address to, uint256 amount) external {
        totalSupply += amount;
        balanceOf[to] += amount;
        emit Transfer(address(0), to, amount);
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        emit Approval(msg.sender, spender, amount);
        return true;
    }

    function transfer(address to, uint256 amount) external virtual returns (bool) {
        _transfer(msg.sender, to, amount);
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external virtual returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        require(allowed >= amount, "ALW");
        if (allowed != type(uint256).max) allowance[from][msg.sender] = allowed - amount;
        _transfer(from, to, amount);
        return true;
    }

    function _transfer(address from, address to, uint256 amount) internal {
        require(balanceOf[from] >= amount, "BAL");
        unchecked {
            balanceOf[from] -= amount;
            balanceOf[to] += amount;
        }
        emit Transfer(from, to, amount);
    }
}

// ERC20 that does not return booleans (no-return pattern)
contract NoReturnERC20 {
    string public name = "NoReturn";
    string public symbol = "NORET";
    uint8 public decimals = 18;

    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    event Transfer(address indexed from, address indexed to, uint256 amount);
    event Approval(address indexed owner, address indexed spender, uint256 amount);

    function mint(address to, uint256 amount) external {
        totalSupply += amount;
        balanceOf[to] += amount;
        emit Transfer(address(0), to, amount);
    }

    // No return value
    function approve(address spender, uint256 amount) external {
        allowance[msg.sender][spender] = amount;
        emit Approval(msg.sender, spender, amount);
    }

    // No return value
    function transfer(address to, uint256 amount) external {
        _transfer(msg.sender, to, amount);
    }

    // No return value
    function transferFrom(address from, address to, uint256 amount) external {
        uint256 allowed = allowance[from][msg.sender];
        require(allowed >= amount, "ALW");
        if (allowed != type(uint256).max) allowance[from][msg.sender] = allowed - amount;
        _transfer(from, to, amount);
    }

    function _transfer(address from, address to, uint256 amount) internal {
        require(balanceOf[from] >= amount, "BAL");
        unchecked {
            balanceOf[from] -= amount;
            balanceOf[to] += amount;
        }
        emit Transfer(from, to, amount);
    }
}

// Fee-on-transfer token that deducts 1% fee to a burn address
contract FeeOnTransferERC20 {
    string public name = "FeeToken";
    string public symbol = "FEE";
    uint8 public decimals = 18;

    uint256 public totalSupply;
    mapping(address => uint256) public balanceOf;
    mapping(address => mapping(address => uint256)) public allowance;

    event Transfer(address indexed from, address indexed to, uint256 amount);
    event Approval(address indexed owner, address indexed spender, uint256 amount);

    address public constant BURN = address(0xdead);

    function mint(address to, uint256 amount) external {
        totalSupply += amount;
        balanceOf[to] += amount;
        emit Transfer(address(0), to, amount);
    }

    function approve(address spender, uint256 amount) external returns (bool) {
        allowance[msg.sender][spender] = amount;
        emit Approval(msg.sender, spender, amount);
        return true;
    }

    function transfer(address to, uint256 amount) external returns (bool) {
        _transfer(msg.sender, to, amount);
        return true;
    }

    function transferFrom(address from, address to, uint256 amount) external returns (bool) {
        uint256 allowed = allowance[from][msg.sender];
        require(allowed >= amount, "ALW");
        if (allowed != type(uint256).max) allowance[from][msg.sender] = allowed - amount;
        _transfer(from, to, amount);
        return true;
    }

    function _transfer(address from, address to, uint256 amount) internal {
        require(balanceOf[from] >= amount, "BAL");
        // 1% fee
        uint256 fee = amount / 100;
        uint256 receiveAmt = amount - fee;
        unchecked {
            balanceOf[from] -= amount;
            balanceOf[to] += receiveAmt;
            balanceOf[BURN] += fee;
        }
        emit Transfer(from, to, receiveAmt);
        emit Transfer(from, BURN, fee);
    }
}

// ---------- ETH receivers for try/safe paths ----------

contract AcceptingReceiver {
    uint256 public totalReceived;

    receive() external payable {
        totalReceived += msg.value;
    }
}

contract FailingReceiver {
    receive() external payable {
        revert("nope");
    }
}

contract ReentrantReceiver {
    TokenUtilsHarness public h;
    address payable public sweepTo;
    uint256 public reentered;

    constructor(TokenUtilsHarness _h, address payable _sweepTo) {
        h = _h;
        sweepTo = _sweepTo;
    }

    receive() external payable {
        // Attempt to sweep remaining ETH from harness on first entry
        if (reentered == 0) {
            reentered = 1;
            h.transferBalanceToken(address(0), sweepTo);
        }
    }
}

// ---------- Handler that Foundry will fuzz-call ----------

contract Handler {
    TokenUtilsHarness public h;
    MockERC20 public token;

    address payable public rec1;
    address payable public rec2;
    AcceptingReceiver public acceptor;
    FailingReceiver public rejector;

    // Track last tryTransfer outcomes to assert semantics in invariants
    bool public lastTryERC20Success;
    uint256 public lastTryERC20Amount;
    address public lastTryERC20Recipient;
    uint256 public lastTryERC20Before;
    uint256 public lastTryERC20After;

    uint256 public lastTryERC20HarnessBefore;
    uint256 public lastTryERC20HarnessAfter;

    bool public lastTryETHSuccess;
    uint256 public lastTryETHAmount;
    address public lastTryETHRecipient;
    uint256 public lastTryETHBefore;
    uint256 public lastTryETHAfter;

    uint256 public lastTryETHHarnessBefore;
    uint256 public lastTryETHHarnessAfter;

    constructor(
        TokenUtilsHarness _h,
        MockERC20 _token,
        address payable _rec1,
        address payable _rec2,
        AcceptingReceiver _acceptor,
        FailingReceiver _rejector
    ) {
        h = _h;
        token = _token;
        rec1 = _rec1;
        rec2 = _rec2;
        acceptor = _acceptor;
        rejector = _rejector;
    }

    // ---- Actions ----

    function sendToken(uint256 amt, uint8 which) external {
        uint256 bal = token.balanceOf(address(h));
        if (bal == 0) return;
        address payable to = (which % 2 == 0) ? rec1 : rec2;
        uint256 x = amt % (bal + 1); // <= balance
        if (x == 0) return;
        h.transferToken(address(token), to, x);
    }

    function sendTryToken(uint256 amt, uint8 which) external {
        uint256 bal = token.balanceOf(address(h));
        address payable to = (which % 2 == 0) ? rec1 : rec2;
        uint256 x = (bal == 0) ? 0 : amt % (bal + 1);

        // Deterministic allowance scenarios to cover success/failure paths:
        // - which % 4 == 0: approve max (success path)
        // - which % 4 == 1: explicitly set allowance to 0 (force failure)
        // - which % 4 == 2: leave as-is (fuzz may toggle)
        // - which % 4 == 3: approve a small allowance < x if possible
        uint8 mode = which % 4;
        if (mode == 0) {
            h.approveToken(address(token), address(h), type(uint256).max);
        } else if (mode == 1) {
            h.approveToken(address(token), address(h), 0);
        } else if (mode == 3) {
            uint256 allowAmt = x > 0 ? x - 1 : 0;
            h.approveToken(address(token), address(h), allowAmt);
        }

        lastTryERC20Recipient = to;
        lastTryERC20Amount = x;
        lastTryERC20Before = token.balanceOf(to);
        lastTryERC20HarnessBefore = token.balanceOf(address(h));

        bool ok = h.tryTransferToken(address(token), to, x);
        lastTryERC20Success = ok;
        lastTryERC20After = token.balanceOf(to);
        lastTryERC20HarnessAfter = token.balanceOf(address(h));
    }

    function sendETH(uint256 amt, uint8 which) external {
        uint256 bal = address(h).balance;
        if (bal == 0) return;
        address payable to = (which % 3 == 0) ? rec1 : (which % 3 == 1) ? rec2 : payable(address(acceptor));
        uint256 x = amt % (bal + 1);
        if (x == 0) return;
        h.transferToken(address(0), to, x);
    }

    function sendTryETH(uint256 amt, bool toRejector) external {
        uint256 bal = address(h).balance;
        address payable to = toRejector ? payable(address(rejector)) : payable(address(acceptor));
        uint256 x = (bal == 0) ? 0 : amt % (bal + 1);

        lastTryETHRecipient = to;
        lastTryETHAmount = x;

        uint256 beforeBal = to.balance;
        lastTryETHBefore = beforeBal;
        lastTryETHHarnessBefore = address(h).balance;

        bool ok = h.tryTransferToken(address(0), to, x);
        lastTryETHSuccess = ok;

        lastTryETHAfter = to.balance;
        lastTryETHHarnessAfter = address(h).balance;
    }

    function sweepTokenBalance(uint8 which) external {
        address payable to = (which % 2 == 0) ? rec1 : rec2;
        h.transferBalanceToken(address(token), to);
    }

    function sweepEthBalance(uint8 which) external {
        address payable to = (which % 2 == 0) ? rec1 : rec2;
        h.transferBalanceToken(address(0), to);
    }

    function approveSelf(uint256 amount) external {
        h.approveToken(address(token), address(h), amount);
    }
}

// ---------- Invariant test ----------

contract TokenUtilsInvariants is StdInvariant, Test {
    TokenUtilsHarness h;
    MockERC20 token;
    AcceptingReceiver acceptor;
    FailingReceiver rejector;
    Handler handler;

    address payable rec1;
    address payable rec2;

    uint256 initialTokenAtHarness;
    uint256 initialEthAtHarness;

    function setUp() public {
        h = new TokenUtilsHarness();
        token = new MockERC20();
        acceptor = new AcceptingReceiver();
        rejector = new FailingReceiver();

        rec1 = payable(makeAddr("rec1"));
        rec2 = payable(makeAddr("rec2"));

        // Seed balances
        initialTokenAtHarness = 1_000_000 ether;
        token.mint(address(h), initialTokenAtHarness);

        initialEthAtHarness = 100 ether;
        vm.deal(address(h), initialEthAtHarness);

        // Do not pre-approve; allowance will be varied by the handler to exercise both branches

        handler = new Handler(h, token, rec1, rec2, acceptor, rejector);

        // Fuzzer will only call into the Handler
        targetContract(address(handler));

        // Optionally constrain which functions get called more often
        bytes4[] memory sels = new bytes4[](7);
        sels[0] = Handler.sendToken.selector;
        sels[1] = Handler.sendTryToken.selector;
        sels[2] = Handler.sendETH.selector;
        sels[3] = Handler.sendTryETH.selector;
        sels[4] = Handler.sweepTokenBalance.selector;
        sels[5] = Handler.sweepEthBalance.selector;
        sels[6] = Handler.approveSelf.selector;
        targetSelector(FuzzSelector({addr: address(handler), selectors: sels}));
    }

    // ---- Invariants ----

    // 1) ERC20 conservation: all minted tokens stay within the known set
    function invariant_ERC20Conservation() public view {
        uint256 sum = token.balanceOf(address(h)) + token.balanceOf(rec1) + token.balanceOf(rec2);
        assertEq(sum, initialTokenAtHarness, "ERC20 conservation broken");
    }

    // 2) ETH conservation: ETH only moves among harness + receivers we use
    function invariant_ETHConservation() public view {
        uint256 sum =
            address(h).balance + rec1.balance + rec2.balance + address(acceptor).balance + address(rejector).balance; // should remain zero (always reverts)
        assertEq(sum, initialEthAtHarness, "ETH conservation broken");
    }

    // 3) tryTransfer(ERC20): if it reports failure, recipient's balance didn't increase

    function invariant_TryERC20Semantics() public view {
        if (!handler.lastTryERC20Success()) {
            assertEq(
                handler.lastTryERC20After(), handler.lastTryERC20Before(), "ERC20 tryTransfer false but balance changed"
            );
            // Harness balance unchanged on failure
            assertEq(
                handler.lastTryERC20HarnessAfter(),
                handler.lastTryERC20HarnessBefore(),
                "ERC20 harness delta on failed try"
            );
        } else {
            // On success, recipient gained exactly the attempted amount
            assertEq(
                handler.lastTryERC20After() - handler.lastTryERC20Before(),
                handler.lastTryERC20Amount(),
                "ERC20 tryTransfer true but incorrect delta"
            );
            // Harness lost exactly the attempted amount
            assertEq(
                handler.lastTryERC20HarnessBefore() - handler.lastTryERC20HarnessAfter(),
                handler.lastTryERC20Amount(),
                "ERC20 harness delta mismatch on success"
            );
        }
    }

    // 4) tryTransfer(ETH): same semantics
    function invariant_TryETHSemantics() public view {
        if (!handler.lastTryETHSuccess()) {
            assertEq(handler.lastTryETHAfter(), handler.lastTryETHBefore(), "ETH tryTransfer false but balance changed");
            assertEq(
                handler.lastTryETHHarnessAfter(), handler.lastTryETHHarnessBefore(), "ETH harness delta on failed try"
            );
        } else {
            assertEq(
                handler.lastTryETHAfter() - handler.lastTryETHBefore(),
                handler.lastTryETHAmount(),
                "ETH tryTransfer true but incorrect delta"
            );
            assertEq(
                handler.lastTryETHHarnessBefore() - handler.lastTryETHHarnessAfter(),
                handler.lastTryETHAmount(),
                "ETH harness delta mismatch on success"
            );
        }
    }

    // ---------- Unit tests for specific behaviors ----------

    function test_EthSafeTransferToRejectorReverts() public {
        vm.expectRevert();
        h.transferToken(address(0), payable(address(rejector)), 1 ether);
    }

    function test_TransferBalanceErc20() public {
        // Ensure harness has some tokens
        uint256 pre = token.balanceOf(address(h));
        assertGt(pre, 0);
        uint256 recPre = token.balanceOf(rec1);

        uint256 swept = h.transferBalanceToken(address(token), rec1);
        assertEq(swept, pre);
        assertEq(token.balanceOf(address(h)), 0);
        assertEq(token.balanceOf(rec1) - recPre, pre);

        // Refill a bit for subsequent invariants
        token.mint(address(h), 1000 ether);
    }

    function test_TransferBalanceEth() public {
        uint256 pre = address(h).balance;
        assertGt(pre, 0);
        uint256 recPre = rec2.balance;

        uint256 swept = h.transferBalanceToken(address(0), rec2);
        assertEq(swept, pre);
        assertEq(address(h).balance, 0);
        assertEq(rec2.balance - recPre, pre);

        // Replenish
        vm.deal(address(h), 10 ether);
    }

    function test_ApproveSemanticsErc20() public {
        uint256 amt = 123 ether;
        handler.approveSelf(amt);
        assertEq(token.allowance(address(h), address(h)), amt);
        // Balances unchanged
        assertEq(token.balanceOf(address(h)), initialTokenAtHarness);
    }

    function test_ApproveSemanticsEthNoop() public {
        uint256 preH = address(h).balance;
        uint256 preR1 = rec1.balance;
        uint256 preR2 = rec2.balance;
        handler.h().approveToken(address(0), address(h), 777);
        assertEq(address(h).balance, preH);
        assertEq(rec1.balance, preR1);
        assertEq(rec2.balance, preR2);
    }

    function test_TransferFromEthReverts() public {
        vm.expectRevert();
        h.transferFromToken(address(0), address(this), address(rec1), 1);
    }

    function test_TransferFromErc20MovesAndDecrementsAllowance() public {
        address alice = makeAddr("alice");
        token.mint(alice, 1000 ether);
        vm.startPrank(alice);
        token.approve(address(h), 200 ether);
        vm.stopPrank();

        uint256 recPre = token.balanceOf(rec1);
        handler.h().transferFromToken(address(token), alice, rec1, 150 ether);
        assertEq(token.balanceOf(rec1) - recPre, 150 ether);
        assertEq(token.allowance(alice, address(h)), 50 ether);
    }

    function test_TransferFromErc20AllowanceDoubleDecrement() public {
        address bob = makeAddr("bob");
        token.mint(bob, 1000 ether);
        vm.startPrank(bob);
        token.approve(address(h), 200 ether);
        vm.stopPrank();

        handler.h().transferFromToken(address(token), bob, rec1, 150 ether);
        assertEq(token.allowance(bob, address(h)), 50 ether);
        handler.h().transferFromToken(address(token), bob, rec2, 30 ether);
        assertEq(token.allowance(bob, address(h)), 20 ether);
    }

    function test_Erc20SafeTransferRevertsIfInsufficient() public {
        uint256 bal = token.balanceOf(address(h));
        vm.expectRevert(); // MockERC20 reverts with "BAL"
        h.transferToken(address(token), rec1, bal + 1);
    }

    function test_TransferFromErc20MaxAllowanceNotDecrement() public {
        address alice = makeAddr("aliceMax");
        token.mint(alice, 1000 ether);
        vm.startPrank(alice);
        token.approve(address(h), type(uint256).max);
        vm.stopPrank();

        uint256 allowPre = token.allowance(alice, address(h));
        handler.h().transferFromToken(address(token), alice, rec2, 10 ether);
        assertEq(token.allowance(alice, address(h)), allowPre);
    }

    function test_CheckBalanceProperty() public {
        // Construct a mixed array: [too small, exactly enough (ETH), big gap, enough (ERC20), none]
        TokenAmount[] memory arr = new TokenAmount[](5);
        arr[0] = TokenAmount({token: address(token), amount: token.balanceOf(address(h)) + 1});
        arr[1] = TokenAmount({token: address(0), amount: address(h).balance});
        arr[2] = TokenAmount({token: address(0), amount: address(h).balance + 1});
        arr[3] = TokenAmount({token: address(token), amount: token.balanceOf(address(h))});
        arr[4] = TokenAmount({token: address(0), amount: type(uint256).max});

        uint256 idx = h.checkBalanceHarness(arr);
        assertEq(idx, 1);

        // If we drain ETH, the next sufficient should be the ERC20 entry at index 3
        h.transferBalanceToken(address(0), rec1);
        uint256 idx2 = h.checkBalanceHarness(arr);
        assertEq(idx2, 3);
    }

    function test_ZeroAmountTrySemantics() public {
        // Ensure allowance is sufficient
        handler.approveSelf(type(uint256).max);
        uint256 recPre = token.balanceOf(rec1);
        uint256 hPre = token.balanceOf(address(h));
        bool ok = handler.h().tryTransferToken(address(token), rec1, 0);
        assertTrue(ok);
        assertEq(token.balanceOf(rec1), recPre);
        assertEq(token.balanceOf(address(h)), hPre);
    }

    function test_NoReturnTokenSupport() public {
        NoReturnERC20 nr = new NoReturnERC20();
        nr.mint(address(h), 100 ether);
        // Approve self for transferFrom path
        vm.prank(address(h));
        nr.approve(address(h), type(uint256).max);
        bool ok = handler.h().tryTransferToken(address(nr), rec1, 10 ether);
        assertTrue(ok);
        assertEq(nr.balanceOf(rec1), 10 ether);
    }

    function test_FeeOnTransfer_DoesNotGuaranteeFullDelivery() public {
        FeeOnTransferERC20 ft = new FeeOnTransferERC20();
        ft.mint(address(h), 100 ether);
        // Approve self
        vm.prank(address(h));
        ft.approve(address(h), type(uint256).max);
        uint256 recPre = ft.balanceOf(rec1);
        bool ok = handler.h().tryTransferToken(address(ft), rec1, 50 ether);
        assertTrue(ok);
        // Recipient gets less than requested amount (due to 1% fee)
        uint256 delta = ft.balanceOf(rec1) - recPre;
        assertLt(delta, 50 ether);
    }

    function test_ReentrancySignalCheck() public {
        ReentrantReceiver r = new ReentrantReceiver(h, rec2);
        uint256 hPre = address(h).balance;
        vm.deal(address(h), hPre + 1 ether);
        // Send 1 ether which will trigger reentrant sweep
        handler.h().transferToken(address(0), payable(address(r)), 1 ether);
        // All ETH should now be at rec2 except the 1 ether sent to r
        // Conservation already covered by invariant; here we only assert no revert and that r observed reentry
        assertEq(r.reentered(), 1);
    }

    function test_TransferBalanceZero_NoOp() public {
        h.transferBalanceToken(address(token), rec1);
        h.transferBalanceToken(address(0), rec1);

        uint256 erc20 = h.transferBalanceToken(address(token), rec1);
        uint256 eth = h.transferBalanceToken(address(0), rec1);

        assertEq(erc20, 0);
        assertEq(eth, 0);
    }

    function test_TryERC20_InsufficientBalance_FalseNoDelta() public {
        uint256 bal = token.balanceOf(address(h));
        handler.approveSelf(type(uint256).max);
        uint256 rPre = token.balanceOf(rec1);
        bool ok = h.tryTransferToken(address(token), rec1, bal + 1);
        assertFalse(ok);
        assertEq(token.balanceOf(rec1), rPre);
        assertEq(token.balanceOf(address(h)), bal);
    }

    function test_TryETH_InsufficientBalance_FalseNoDelta() public {
        uint256 bal = address(h).balance;
        uint256 rPre = rec1.balance;
        bool ok = h.tryTransferToken(address(0), rec1, bal + 1);
        assertFalse(ok);
        assertEq(rec1.balance, rPre);
        assertEq(address(h).balance, bal);
    }

    function test_TransferBalanceEth_ToRejector_Reverts() public {
        vm.deal(address(h), 1 ether);
        vm.expectRevert();
        h.transferBalanceToken(address(0), payable(address(rejector)));
    }

    function test_CheckBalance_EmptyAndZeroAmount() public view {
        TokenAmount[] memory empty = new TokenAmount[](0);
        assertEq(h.checkBalanceHarness(empty), 0);

        TokenAmount[] memory z = new TokenAmount[](2);
        z[0] = TokenAmount({token: address(0), amount: 0});
        z[1] = TokenAmount({token: address(token), amount: type(uint256).max});
        assertEq(h.checkBalanceHarness(z), 0);
    }

    function test_FalseReturnToken_TryTransferFalse() public {
        FalseReturnERC20 f = new FalseReturnERC20();
        f.mint(address(h), 100 ether);
        vm.prank(address(h));
        f.approve(address(h), type(uint256).max);
        uint256 rPre = f.balanceOf(rec1);
        bool ok = h.tryTransferToken(address(f), rec1, 10 ether);
        assertFalse(ok);
        assertEq(f.balanceOf(rec1), rPre);
    }
}

contract FalseReturnERC20 is MockERC20 {
    function transfer(address, uint256) public pure override returns (bool) {
        return false;
    }

    function transferFrom(address, address, uint256) public pure override returns (bool) {
        return false;
    }
}

// ---------- Kontrol-friendly parameterized proofs ----------

contract TokenUtilsProofs is Test {
    function prove_TryERC20_Success_DebitsExact(uint256 amt) public {
        TokenUtilsHarness h = new TokenUtilsHarness();
        MockERC20 t = new MockERC20();
        t.mint(address(h), 1_000_000 ether);
        h.approveToken(address(t), address(h), type(uint256).max);

        address payable to = payable(address(0xBEEF));
        uint256 balH0 = t.balanceOf(address(h));
        vm.assume(amt <= balH0);

        uint256 to0 = t.balanceOf(to);
        bool ok = h.tryTransferToken(address(t), to, amt);
        assertTrue(ok);
        assertEq(t.balanceOf(to) - to0, amt);
        assertEq(balH0 - t.balanceOf(address(h)), amt);
    }

    function prove_TryERC20_Insufficient_FalseNoDelta(uint256 amt) public {
        TokenUtilsHarness h = new TokenUtilsHarness();
        MockERC20 t = new MockERC20();
        t.mint(address(h), 100 ether);
        h.approveToken(address(t), address(h), type(uint256).max);

        address payable to = payable(address(0xCAFE));
        uint256 balH0 = t.balanceOf(address(h));
        vm.assume(amt > balH0);

        uint256 to0 = t.balanceOf(to);
        bool ok = h.tryTransferToken(address(t), to, amt);
        assertFalse(ok);
        assertEq(t.balanceOf(to), to0);
        assertEq(t.balanceOf(address(h)), balH0);
    }

    function prove_TryETH_Success_DebitsExact(uint256 amt) public {
        TokenUtilsHarness h = new TokenUtilsHarness();
        AcceptingReceiver acceptor = new AcceptingReceiver();
        vm.deal(address(h), 1_000_000 ether);

        address payable to = payable(address(acceptor));
        uint256 balH0 = address(h).balance;
        vm.assume(amt <= balH0);

        uint256 to0 = to.balance;
        bool ok = h.tryTransferToken(address(0), to, amt);
        assertTrue(ok);
        assertEq(to.balance - to0, amt);
        assertEq(balH0 - address(h).balance, amt);
    }

    function prove_TryETH_Insufficient_FalseNoDelta(uint256 amt) public {
        TokenUtilsHarness h = new TokenUtilsHarness();
        AcceptingReceiver acceptor = new AcceptingReceiver();
        vm.deal(address(h), 100 ether);

        address payable to = payable(address(acceptor));
        uint256 balH0 = address(h).balance;
        vm.assume(amt > balH0);

        uint256 to0 = to.balance;
        bool ok = h.tryTransferToken(address(0), to, amt);
        assertFalse(ok);
        assertEq(to.balance, to0);
        assertEq(address(h).balance, balH0);
    }

    function prove_TransferBalanceERC20_SweepsAll() public {
        TokenUtilsHarness h = new TokenUtilsHarness();
        MockERC20 t = new MockERC20();
        t.mint(address(h), 1234 ether);

        address payable rec = payable(address(0xA11CE));
        uint256 pre = t.balanceOf(address(h));
        uint256 recPre = t.balanceOf(rec);
        uint256 swept = h.transferBalanceToken(address(t), rec);
        assertEq(swept, pre);
        assertEq(t.balanceOf(address(h)), 0);
        assertEq(t.balanceOf(rec) - recPre, pre);
    }

    function prove_TransferBalanceETH_SweepsAll() public {
        TokenUtilsHarness h = new TokenUtilsHarness();
        vm.deal(address(h), 77 ether);
        address payable rec = payable(address(0xB0B));
        uint256 pre = address(h).balance;
        uint256 recPre = rec.balance;
        uint256 swept = h.transferBalanceToken(address(0), rec);
        assertEq(swept, pre);
        assertEq(address(h).balance, 0);
        assertEq(rec.balance - recPre, pre);
    }

    function prove_ApproveSemanticsErc20(uint256 amt) public {
        TokenUtilsHarness h = new TokenUtilsHarness();
        MockERC20 t = new MockERC20();
        t.mint(address(h), 1 ether);
        uint256 pre = t.balanceOf(address(h));
        h.approveToken(address(t), address(h), amt);
        assertEq(t.allowance(address(h), address(h)), amt);
        assertEq(t.balanceOf(address(h)), pre);
    }

    function prove_ApproveSemanticsEthNoop(uint256 amt) public {
        TokenUtilsHarness h = new TokenUtilsHarness();
        vm.deal(address(h), 5 ether);
        uint256 preH = address(h).balance;
        h.approveToken(address(0), address(h), amt);
        assertEq(address(h).balance, preH);
    }

    function prove_TransferFromErc20_DecrementsAndMoves(uint256 approveAmt, uint256 spendAmt) public {
        TokenUtilsHarness h = new TokenUtilsHarness();
        MockERC20 t = new MockERC20();
        address from = address(0xF00D);
        address payable to = payable(address(0xD00D));
        uint256 minted = 1000 ether;
        t.mint(from, minted);

        vm.assume(approveAmt > 0 && approveAmt <= minted);
        vm.assume(spendAmt > 0 && spendAmt <= approveAmt);

        vm.prank(from);
        t.approve(address(h), approveAmt);

        uint256 toPre = t.balanceOf(to);
        h.transferFromToken(address(t), from, to, spendAmt);
        assertEq(t.balanceOf(to) - toPre, spendAmt);
        assertEq(t.allowance(from, address(h)), approveAmt - spendAmt);
    }

    function prove_TransferFromErc20_MaxAllowanceNotDecrement(uint256 spendAmt) public {
        TokenUtilsHarness h = new TokenUtilsHarness();
        MockERC20 t = new MockERC20();
        address from = address(0xAABB);
        address payable to = payable(address(0xCCDD));
        uint256 minted = 1000 ether;
        t.mint(from, minted);

        vm.assume(spendAmt > 0 && spendAmt <= minted);

        vm.prank(from);
        t.approve(address(h), type(uint256).max);
        uint256 allowPre = t.allowance(from, address(h));

        h.transferFromToken(address(t), from, to, spendAmt);
        assertEq(t.allowance(from, address(h)), allowPre);
    }

    function prove_NoReturnToken_Supported(uint256 amt) public {
        TokenUtilsHarness h = new TokenUtilsHarness();
        NoReturnERC20 nr = new NoReturnERC20();
        nr.mint(address(h), 100 ether);
        vm.prank(address(h));
        nr.approve(address(h), type(uint256).max);

        address payable to = payable(address(0xCAFE));
        uint256 balH0 = nr.balanceOf(address(h));
        vm.assume(amt <= balH0);
        uint256 to0 = nr.balanceOf(to);
        bool ok = h.tryTransferToken(address(nr), to, amt);
        assertTrue(ok);
        assertEq(nr.balanceOf(to) - to0, amt);
        assertEq(balH0 - nr.balanceOf(address(h)), amt);
    }

    function prove_FalseReturnToken_TryTransferFalse(uint256 amt) public {
        TokenUtilsHarness h = new TokenUtilsHarness();
        FalseReturnERC20 f = new FalseReturnERC20();
        f.mint(address(h), 100 ether);
        vm.prank(address(h));
        f.approve(address(h), type(uint256).max);

        address payable to = payable(address(0xBEEF));
        uint256 balH0 = f.balanceOf(address(h));
        vm.assume(amt <= balH0);
        uint256 to0 = f.balanceOf(to);
        bool ok = h.tryTransferToken(address(f), to, amt);
        assertFalse(ok);
        assertEq(f.balanceOf(to), to0);
        assertEq(f.balanceOf(address(h)), balH0);
    }

    function prove_CheckBalance_IndexFirstSufficient(
        uint256 ethBal,
        uint256 t1Bal,
        uint256 t2Bal,
        uint256 need0,
        uint256 need1,
        uint256 need2
    ) public {
        TokenUtilsHarness h = new TokenUtilsHarness();
        MockERC20 t1 = new MockERC20();
        MockERC20 t2 = new MockERC20();

        // Seed balances
        vm.deal(address(h), ethBal);
        t1.mint(address(h), t1Bal);
        t2.mint(address(h), t2Bal);

        // (Optional) keep numbers sane to avoid giant traces
        vm.assume(ethBal <= 1e36 && t1Bal <= 1e36 && t2Bal <= 1e36);
        vm.assume(need0 <= 1e36 && need1 <= 1e36 && need2 <= 1e36);

        // Build an array where every non-zero token is a real ERC-20
        TokenAmount[] memory arr = new TokenAmount[](3);
        arr[0] = TokenAmount({token: address(0), amount: need0});
        arr[1] = TokenAmount({token: address(t1), amount: need1});
        arr[2] = TokenAmount({token: address(t2), amount: need2});

        uint256 idx = h.checkBalanceHarness(arr);

        // Expected: first index with sufficient balance, else n
        uint256 expect = 3;
        if (ethBal >= need0) expect = 0;
        else if (t1Bal >= need1) expect = 1;
        else if (t2Bal >= need2) expect = 2;

        assertEq(idx, expect);
    }

    function prove_TransferBalanceZero_NoOp() public {
        TokenUtilsHarness h = new TokenUtilsHarness();
        MockERC20 t = new MockERC20();
        address payable rec = payable(address(0x1234));

        uint256 erc20BeforeH = t.balanceOf(address(h));
        uint256 ethBeforeH = address(h).balance;
        uint256 recErc20Before = t.balanceOf(rec);
        uint256 recEthBefore = rec.balance;

        uint256 sweptErc20 = h.transferBalanceToken(address(t), rec);
        uint256 sweptEth = h.transferBalanceToken(address(0), rec);

        assertEq(sweptErc20, 0);
        assertEq(sweptEth, 0);
        assertEq(t.balanceOf(address(h)), erc20BeforeH);
        assertEq(address(h).balance, ethBeforeH);
        assertEq(t.balanceOf(rec), recErc20Before);
        assertEq(rec.balance, recEthBefore);
    }

    function prove_TryERC20_ZeroAmount_NoDelta() public {
        TokenUtilsHarness h = new TokenUtilsHarness();
        MockERC20 t = new MockERC20();
        t.mint(address(h), 100 ether);

        address payable to = payable(address(0xBEEF));
        uint256 toBefore = t.balanceOf(to);
        uint256 hBefore = t.balanceOf(address(h));

        bool ok = h.tryTransferToken(address(t), to, 0);
        assertTrue(ok);
        assertEq(t.balanceOf(to), toBefore);
        assertEq(t.balanceOf(address(h)), hBefore);
    }

    function prove_Erc20SafeTransfer_RevertsIfInsufficient(uint256 minted) public {
        TokenUtilsHarness h = new TokenUtilsHarness();
        MockERC20 t = new MockERC20();
        vm.assume(minted <= 1e36);
        t.mint(address(h), minted);

        address payable to = payable(address(0xA11CE));
        vm.expectRevert();
        h.transferToken(address(t), to, minted + 1);
    }

    function prove_TransferFromEth_Reverts(uint256 amt) public {
        TokenUtilsHarness h = new TokenUtilsHarness();
        address from = address(this);
        address payable to = payable(address(0xD00D));
        vm.expectRevert();
        h.transferFromToken(address(0), from, to, amt);
    }

    function prove_EthTransferToRejector_Reverts(uint256 amt) public {
        TokenUtilsHarness h = new TokenUtilsHarness();
        FailingReceiver rejector = new FailingReceiver();
        vm.deal(address(h), 100 ether);
        vm.assume(amt > 0 && amt <= address(h).balance);
        vm.expectRevert();
        h.transferToken(address(0), payable(address(rejector)), amt);
    }

    function prove_TransferFromErc20_AllowanceAccumulatesAcrossCalls(uint256 approveAmt, uint256 spend1, uint256 spend2)
        public
    {
        TokenUtilsHarness h = new TokenUtilsHarness();
        MockERC20 t = new MockERC20();
        address from = address(0xF00D);
        address payable to1 = payable(address(0xD00D));
        address payable to2 = payable(address(0xC0FFEE));

        uint256 minted = 1_000_000 ether;
        t.mint(from, minted);

        vm.assume(approveAmt > 0 && approveAmt <= minted);
        vm.assume(spend1 > 0 && spend1 <= approveAmt);
        vm.assume(spend2 > 0 && spend2 <= approveAmt - spend1);

        vm.prank(from);
        t.approve(address(h), approveAmt);

        uint256 to1Before = t.balanceOf(to1);
        uint256 to2Before = t.balanceOf(to2);

        h.transferFromToken(address(t), from, to1, spend1);
        assertEq(t.balanceOf(to1) - to1Before, spend1);
        assertEq(t.allowance(from, address(h)), approveAmt - spend1);

        h.transferFromToken(address(t), from, to2, spend2);
        assertEq(t.balanceOf(to2) - to2Before, spend2);
        assertEq(t.allowance(from, address(h)), approveAmt - spend1 - spend2);
    }

    function prove_FeeOnTransfer_DeliversLess(uint256 amt) public {
        TokenUtilsHarness h = new TokenUtilsHarness();
        FeeOnTransferERC20 ft = new FeeOnTransferERC20();
        ft.mint(address(h), 1_000_000 ether);
        vm.prank(address(h));
        ft.approve(address(h), type(uint256).max);

        address payable to = payable(address(0xBEEF));
        vm.assume(amt > 0 && amt <= ft.balanceOf(address(h)));
        // Ensure fee > 0 under integer division
        vm.assume(amt >= 100);

        uint256 toBefore = ft.balanceOf(to);
        bool ok = h.tryTransferToken(address(ft), to, amt);
        assertTrue(ok);
        uint256 delta = ft.balanceOf(to) - toBefore;
        assertLt(delta, amt);
    }
}
