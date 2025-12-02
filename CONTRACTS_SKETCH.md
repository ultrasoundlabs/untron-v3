# High-level picture

Untron is a two-chain protocol:

- On Tron:
  - `UntronController` manages a huge address space of `UntronReceiver`s (CREATE2 contracts).
  - Users send Tron tokens (mainly USDT) to those receivers.
  - Periodically, the controller dumps receivers and bridges funds to Arbitrum.
- On Arbitrum:
  - `UntronManager` keeps track of leases on those receivers.
  - Each lease defines:
    - who gets paid when deposits hit a given receiver,
    - on what terms (fee, duration),
    - and where/how they want to receive value (swap/bridge via swappers).

Relayers glue it together:
- Proving Tron txs to a Tron light client on Arbitrum.
- Calling `UntronManager` to:
  - pre-entitle deposits (instant credits),
  - process dumps (backing + opaque volume),
  - and drive the claim queue.

Liquidity for instant payouts comes from a fast-fill vault (USDT on Arbitrum), initially mostly team-funded, possibly LP-augmented later.

---

# Components on Tron

## UntronController

A Tron smart contract that:
- Deploys `UntronReceiver`s via CREATE2.
- Sweeps funds from receivers and bridges to Arbitrum.
- Restricts how receivers can move tokens (they are only callable by the controller).

### Functional responsibilities:

1. **Receiver deployment**
   - `deployReceiver(bytes32 salt)`:
     - Computes the receiver address from `(controller, salt, initCodeHash)`.
     - Deploys a minimal receiver contract at that address via CREATE2.
   - In practice:
     - Realtors precompute many salts off-chain (often vanity salts so receiver addresses end with e.g. `…Untron`).
     - Any realtor can reuse any salt, subject to lease rules on Arbitrum.

2. **Pulling tokens from receivers**
   - `pullFromReceivers(address token, bytes32[] receiverSalts, uint256[] amounts)`:
     - `amounts[]` is **caller-supplied calldata** and is interpreted as the per-salt sweep amount that the caller claims will be moved from each receiver.
     - For each `salt[i]`:
       - Compute `receiver = addressFromSalt(salt[i])`.
       - Read `balance = token.balanceOf(receiver)`.
       - Define the actual sweep amount:
         - If `balance <= 1`: `sweepAmount = 0`.
         - If `balance > 1`: `sweepAmount = balance - 1`.
       - Enforce:
         - `require(amounts[i] == sweepAmount)`.
       - If `sweepAmount > 0`:
         - Call the receiver to transfer exactly `sweepAmount` from the receiver to the controller.
         - Accumulate `amountDumped[salt[i]] = sweepAmount`.
     - Sum a total `totalDumped = Σ_i amounts[i]` for this `(token, salts[])`.
       - Note: `totalDumped` is **fully determined by calldata**, but is also fully checked against live balances on Tron; any mismatch causes a revert.
       - Due to the `balance - 1` rule, exactly `1` smallest unit of `token` is always left behind in each non-empty receiver after a dump.

3. **Bridging tokens from controller**
   - `bridge(address token, address bridger, uint256 inAmount, uint256 outAmount)`:
     - Delegatecall into an `IUntronBridger` implementation:
       - `bridger.bridge(token, inAmount, outAmount, ...)`.
     - Bridger is responsible for performing whatever bridging/swapping on Tron is needed so that:
       - `outAmount` of Arbitrum USDT ends up in `UntronManager` on Arbitrum.
     - If bridger’s computed output amount ≠ `outAmount`, the whole transaction reverts.
     - Combined with `pullFromReceivers`, this gives us a clean, verifiable `outAmount` on Tron, derived from calldata-but-checked `amounts[]`, plus a known USDT inflow on Arbitrum.

4. **Executor mechanism (for future features)**
   - There is an executor Tron address settable by the controller owner:
     - If `executor == 0x00`, controller cannot send funds anywhere itself, only through bridgers via `bridge` function.
     - If `executor != 0x00`, executor address can make controller send any of its funds anywhere using `transferFromController` function.
   - `transferFromController(address token, address recipient, uint256 amount)`:
     - Enforce: `require(msg.sender == executor)`
     - Transfer `amount` of `token` (native token if `token == 0x00`; ERC-20 otherwise) to `recipient`.
   - It's not going to be used from day 1 and is left as a future-proofness measure for potential future protocols for swapping from EVM chains to Tron.

