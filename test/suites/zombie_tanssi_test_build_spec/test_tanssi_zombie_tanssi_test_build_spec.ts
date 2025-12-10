import { beforeAll, describeSuite } from "@moonwall/cli";

describeSuite({
    id: "ZOMBIETANSSIEXPORT",
    title: "Zombie Tanssi Relay Test",
    foundationMethods: "zombie",
    testCases: ({ it }) => {
        beforeAll(async () => {});

        it({
            id: "T01",
            title: "Mock test to test build spec",
            test: async () => {},
        });
    },
});
