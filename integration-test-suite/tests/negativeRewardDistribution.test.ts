import { expect } from "chai";
import { BLOCK_TIME, alith, baltathar } from "../utils/constants";
import {
  killNodes,
  polkadotApi as api,
  spawnNodes,
  polkadotApi,
} from "../utils/util";
import { KeyringPair } from "@polkadot/keyring/types";
import { waitForEvent, waitNfinalizedBlocks } from "../utils/setup";

// We should test within 5 eras  ( 200 blocks)

describe("Negative Reward Distribution tests", function () {
  this.timeout(300 * BLOCK_TIME);

  before(async () => {
    await spawnNodes();
  });

  it("Negative test Reward Distribution with NoSuchValidator  ", async () => {
    // wait to new era
    await waitNfinalizedBlocks(polkadotApi, 45, 1000);

    // Payout fail with invalid validator
    await payoutInValidValidator(baltathar);

    await waitNfinalizedBlocks(polkadotApi, 2, 1000);
  });

  it("Negative test Reward Distribution with Already Claimed ", async () => {
    // wait to new era
    await waitNfinalizedBlocks(polkadotApi, 45, 1000);

    // // Valid payout transaction
    await payoutSuccess(alith);

    await waitForEvent(api, "reward", "Rewarded");
    // // Invalid payout transaction Already Claimed
    await payoutAlreadyClaimed(alith);

    // await waitNfinalizedBlocks(polkadotApi, 2, 1000);
  });

  after(async () => {
    await killNodes();
  });
});

// Payout Transaction
// alice : stash account
// alice_stash : controller account

async function payoutSuccess(alith: KeyringPair) {
  console.log(`\n Payout Success`);
  const payout = await api.tx.reward.getRewards(alith.address);

  const transaction = new Promise<{}>(async (resolve, reject) => {
    const unsub = await payout.signAndSend(
      alith,
      { tip: 2000, nonce: -1 },
      (result) => {
        console.log(`Payout transaction is ${result.status}`);
        if (result.status.isInBlock) {
          console.log(
            `Payout Transaction included at blockHash ${result.status.asInBlock}`,
          );
          console.log(`Waiting for finalization... (can take a minute)`);
        } else if (result.status.isFinalized) {
          unsub();
          resolve({});
        }
      },
    );
  });
}

// Alice is is not validator
async function payoutInValidValidator(baltathar: KeyringPair) {
  console.log(`\n Payout InValid Validator`);
  const payout = await api.tx.reward.getRewards(baltathar.address);

  const transaction = new Promise<{}>(async (resolve, reject) => {
    const unsub = await payout.signAndSend(
      baltathar,
      { tip: 2000, nonce: -1 },
      (result) => {
        console.log(`Payout transaction is ${result.status}`);
        if (result.status.isInBlock) {
          console.log(
            `Payout Transaction included at blockHash ${result.status.asInBlock}`,
          );
          console.log(`Waiting for finalization... (can take a minute)`);
        } else if (result.status.isFinalized) {
          const data = JSON.stringify(result.events);
          const dataStr = JSON.parse(data);
          if (result.dispatchError) {
            const filteredData = dataStr.filter(
              (item: any) => item.event.index === "0x0001",
            );
            expect(filteredData[0].event.data[0].module.error).to.equal(
              "0x00000000",
            ); // NoSuchValidator
          }

          unsub();
          resolve({});
        }
      },
    );
  });
}

async function payoutAlreadyClaimed(alith: KeyringPair) {
  console.log(`\n Payout fail due to AlreadyClaimed`);
  const payout = await api.tx.reward.getRewards(alith.address);

  const transaction = new Promise<{}>(async (resolve, reject) => {
    const unsub = await payout.signAndSend(
      alith,
      { tip: 2000, nonce: -1 },
      (result) => {
        console.log(`Payout transaction is ${result.status}`);
        if (result.status.isInBlock) {
          console.log(
            `Payout Transaction included at blockHash ${result.status.asInBlock}`,
          );
          console.log(`Waiting for finalization... (can take a minute)`);
        } else if (result.status.isFinalized) {
          const data = JSON.stringify(result.events);
          const dataStr = JSON.parse(data);
          if (result.dispatchError) {
            const filteredData = dataStr.filter(
              (item: any) => item.event.index === "0x0001",
            );

            expect(filteredData[0].event.data[0].module.error).to.equal(
              "0x00000000",
            ); // AlreadyClaimed
          }

          unsub();
          resolve({});
        }
      },
    );
  });
}
