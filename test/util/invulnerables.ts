import { DevModeContext } from "@moonwall/cli";

export async function createBlockAndRemoveInvulnerables(context: DevModeContext) {
    let nonce = (await context.polkadotJs().rpc.system.accountNextIndex(context.keyring.alice.address)).toNumber();

    await context.createBlock([
        context
            .polkadotJs()
            .tx.sudo.sudo(context.polkadotJs().tx.invulnerables.removeInvulnerable(context.keyring.alice.address))
            .signAsync(context.keyring.alice, { nonce: nonce++ }),
        context
            .polkadotJs()
            .tx.sudo.sudo(context.polkadotJs().tx.invulnerables.removeInvulnerable(context.keyring.bob.address))
            .signAsync(context.keyring.alice, { nonce: nonce++ }),
        context
            .polkadotJs()
            .tx.sudo.sudo(context.polkadotJs().tx.invulnerables.removeInvulnerable(context.keyring.charlie.address))
            .signAsync(context.keyring.alice, { nonce: nonce++ }),
        context
            .polkadotJs()
            .tx.sudo.sudo(context.polkadotJs().tx.invulnerables.removeInvulnerable(context.keyring.dave.address))
            .signAsync(context.keyring.alice, { nonce: nonce++ }),
    ]);
}
