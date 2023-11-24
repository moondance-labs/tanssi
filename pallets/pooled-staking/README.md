# Pooled staking pallet

This pallet implements a Delegated Proof of Stake (DPoS) election system based
on a pool logic inspired from AMM Liquidity Pools, which provide computationally
efficient reward distribution.

## Pool design

[//]: # (SBP-M1 review: 'represents', consider currency -> asset, better term for 'users'?)
A pool represent an amount of currency shared among many users, whom own some amount of shares

[//]: # (SBP-M1 review: 'increases')
among a total share supply. Users can join or leave the pool, which both increase the total amount

[//]: # (SBP-M1 review: consider currency -> asset)
of shared currency and the supply of shares such that each share keeps the same value. Rewards or

[//]: # (SBP-M1 review: 'shareholders')
slashing are shared among all share holders by increasing/decreasing the total amount of shared

[//]: # (SBP-M1 review: consider currency -> asset, 'share amounts')
currency without changing the shares amounts or supply. This pool system can be used for any state a delegator can be in which they can receive rewards and/or be slashed.

[//]: # (SBP-M1 review: crate doc comments note 3 pools only?. Consider defining candidate, delegator terms.)
For each candidate there are 4 pools a delegator can be in:
- **Joining pool**: The delegator requested to start delegating for that candidate. However they must wait some
  
  [//]: # (SBP-M1 review: 'otherwise allow earning rewards')
  time before they are eligible to rewards, as it would otherwise allow to earn rewards for past sessions the

  [//]: # (SBP-M1 review: '...has elapsed...')
  delegator was not yet contributing to the election of the candidate. Once the joining delay is elapsed the
  delegator can convert their **joining shares** into **auto compounding shares** or **manual rewards shares**

  [//]: # (SBP-M1 review: 'conversion')
  (decided in advance so that anyone can trigger the convertion).
- **Auto compounding pool**: The delegator is eligible to rewards which are automatically compounded. This is
  done by increasing the total amount of stake backing the pool without changing the amount of shares owned, which indirectly increase the value of each share.
- **Manual rewards pool**: The delegator is eligible to rewards which are kept out of the pool. It is based

  [//]: # (SBP-M1 review: 'For each delegator we store the value of the counter...')
  around a counter of how much reward has been distributed per share since genesis. For each delegator is stored the value of the counter when they joined the pool or last claimed, such that it is possible to compute the amount of withdrawable rewards based on the amount of owned shares. Any change of the amount

  [//]: # (SBP-M1 review: '...requires force claiming of rewards...')
  of shares of a delegator (joining/leaving) requires to force claiming the rewards to keep the calculations
  correct.
- **Leaving pool**: The delegator requested to stop delegating for that candidate. However they are still
  accountable if the candidate is slashed until the end of the leaving delay. They no longer count towards
  the candidate score nor are eligible to rewards.

[//]: # (SBP-M1 review: consider currency -> asset)
## Held currency

To allow delegators to participate in other pallets such as democracy, their stake stays in their account and
is **held** by the staking pallet. However since reward distribution and slashing are made indirectly without
iterating over the set of delegators, the amount held in the account can mismatch the funds at stake. It means

[//]: # (SBP-M1 review: 'submit an extrinsic')
rewards are distributed to an account dedicated to the staking pallet, and delegators can then call an 

[//]: # (SBP-M1 review: 'transferred', 'a hold')
extrinsic to get their rewards transfered to their account (with an hold for auto compounding rewards).

[//]: # (SBP-M1 review: 'submit an extrinsic', consider currency -> asset)
For slashing, it requires anyone to call an extrinsic to transfer the slashed currency out of the
slashed delegators account.