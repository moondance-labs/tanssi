import { DevModeContext } from "@moonwall/cli";
import { KeyringPair } from "@moonwall/util";

export async function createBlockAndRemoveInvulnerables(context: DevModeContext, sudoKey: KeyringPair) {
    let nonce = (await context.polkadotJs().rpc.system.accountNextIndex(sudoKey.address)).toNumber();
    const invulnerables = await context.polkadotJs().query.invulnerables.invulnerables();

    const txs = invulnerables.map((invulnerable) =>
        context
            .polkadotJs()
            .tx.sudo.sudo(context.polkadotJs().tx.invulnerables.removeInvulnerable(invulnerable))
            .signAsync(sudoKey, { nonce: nonce++ })
    );

    await context.createBlock(txs);
}
