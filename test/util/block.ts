import { DevTestContext } from "@moonwall/cli";

export async function jumpSessions(context: DevTestContext, count: Number): Promise<string | null> {
    const session = (await context.polkadotJs().query.session.currentIndex())
      .addn(count.valueOf())
      .toNumber();
  
    return jumpToSession(context, session);
}

export async function jumpToSession(context: DevTestContext, session: number): Promise<string | null> {
    let lastBlockHash = null;
    while (true) {
      const currentSession = (
        await context.polkadotJs().query.session.currentIndex()).toNumber();
      if (currentSession === session) {
        return lastBlockHash;
      } else if (currentSession > session) {
        return null;
      }

      lastBlockHash = (await context.createBlock()).block.hash.toString();
    }
}

export async function jumpBlocks(context: DevTestContext, blockCount: number) {
    while (blockCount > 0) {
      await context.createBlock();
      blockCount--;
    }
}

export async function waitSessions(context, paraApi: ApiPromise, count: Number): Promise<string | null> {
  const session = (await paraApi.query.session.currentIndex())
    .addn(count.valueOf())
    .toNumber();

  return waitToSession(context, paraApi, session);
}

export async function waitToSession(context, paraApi: ApiPromise, session: number): Promise<string | null> {
  while (true) {
    const currentSession = (
      await paraApi.query.session.currentIndex()).toNumber();
    if (currentSession === session) {
      const signedBlock = await paraApi.rpc.chain.getBlock();
      return signedBlock.block.header.hash;
    } else if (currentSession > session) {
      return null;
    }

    await context.waitBlock(1, "Tanssi");
  }
}

/// Same as tx.signAndSend(account), except that it waits for the transaction to be included in a block:
/// 
/// ```
/// const txHash = await tx.signAndSend(alice);
/// // We don't know if the transaction has been included in a block or not
/// const { txHash, blockHash } = await signAndSendAndInclude(tx, alice);
/// // We know the blockHash of the block that includes this transaction
/// ```
export function signAndSendAndInclude(tx, account): Promise<{txHash, blockHash, status}> {
  return new Promise((resolve, reject) => {
    const unsub = tx.signAndSend(account, ({status, txHash}) => {
      if (status.isInBlock) {
        resolve({
          txHash,
          blockHash: status.asInBlock,
          status,
        });
      }
    });
  });
}
