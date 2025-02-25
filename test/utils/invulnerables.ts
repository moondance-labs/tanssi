import type { DevModeContext } from "@moonwall/cli";
import type { KeyringPair } from "@moonwall/util";
import type { SubmittableExtrinsic } from "@polkadot/api-base/types";
import type { ISubmittableResult } from "@polkadot/types/types";

export async function createBlockAndRemoveInvulnerables(
    context: DevModeContext,
    sudoKey: KeyringPair,
    isTanssiInvulns = false
) {
    let txs: Promise<SubmittableExtrinsic<"promise", ISubmittableResult>>[];
    let nonce = (await context.polkadotJs().rpc.system.accountNextIndex(sudoKey.address)).toNumber();

    if (isTanssiInvulns) {
        const invulnerables = await context.polkadotJs().query.tanssiInvulnerables.invulnerables();
        txs = invulnerables.map((invulnerable) =>
            context
                .polkadotJs()
                .tx.sudo.sudo(context.polkadotJs().tx.tanssiInvulnerables.removeInvulnerable(invulnerable))
                .signAsync(sudoKey, { nonce: nonce++ })
        );
    } else {
        const invulnerables = await context.polkadotJs().query.invulnerables.invulnerables();
        txs = invulnerables.map((invulnerable) =>
            context
                .polkadotJs()
                .tx.sudo.sudo(context.polkadotJs().tx.invulnerables.removeInvulnerable(invulnerable))
                .signAsync(sudoKey, { nonce: nonce++ })
        );
    }

    await context.createBlock(txs);
}