---

## UntronReceiver

A minimal, single-purpose CREATE2 contract:
- Deployed only by `UntronController`.
- It is only callable by its controller; all token movements are initiated by `UntronController`.
- The controller always routes receiver funds back to itself (via `pullFromReceivers`).
- The receiver itself cannot:
  - approve other addresses,
  - initiate transfers on its own or accept arbitrary callers,
  - run custom logic.

### Identity model:

- Protocol uses salt as the canonical identifier:
  - `receiverSalt` is a 32-byte value.
  - Address is `receiver = f(controller, salt, receiverInitCodeHash)`.
- This lets:
  - Real-world software treat salts as “receiver IDs”.
  - The protocol reason about “silent deployments”: users can send Tron USDT to the future address before the receiver contract is deployed; the controller can CREATE2 it later and dump the balance.

---

## IBridger

An interface for stateless bridger plugins on Tron.
- Called via DELEGATECALL from `UntronController`, so:
  - `IBridger` must be stateless (no own storage).
  - Execution context is the controller’s storage/permissions.

Single main function conceptually:
- `bridge(address token, uint256 inAmount, bytes calldata payload) external returns (uint256 outAmount)`:
  - Performs:
    - token swaps on Tron if needed,
    - a call into some underlying bridge,
    - ensures that Arbitrum USDT arrives in `UntronManager` on Arbitrum.
  - Must return:
    - the effective output amount from bridge (in `outAmount`).
    - If bridging was unsuccessful, it must revert (which reverts the `pullFromReceivers` as well).

The controller owner:
- Whitelists bridger contracts per token.
- Relayers, when calling `pullFromReceivers`, can only target whitelisted bridgers for the token.

Design assumption we’re going with:
- All bridging ultimately produces Arbitrum USDT.
- All protocol accounting on Arbitrum is in Arbitrum USDT units.

---

# Components on Arbitrum

## Tron Light Client

A smart contract that holds Tron headers and lets anyone:
- Prove:
  - “Tx X is included in Tron block B with timestamp T_tron”
  - “Tx X had `success=true`”
  - “Here is the calldata of X”
- Optionally, in future:
  - Use a configurable confirmation depth `k` so `UntronManager` only accepts blocks older than `latestHeader - k`.

Initial assumption:
- Tron has never had a reorg historically.
- v1 can treat every included tx as final (effectively `k=0`).
- Contracts can still be written to support a `k` parameter in case the world changes.

The light client does not prove Tron state roots, only tx inclusion + metadata. All reasoning about “who received what” is done from:
- `contractAddress`,
- `calldata`,
- `success=true`.

---

## UntronManager

The Arbitrum contract that:
- Tracks leases on `(receiverSalt, tronToken)` pairs.
- Converts Tron events into USDT claims for lessees.
- Manages the claim queue and payout via swappers.
- Manages the fast-fill vault and LP deposits.

---

# Leases and realtors

## Leases

For each `(receiverSalt S, tronToken T)`, `UntronManager` maintains a timeline of leases.

Each lease `L` has:
- **realtor** – the address that created it.
- **lessee** – the address that is economically entitled to flow.
- **startTime** – Arbitrum `block.timestamp` at lease creation.
- **nukeableAfter** – Arbitrum timestamp after which anyone is allowed to create a new lease for this salt.
- **leaseFee_bps** – immutable fee in basis points (e.g. 25 = 0.25%).
  - Must satisfy:
    ```
    leaseFee_bps >= max(protocolFloor_T, realtorMin_T[realtor])
    ```
  - No upper limit: if users accept 5% for a weird flow, the protocol doesn’t enforce a ceiling.

- Optional **flatFee** (in USDT units) applied per claim (v1 can just use 0).
- **Payout config** (fully mutable by lessee):
  - swapper address (must be whitelisted),
  - target chainId and token (semantics are swapper-specific),
  - beneficiary (final address to receive tokens / bridged funds).

### Lease validity over time:

- A lease has only `startTime` and `nukeableAfter`; no explicit on-chain endTime.
- Economically:
  - The lease continues to “own” deposits until a new lease is created for the same `(S,T)` pair.
