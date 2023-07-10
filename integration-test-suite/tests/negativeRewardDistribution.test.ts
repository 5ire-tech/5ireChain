import { expect } from "chai";
import { BLOCK_TIME } from "../utils/constants";
import {
  killNodes,
  polkadotApi as api,
  spawnNodes,
  polkadotApi,
} from "../utils/util";
import { Keyring } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { waitForEvent, waitNfinalizedBlocks } from "../utils/setup";
// Keyring needed to sign using Alice account
const keyring = new Keyring({ type: "sr25519" });

// We should test within 5 eras  ( 200 blocks)

describe.only("Negative Reward Distribution tests", function () {
  this.timeout(300 * BLOCK_TIME);

  before(async () => {
    await spawnNodes();
  });

  it("Negative test Reward Distribution with Invalid Era  ", async () => {
    const { alice, aliceStash } = await init();
    let invalidEra = 10;

    await payoutInvalidEra(alice, aliceStash,invalidEra );

    await waitNfinalizedBlocks(polkadotApi, 2, 1000);
  });

  it("Negative test Reward Distribution with Already Claimed with certain era ", async () => {
    const { alice, aliceStash } = await init();
    let eraZero = await getCurrentEra();
    await waitNfinalizedBlocks(polkadotApi, 45, 1000);
    // Valid payout transaction
    await payoutSuccess(alice, aliceStash,eraZero );

    await waitForEvent(api, "staking", "Rewarded");
    // Invalid payout transaction Already Claimed
    await payoutAlreadyClaimed(alice, aliceStash,eraZero );

    await waitNfinalizedBlocks(polkadotApi, 2, 1000);
  });

  after(async () => {
    await killNodes();
  });
});

// Setup the API and Accounts
async function init() {
  const alice = keyring.addFromUri("//Alice", { name: "Alice default" });
  const aliceStash = keyring.addFromUri("//Alice//stash");
  const bobStash = keyring.addFromUri("//Bob//stash");
  return { alice, aliceStash, bobStash };
}

async function getCurrentEra() {
    const currentEra = await api.query.session.currentIndex();
    console.log("\n: Current Era:", currentEra);
    return currentEra;
  }

// Payout Transaction
// alice : stash account
// alice_stash : controller account

async function payoutSuccess(alice: KeyringPair, aliceStash: KeyringPair, era: any) {
    console.log(`\n Payout Success`); 
    const payout = await api.tx.staking.payoutStakers(aliceStash.address, era);
  
    const transaction = new Promise<{}>(async (resolve, reject) => {
      const unsub = await payout.signAndSend(
        alice,
        { tip: 2000, nonce: -1 },
        (result) => {
          console.log(`Payout transaction is ${result.status}`);
          if (result.status.isInBlock) {
            console.log(
              `Payout Transaction included at blockHash ${result.status.asInBlock}`
            );
            console.log(`Waiting for finalization... (can take a minute)`);
          } else if (result.status.isFinalized) {
            unsub();
            resolve({});
          }
        }
      );
    });
    
  }


async function payoutAlreadyClaimed(alice: KeyringPair, aliceStash: KeyringPair, era: any) {
    console.log(`\n Payout fail due to AlreadyClaimed`);  
    const payout = await api.tx.staking.payoutStakers(aliceStash.address, era);

  const transaction = new Promise<{}>(async (resolve, reject) => {
    const unsub = await payout.signAndSend(
      alice,
      { tip: 2000, nonce: -1 },
      (result) => {
        console.log(`Payout transaction is ${result.status}`);
        if (result.status.isInBlock) {
          console.log(
            `Payout Transaction included at blockHash ${result.status.asInBlock}`
          );
          console.log(`Waiting for finalization... (can take a minute)`);
        } else if (result.status.isFinalized) {
          const data = JSON.stringify(result.events);
          const dataStr = JSON.parse(data);
          if (result.dispatchError){
            const filteredData = dataStr.filter((item: any) => item.event.index === "0x0001");
  
            expect(filteredData[0].event.data[0].module.error).to.equal("0x0e000000");// AlreadyClaimed
          }

          unsub();
          resolve({});
        }
      }
    );
  });
  
}


async function payoutInvalidEra(alice: KeyringPair, aliceStash: KeyringPair, era: any) {
    console.log(`\n Payout fail due to InvalidEra`); 
    const payout = await api.tx.staking.payoutStakers(aliceStash.address, era);
  
    const transaction = new Promise<{}>(async (resolve, reject) => {
      const unsub = await payout.signAndSend(
        alice,
        { tip: 2000, nonce: -1 },
        (result) => {
          console.log(`Payout transaction is ${result.status}`);
          if (result.status.isInBlock) {
            console.log(
              `Payout Transaction included at blockHash ${result.status.asInBlock}`
            );
            console.log(`Waiting for finalization... (can take a minute)`);
          } else if (result.status.isFinalized) {
            const data = JSON.stringify(result.events);
            const dataStr = JSON.parse(data);
            if (result.dispatchError){
              const filteredData = dataStr.filter((item: any) => item.event.index === "0x0001");
    
              expect(filteredData[0].event.data[0].module.error).to.equal("0x0b000000");// InvalidEratoReward
            }
  
            unsub();
            resolve({});
          }
        }
      );
    });
    
  }



