import "@tanssi/api-augment";
import { bnToU8a, stringToU8a } from "@polkadot/util";
import { blake2AsU8a } from "@polkadot/util-crypto";
import type { ApiPromise } from "@polkadot/api";
import type { ParaId } from "@polkadot/types/interfaces";
import type { ApiDecoration } from "@polkadot/api/types";

// Tank account is blake2(b"modlpy/serpayment" + parahain ID)
export function paraIdTank(paraId: number) {
    const seedBytes = stringToU8a("modlpy/serpayment");
    const paraIdBytes = bnToU8a(paraId, { bitLength: 32 });
    const combinedBytes = new Uint8Array(seedBytes.length + paraIdBytes.length);
    combinedBytes.set(seedBytes);
    combinedBytes.set(paraIdBytes, seedBytes.length);
    const para_tank = blake2AsU8a(combinedBytes, 256);
    return para_tank;
}

export async function hasEnoughCredits(
    paraApi: ApiPromise | ApiDecoration<"promise">,
    paraId: ParaId | number | string,
    blocksPerSession: bigint,
    // TODO: minSessionRequirement should be 2 if the chain had collators in the previous session, and 1 otherwise
    minCollatorSessionRequirement: bigint,
    minBlockSessionRequirement: bigint,
    costPerSession: bigint,
    costPerBlock: bigint
): Promise<boolean> {
    const paraIdNumber =
        typeof paraId === "number" ? paraId : typeof paraId === "string" ? Number.parseInt(paraId) : paraId.toNumber();
    const existentialDeposit = paraApi.consts.balances.existentialDeposit.toBigInt();

    const freeBlockCredits = (await paraApi.query.servicesPayment.blockProductionCredits(paraIdNumber))
        .unwrap()
        .toBigInt();

    const freeSessionCredits = (await paraApi.query.servicesPayment.collatorAssignmentCredits(paraIdNumber))
        .unwrap()
        .toBigInt();

    // We need, combined, at least credits for 2 session coverage + blocks
    const neededBlockPaymentAfterCredits =
        minBlockSessionRequirement * blocksPerSession - freeBlockCredits < 0n
            ? 0n
            : minBlockSessionRequirement * blocksPerSession - freeBlockCredits;
    const neededCollatorAssignmentPaymentAfterCredits =
        minCollatorSessionRequirement - freeSessionCredits < 0n
            ? 0n
            : minCollatorSessionRequirement - freeSessionCredits;

    if (neededBlockPaymentAfterCredits > 0n || neededCollatorAssignmentPaymentAfterCredits > 0n) {
        const neededTankMoney =
            existentialDeposit +
            neededCollatorAssignmentPaymentAfterCredits * costPerSession +
            neededBlockPaymentAfterCredits * costPerBlock;
        const tankBalance = (await paraApi.query.system.account(paraIdTank(paraIdNumber))).data.free.toBigInt();
        if (tankBalance >= neededTankMoney) {
            return true;
        }
        return false;
    }
    return true;
}
