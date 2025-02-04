import { describeSuite, expect } from "@moonwall/cli";
import { RELAY_V3_SOURCE_LOCATION } from "helpers/assets";

describeSuite({
    id: "CPX0206",
    title: "XCM - LocationToAccountApi",
    foundationMethods: "dev",
    testCases: ({ context, it }) => {
        it({
            id: "T01",
            title: "Should succeed calling convertLocation",
            test: async () => {
                const chain = context.polkadotJs().consts.system.version.specName.toString();

                const convertLocation = await context
                    .polkadotJs()
                    .call.locationToAccountApi.convertLocation(RELAY_V3_SOURCE_LOCATION);

                expect(convertLocation.isOk).to.be.true;

                if (chain === "frontier-template")
                    expect(convertLocation.asOk.toHuman()).to.eq("0x506172656E740000000000000000000000000000");
                else expect(convertLocation.asOk.toHuman()).to.eq("5Dt6dpkWPwLaH4BBCKJwjiWrFVAGyYk3tLUabvyn4v7KtESG");
            },
        });
    },
});