- Permissions:
  - Before `nukeableAfter`:
    - No one may create a new lease for that `(S,T)`.
  - After `nukeableAfter`:
    - Any realtor may create a new lease for `(S,T)` (salt recycling).
- Deposit assignment:
  - A Tron event with timestamp `T_tron` is assigned to the last lease `L` on `(S,T)` such that:
    ```
    L.startTime <= T_tron
    ```

This supports:
1. **Checkout flow**
   - Short-lived leases (10–30 minutes).
   - Lower `leaseFee`.
   - Salts are recycled aggressively; a small set of salts can be reused many times.
2. **Deposit flow**
   - Long-lived leases (months, year).
   - Higher `leaseFee`.
   - Realtor-specific limits (min fee, max duration) make economics sustainable.

**Stray deposits** (e.g. someone reuses an old address after its intended lifetime):
- If they arrive after `nukeableAfter` but before a new lease is created:
  - They are still credited to the previous lease (since it is still the last one with `startTime <= T_tron`).
  - The protocol doesn’t try to solve “who should be refunded”; it just credits the current lease owner at that time.

---

## Realtors

Realtors are Arbitrum addresses with privileges in `UntronManager`. They can:
- Create leases for any `(receiverSalt, tronToken)` pair that:
  - Has never been leased, or
  - Whose last lease is past `nukeableAfter`.

Per realtor and per token `T`, `UntronManager` can store:
- `realtorMinFee_T[realtor]` – realtor-specific minimum fee.
- Possibly `realtorMinFlat_T[realtor]` or `realtorMaxDuration_T[realtor]` etc., configured by the team.

Protocol-level per-token config:
- `protocolFloor_T` – global minimum fee (bps) for that token.

When creating a lease, realtor chooses:
- `leaseFee_bps` such that:
  ```
  leaseFee_bps >= max(protocolFloor_T, realtorMinFee_T[realtor])
  ```
- `nukeableAfter` (subject to realtor-specific / protocol-specific bounds).

Economically:
- The entire percentage fee `leaseFee` is the user’s fee.
- The protocol’s actual profit/loss per unit depends on:
  - `leaseFee` chosen by the realtor, and
  - effective bridge fee `f_bridge` from the `IUntronBridger` used in practice.

There is no enforced on-chain spread-credit for the realtor; their economics live off-chain (e.g. offering rates to users).

---

## Raw volume accounting per lease

For each lease `L` on `(S,T)`:
- `recognizedRaw_L` – total raw token volume that has been assigned to this lease, from:
  - pre-entitled deposits (recognizable transfer / transferFrom patterns),
  - opaque volume from dumps (remaining after repaying older leases).
- `backedRaw_L` – how much of `recognizedRaw_L` has been backed by `pullFromReceivers` events.
- `unbackedRaw_L = recognizedRaw_L - backedRaw_L` – how much volume was already promised to this lease (claims created, possibly even filled) but not yet matched by dumps from Tron.

Global guards:
- `depositProcessed[tronTx, logIndex]` – to ensure each recognizable deposit is used at most once.
- `dumpProcessed[tronTx]` – to ensure each dump is processed only once per `(S,T)`.

This structure ensures:
- You never over-credit raw volume:
  Across all leases and all time, the total `recognizedRaw` is constrained by:
  - pre-entitlement logic (which only triggers on safe patterns), plus
  - the sum of `amountDumped` values, plus at most `1` smallest unit of `T` per `(receiverSalt, T)`, due to the `balance - 1` sweep rule:
    - Each `(receiverSalt, T)` that ever holds a positive balance can end up with a permanent `1`-unit dust that is never swept.
    - The protocol treats this as a tiny, bounded negative PnL (covered by team/fee-vault capital), not as user-facing loss.

---

## Pre-entitlement (instant credit) on Arbitrum

When a relayer proves a recognizable deposit on Tron:
- Example: TRC20 USDT `transfer(to = receiver, amount = Q)` with `success=true`.

Steps in `UntronManager`:

1. If `depositProcessed[tx, logIndex]` → ignore.
2. Mark `depositProcessed[tx, logIndex] = true`.
3. Determine the lease `L` on `(S,T)`:
   - `S = receiverSalt`,
   - `T = tronToken`,
   - `T_tron` = Tron block timestamp.
   - `L` = last lease s.t. `L.startTime <= T_tron`.
