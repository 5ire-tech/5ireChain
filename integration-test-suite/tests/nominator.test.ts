import { expect } from "chai";
import { BLOCK_TIME, alith, charleth } from "../utils/constants";
import { killNodes, polkadotApi, spawnNodes } from "../utils/util";
import { waitForEvent, waitNfinalizedBlocks } from "../utils/setup";

describe("Nominator tests", function () {
  this.timeout(300 * BLOCK_TIME);
  // 4 session.
  this.slow(40 * BLOCK_TIME);

  before(async () => {
    await spawnNodes();
  });

  // Should stake to become a nominator
  it("Should stake to become a nominator", async () => {
    const initialValidatorsNominated =
      await polkadotApi.query.staking.nominators(charleth.address);
    console.log(JSON.stringify(initialValidatorsNominated.toHuman()));
    // @ts-ignore
    expect(initialValidatorsNominated.toHuman() == null).true;

    const controller = polkadotApi.registry.createType(
      "PalletStakingRewardDestination",
      "Staked",
    );

    const amount = polkadotApi.createType("Balance", "900000000000000000000");
    let bondValidator = polkadotApi.tx.staking.bond(amount, controller);
    const bondValidatorTransaction = new Promise<{
      block: string;
      address: string;
    }>(async (resolve, reject) => {
      const unsub = await bondValidator.signAndSend(
        charleth,
        { tip: 200, nonce: -1 },
        (result) => {
          console.log(`bond nominator transaction is ${result.status}`);
          if (result.status.isInBlock) {
            console.log(
              `bond nominator transaction included at blockHash ${result.status.asInBlock}`,
            );
            console.log(
              `bond nominator transaction waiting for finalization... (can take a minute)`,
            );
          } else if (result.status.isFinalized) {
            console.log(
              `bond nominator transaction events are ${result.events}`,
            );
            console.log(
              `bond nominator transaction finalized at blockHash ${result.status.asFinalized}`,
            );
            unsub();
          }
        },
      );
    });
    await waitForEvent(polkadotApi, "staking", "Bonded");

    //const nominatedAddress = polkadotApi.registry.createType("MultiAddress", alith.address);
    const prefs = polkadotApi.registry.createType("Vec<MultiAddress>", [
      alith.address,
    ]);

    let nominateValidator = polkadotApi.tx.staking.nominate(prefs);
    const nominateValidatorTransaction = new Promise<{
      block: string;
      address: string;
    }>(async (resolve, reject) => {
      const unsub = await nominateValidator.signAndSend(
        charleth,
        { tip: 200, nonce: -1 },
        (result) => {
          console.log(`nominate validator transaction is ${result.status}`);
          if (result.status.isInBlock) {
            console.log(
              `nominate validator transaction included at blockHash ${result.status.asInBlock}`,
            );
            console.log(
              `nominate validator transaction waiting for finalization... (can take a minute)`,
            );
          } else if (result.status.isFinalized) {
            console.log(
              `nominate validator transaction events are ${result.events}`,
            );
            console.log(
              `nominate validator transaction finalized at blockHash ${result.status.asFinalized}`,
            );
            unsub();
          }
        },
      );
    });
    // wait 2 eras to become active nominator
    await waitNfinalizedBlocks(polkadotApi, 100, 1000);
    const validatorsNominated = await polkadotApi.query.staking.nominators(
      charleth.address,
    );
    console.log(JSON.stringify(validatorsNominated.toHuman()));

    // @ts-ignore
    expect(validatorsNominated.toHuman().targets[0] == alith.address).true;
  });

  it("Should unbond funds from a nominator", async () => {
    const amount = polkadotApi.createType("Balance", "200000000000000000000");
    let unbondValidator = polkadotApi.tx.staking.unbond(amount);
    const unbondValidatorTransaction = new Promise<{
      block: string;
      address: string;
    }>(async (resolve, reject) => {
      const unsub = await unbondValidator.signAndSend(
        charleth,
        { tip: 200, nonce: -1 },
        (result) => {
          console.log(`unbond nominator transaction is ${result.status}`);
          if (result.status.isInBlock) {
            console.log(
              `unbond nominator transaction included at blockHash ${result.status.asInBlock}`,
            );
            console.log(
              `unbond nominator transaction waiting for finalization... (can take a minute)`,
            );
          } else if (result.status.isFinalized) {
            console.log(
              `unbond nominator transaction events are ${result.events}`,
            );
            console.log(
              `unbond nominator transaction finalized at blockHash ${result.status.asFinalized}`,
            );
            unsub();
          }
        },
      );
    });
    await waitForEvent(polkadotApi, "staking", "Unbonded");
  });

  it("Should chill a nominator", async () => {
    let call = polkadotApi.tx.staking.chill();
    const callTransaction = new Promise<{ block: string; address: string }>(
      async (resolve, reject) => {
        const unsub = await call.signAndSend(
          charleth,
          { tip: 200, nonce: -1 },
          (result) => {
            console.log(`chill transaction is ${result.status}`);
            if (result.status.isInBlock) {
              console.log(
                `chill transaction included at blockHash ${result.status.asInBlock}`,
              );
              console.log(
                `chill transaction waiting for finalization... (can take a minute)`,
              );
            } else if (result.status.isFinalized) {
              console.log(`chill transaction events are ${result.events}`);
              console.log(
                `chill transaction finalized at blockHash ${result.status.asFinalized}`,
              );
              unsub();
            }
          },
        );
      },
    );
    await waitForEvent(polkadotApi, "staking", "Chilled");

    const validatorsNominated = await polkadotApi.query.staking.nominators(
      charleth.address,
    );
    console.log(JSON.stringify(validatorsNominated.toHuman()));
    // @ts-ignore
    expect(validatorsNominated.toHuman() == null).true;
  });

  after(async () => {
    await killNodes();
  });
});
