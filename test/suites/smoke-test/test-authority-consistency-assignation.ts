import { describeSuite, expect, beforeAll } from "@moonwall/cli";
import { getBlockTime } from "@moonwall/util";

import { ApiPromise } from "@polkadot/api";

describeSuite({
  id: "R03",
  title: "Sample suite that only runs on Dancebox chains",
  foundationMethods: "read_only",
  testCases: ({ it, context, log }) => {
    let api: ApiPromise;

    beforeAll(() => {
      api = context.polkadotJs();
    });

    it({
      id: "C01",
      title: "Collator assignation and authority assignation should match with observed mapping in orchestrator",
      test: async function () {
        const assignmentCollatorAccount = (await api.query.collatorAssignment.collatorContainerChain()).toJSON();
        const sessionIndex = (await api.query.session.currentIndex()).toNumber();

        const assignmentCollatorKey = (await api.query.authorityAssignment.collatorContainerChain(sessionIndex)).toJSON();
        const authorityKeyMapping = (await api.query.authorityMapping.authorityIdMapping(sessionIndex)).toJSON();
        for (let key of  assignmentCollatorKey["orchestratorChain"]) {
            const assignedAccount = authorityKeyMapping[key.toString()];
            expect(assignmentCollatorAccount["orchestratorChain"].includes(assignedAccount.toString())).to.be.true;
        }
      } 
    });

    it({
        id: "C02",
        title: "Collator assignation and authority assignation should match with observed mapping in containers",
        test: async function () {
          const assignmentCollatorAccount = (await api.query.collatorAssignment.collatorContainerChain()).toJSON();
          const sessionIndex = (await api.query.session.currentIndex()).toNumber();
          const assignmentCollatorKey = (await api.query.authorityAssignment.collatorContainerChain(sessionIndex)).toJSON();
          const authorityKeyMapping = (await api.query.authorityMapping.authorityIdMapping(sessionIndex)).toJSON();
          for (let container of Object.keys(assignmentCollatorKey["containerChains"])) {
            for (let key of assignmentCollatorKey["containerChains"][container]) {
              const assignedAccount = authorityKeyMapping[key.toString()];
              expect(assignmentCollatorAccount["containerChains"][container].includes(assignedAccount.toString())).to.be.true;
            }
          }
        } 
      });
  },
});