4. Compute lessee’s USDT claim:
   - Percentage:
     ```
     percentageOut = Q * (1 - leaseFee_L)
     ```
   - Flat fee (per claim, optional; currently often 0):
     ```
     netOut = max(0, percentageOut - flatFee_L)
     ```
5. Append to global FIFO claim queue:
   ```
   claims.push({ amountUSDT: netOut, leaseId: L })
   ```
6. Update raw volume:
   ```
   recognizedRaw_L += Q
   unbackedRaw_L   += Q
   ```

- Lessee is now entitled to that USDT amount.
- Fast-fill vault may later pay this via `fill()` even before any dump happens.

NOTE: The set of recognizable patterns is pluggable. v1 can start with trivial:
- `transfer(to=receiver)`
- `transferFrom(..., to=receiver)`

and later extend via a matcher contract, without changing high-level semantics.

---

## pullFromReceivers processing (backing + opaque volume)

When a relayer proves a `pullFromReceivers` tx for `(receiverSalt S, tronToken T)` with `amountDumped = D`:

1. If `dumpProcessed[tx]` → ignore.
2. Mark `dumpProcessed[tx] = true`.
3. Let `dumpTime = T_tron` (dump’s Tron block timestamp).
4. Initialize:
   ```
   remaining = D
   ```
5. Repay historical unbacked volume across leases for `(S,T)`
   - Consider all leases `{L_old}` on `(S,T)` with `L_old.startTime <= dumpTime`, ordered by startTime ascending.
   - For each such `L_old`:
     ```
     repay = min(unbackedRaw_L_old, remaining)
     backedRaw_L_old   += repay
     unbackedRaw_L_old -= repay
     remaining         -= repay
     ```
   - Stop when `remaining == 0` or all those leases have `unbackedRaw_L_old == 0`.

   **Interpretation:**
   - This part of `D` backs volume that was already pre-entitled (LP exposure from past).
   - It doesn’t create any new claims; it just reconciles backing vs recognized volume.

6. If any raw volume remains (`remaining > 0`):
   - Let `L_current` be the lease on `(S,T)` that is active at `dumpTime` (last lease with `startTime <= dumpTime`).
   - Treat `remaining` as new opaque volume for `L_current`, entirely under this dump:
     ```
     recognizedRaw_L_current += remaining
     backedRaw_L_current     += remaining
     ```
   - Create a new claim:
     ```
     percentageOut = remaining * (1 - leaseFee_L_current)
     netOut        = max(0, percentageOut - flatFee_L_current)
     claims.push({ amountUSDT: netOut, leaseId: L_current })
     ```

This ensures:
- Old leases’ pre-entitlements are “repaid” first whenever dumps happen.
- Any surplus raw volume in a dump (beyond the sum of old unbacked volume) becomes opaque profit for the lease active at dump time.

**Salt reuse is fine:**
- Older leases for that salt/token may still have `unbackedRaw > 0` when a new lease starts.
- Future dumps:
  - First repay old leases’ unbacked volume.
  - Only then give surplus to the new lease.

---

## Claim queue and payout (fill())

There is one global FIFO claim queue:
- Each claim:
  ```
  { amountUSDT, leaseId }
  ```

**Payout logic:**
- Anyone can call `fill()`.

Pseudo-flow:
```
while there is a next claim C at queue head:
    if usdtBalance < C.amountUSDT:
        break

    let L = C.leaseId

    // read live payout config
    (swapper, targetChain, targetToken, beneficiary) = L.currentPayoutConfig()

    // send USDT to swapper & call it
    transfer USDT: C.amountUSDT -> swapper
    call swapper.handlePayout(C.amountUSDT, targetChain, targetToken, beneficiary)

    if call reverts:
        revert fill()  // queue stays stuck on this claim

    else:
        mark C as filled
        pop C
        continue
```

**Key properties:**
- Claims do not snapshot payout config; they use the lease’s current config when filled.
- If a swapper is misconfigured/broken and consistently reverts:
  - The queue stops at that claim.
  - Once the lessee/team fixes the swapper or payout config, `fill()` resumes and that claim (and all later ones) starts flowing.
- As long as there is enough USDT, claims are always processed strictly in FIFO order.

---

## Fast-fill vault and LPs

