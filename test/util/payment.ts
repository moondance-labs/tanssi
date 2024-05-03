import { bnToU8a, stringToU8a } from "@polkadot/util";
import { blake2AsU8a } from "@polkadot/util-crypto";
import { ApiPromise } from "@polkadot/api";

// Tank account is blake2(b"modlpy/serpayment" + parahain ID)
export function paraIdTank(paraId: any): any {
    const seedBytes = stringToU8a("modlpy/serpayment");
    const paraIdBytes = bnToU8a(paraId, { bitLength: 32 });
    const combinedBytes = new Uint8Array(seedBytes.length + paraIdBytes.length);
    combinedBytes.set(seedBytes);
    combinedBytes.set(paraIdBytes, seedBytes.length);
    const para_tank = blake2AsU8a(combinedBytes, 256);
    return para_tank;
}

export async function hasEnoughCredits(
    paraApi: ApiPromise,
    paraId: ParaId,
    blocksPerSession: bigint,
    // TODO: minSessionRequirement should be 2 if the chain had collators in the previous session, and 1 otherwise
    minSessionRequirement: bigint,
    costPerSession: bigint,
    costPerBlock: bigint
): Promise<boolean> {
    const existentialDeposit = await paraApi.consts.balances.existentialDeposit.toBigInt();

    const freeBlockCredits = (await paraApi.query.servicesPayment.blockProductionCredits(paraId)).unwrap().toBigInt();

    const freeSessionCredits = (await paraApi.query.servicesPayment.collatorAssignmentCredits(paraId))
        .unwrap()
        .toBigInt();

    // We need, combined, at least credits for 2 session coverage + blocks
    const neededBlockPaymentAfterCredits =
        minSessionRequirement * blocksPerSession - freeBlockCredits < 0n
            ? 0n
            : minSessionRequirement * blocksPerSession - freeBlockCredits;
    const neededCollatorAssignmentPaymentAfterCredits =
        minSessionRequirement - freeSessionCredits < 0n ? 0n : minSessionRequirement - freeSessionCredits;

    if (neededBlockPaymentAfterCredits > 0n || neededCollatorAssignmentPaymentAfterCredits > 0n) {
        const neededTankMoney =
            existentialDeposit +
            neededCollatorAssignmentPaymentAfterCredits * costPerSession +
            neededBlockPaymentAfterCredits * costPerBlock;
        const tankBalance = (await paraApi.query.system.account(paraIdTank(paraId))).data.free.toBigInt();
        if (tankBalance >= neededTankMoney) {
            return true;
        } else {
            return false;
        }
    } else {
        return true;
    }
}
