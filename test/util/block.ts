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