All protocol USDT sits in `UntronManager`’s balance. On top of that:
- For each LP address:
  - `lpPrincipal[lp]` – how much USDT they have deposited minus withdrawn.
  - `usdtBalance` – current USDT token balance of `UntronManager`.

### Deposits
- LP calls `deposit(amount)`:
  - Transfers `amount` USDT into `UntronManager`.
  - `lpPrincipal[lp] += amount`.

No APY, no share price; just principal tracking.

### Claim fills
- When `fill()` succeeds for a claim of `C.amountUSDT`:
  - `usdtBalance` decreases by `C.amountUSDT`.
  - `lpPrincipal` is not changed here.
  - Economically:
    - LP + team are fronting user withdrawal before the matching dump/bridge.

### Dumps / bridging
- `pullFromReceivers` + `IUntronBridger` on Tron send Arbitrum USDT into `UntronManager`:
  - `usdtBalance` increases by `bridgedAmount`.
  - The difference between:
    - what was fronted via `fill()`, and
    - what bridging actually returns
  - is the effective bridge PnL (absorbed by the fee vault/team side, not by LP principal).

### Withdrawals

When an LP calls `withdraw(amount)`:

1. Require:
   ```
   amount <= lpPrincipal[lp]
   amount <= usdtBalance
   ```

2. If both hold:
   - `lpPrincipal[lp] -= amount`
   - `usdtBalance -= amount`
   - Transfer `amount` USDT to `lp`.

**Consequences:**
- LPs can always withdraw at 1:1 against their recorded principal if there is enough USDT in the contract at that moment.
- If `usdtBalance` is temporarily too low (e.g. heavy negative PnL or many claims filled before dumps):
  - Withdraw reverts.
  - LP must wait until:
    - more dumps/bridging refill USDT, or
    - the team deposits bailout USDT.
- Among LPs, withdrawals are effectively:
  - “whoever sends the withdrawal tx when there’s enough USDT”, not ordered by deposit time.

The team can treat part/all of LP principal as their own capital in v1, and later choose whether/when to let external LPs in under these semantics.

---

## Swappers

Swappers are Arbitrum contracts that:
- Receive USDT from `UntronManager` during `fill()`.
- Turn that USDT into:
  - another dollar stable (USDC, USDT.e, etc.) at effective 1:1 (subsidized by the team if necessary),
  - and optionally bridge it to another chain using a mechanism like CCTP / OFT.

**Important constraints:**
- Swappers are whitelisted by the protocol.
- Only stablecoins are allowed as ultimate user-facing tokens (by design choice).
- Swappers do not change claim amounts:
  - Claims are always denominated in Arbitrum USDT.
  - Swappers just define the “USDT → final asset” conversion path.

**Typical flow for a claim:**
1. `fill()` sends `amountUSDT` to swapper.
2. Swapper:
   - swaps USDT → desired stable (if needed),
   - optionally calls some bridge to move it to targetChain,
   - arranges for beneficiary to receive that stable on the target chain.

If a swapper reverts:
- `fill()` reverts and queue halts until the swapper/config is fixed.
- Once fixed, all old and new claims for leases using that swapper resume.

---

## Bridging risk and economics

For each raw amount `Q` in Tron USDT assigned to a lease with `leaseFee = f_lease`:
- User / lessee claim:
  `Q * (1 - f_lease)` (minus optional flat fee).
- Bridger actually delivers:
  - `Q * (1 - f_bridge)` USDT on Arbitrum.

Protocol’s per-unit economic result is:
```
PnL = Q * (f_lease - f_bridge)
```

- If `f_lease > f_bridge`: protocol/fee vault profits.
- If `f_lease < f_bridge`: protocol loses; that deficit is effectively covered by:
  - the fee vault, and
  - ultimately team deposits into fast-fill vault if needed.

The protocol mitigates this risk by:
- Having per-token fee floors (`protocolFloor_T`).
- Having per-realtor fee/duration limits, so “trusted” realtors can run longer, lower-fee leases, while others are constrained.
- Keeping lease fees fixed per lease:
  - `leaseFee` is chosen at creation and never changes.
- Future changes to bridging economics can be reflected by:
  - updating `protocolFloor_T` for new leases,
  - adjusting realtor-specific minima / lease durations.

Old leases remain on their original economics until replaced, but their exposure is naturally bounded by their duration and by how much flow arrives during their lifetime.
