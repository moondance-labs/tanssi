# P2P Port Conflicts in zombienet (libp2p & litep2p)

Summary of a weird issue when switching form libp2p to litep2p.

## TL;DR

- Both libp2p and litep2p create sockets with `SO_REUSEPORT`. Multiple processes can bind and listen on the same TCP port.
- When two nodes share the same port, incoming connections are load-balanced randomly across them. Handshakes then can fail (if using the wrong node key) and peers get banned.
- In zombienet tests:
    - libp2p: succeeds 50% of the time (luck).
    - litep2p: always fails because it reuses the inbound port for outbound dials (self-connect edge case).
- In any case, there are no logs that can help debug those cases. A helper script exists in `pnpm net-ports`.

## Backstory

litep2p is becoming the default in stable2506 upgrade. All of our zombienet tests work, expect 2 suites that consistently timeout in CI.

```
pnpm moonwall test zombie_data_preservers_embedded_dancebox
pnpm moonwall test zombie_data_preservers_remote_dancebox
```

The timeouts seem to be related to syncing issues, and transaction inclusion timeout.

* The test suites work locally, only failing sporadically.
* There is a dancelight suite that works fine, `zombie_data_preservers_embedded_dancelight`
* Using libp2p instantly fixes the tests.

We notice that collators always have 1 peer when using litep2p, and 2 peers when using libp2p.
So we start to investigate. In the end it turns out to be a weird issue related to port reuse.

## To reproduce

Two minimal test suites can be used to reproduce the issue. They start two validators listening on the same P2P port (30333):

```bash
pnpm moonwall test zombie_tanssi_relay_p2p_port_conflict_libp2p
pnpm moonwall test zombie_tanssi_relay_p2p_port_conflict_litep2p
```

- libp2p: works 50% of the time (gets lucky because there are only 2 nodes on that port, so 50% chance of dialing the correct one).
- litep2p: always fails.

---

## Why litep2p always fails

- litep2p binds its outbound dials to the same local port it uses for inbound.
- With both processes listening on `127.0.0.1:30333`, litep2p ends up attempting a connection from `127.0.0.1:30333` to `127.0.0.1:30333`. 
  This is a special case that never works (self-connect), no messages can be sent or received.
- That behavior can be disabled by setting `reuse_port: bool` to false, which would use a random port for outbound connections, same as libp2p, but that flag is deep inside polkadot-sdk and we can't modify
  it from tanssi.

Note: Using an address different from `127.0.0.1` (e.g., public/bridge/container) would avoid the self‑connect edge-case and we would see the same behavior as in libp2p.

---

## Why libp2p seems to work

Using a helper script to inspect connections between processes:

```bash
pnpm moonwall run zombie_tanssi_relay_p2p_port_conflict_libp2p
pnpm net-ports connections-between --names "tanssi-node,tanssi-relay,polkadot"

> test@1.0.0 net-listeners /home/tomasz/projects/tanssi/test
> tsx scripts/net-listeners.ts "connections-between" "--names" "tanssi-node,tanssi-relay,polkadot"

PID 3541053: tanssi-relay  netns=net:[4026531840]
  Listening (IPv4):
    - 0.0.0.0:9947
    - 0.0.0.0:30333
    - 0.0.0.0:38309

PID 3541166: tanssi-relay  netns=net:[4026531840]
  Listening (IPv4):
    - 0.0.0.0:30333
    - 0.0.0.0:34629
    - 0.0.0.0:37089

Direct TCP/IPv4 connections among provided PIDs:
  [1] 3541053(tanssi-relay) 127.0.0.1:30333  <--ESTABLISHED/ESTABLISHED-->  127.0.0.1:60428  3541166(tanssi-relay)
       (A inode=44473474, B inode=44480557)
```

Even though both processes listen on `30333`, the active connection uses an ephemeral port (`60428`) on one side.
This is the main difference with litep2p, which would try to use `30333` as the outbound port and connect from `30333` to `30333`, which never works.

---

## Why only tanssi-node collators are affected

tanssi-node collators are the most affected because their P2P port is not randomized by zombienet. A more complex test you can run:

```bash
pnpm moonwall test zombie_tanssi_collator_peers
```

This will inspect node logs to check that all ports are unique, so it will fail because both Container-2000 collators try to listen on 30335.

tanssi-relay and tanssi-node solo-chain are less affected because they get a random p2p port from zombienet. tanssi-node collators do not get it because they expect 3 sets of args (separated by `--`), and zombienet only provides 2 sets of args (parachain and relaychain).

But even tanssi-relay is affected when the port is manually set to a conflicting port, see `pnpm moonwall test zombie_tanssi_relay_p2p_port_conflict_litep2p`

---

# Why most test suites seem to work fine

This bug only affects the connections between collators, so connections from collators to the full node still work.
As long as the full node doesn't ban a collator, the network will work fine.

We have observed consistent timeouts in CI in these test suites, which almost always work locally:

```
pnpm moonwall test zombie_data_preservers_embedded_dancebox
pnpm moonwall test zombie_data_preservers_remote_dancebox
```

The exact cause for the timeout is unknown but it could be related to using only 1 full node to connect all the collators between them.

---

## Other port used errors

If the chosen P2P port is already taken by a process that does not allow `SO_REUSEPORT`, nodes will continue running but without any inbound listener, so they will be able to connect to the full node but unable to connect to each other. This is the error you see in logs in that case:

* libp2p
```text
[Container-2000] Can't listen on /ip6/::/tcp/30335/ws because: Other(Custom { kind: Other, error: Custom { kind: Other, error: Other(Left(Left(Left(Left(Left(Transport(Transport(Os { code: 98, kind: AddrInUse, message: "Address already in use" })))))))) } })
[Container-2000] Can't listen on /ip4/0.0.0.0/tcp/30335/ws because: Other(Custom { kind: Other, error: Custom { kind: Other, error: Other(Left(Left(Left(Left(Left(Transport(Transport(Os { code: 98, kind: AddrInUse, message: "Address already in use" })))))))) } })
```

* litep2p
```text
litep2p started with no listen addresses, cannot accept inbound connections
```

---

## Helper utilities

Check out

```
pnpm net-ports --help
```

to see examples on how to use it, and make it easier to detect and debug similar issues with ports and nodes not connecting to each other.
