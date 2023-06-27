import { expect } from "chai";
import { BLOCK_TIME } from "../utils/constants";
import { killNodes, polkadotApi as api, spawnNodes } from "../utils/util";
import { Keyring, WsProvider } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import {sudoTx, waitForEvent} from "../utils/setup";

// Keyring needed to sign using Alice account
const keyring = new Keyring({ type: "sr25519" });

// This script contains the integration test for the ESG pallet.
// ESG pallet is the pallet in 5ire-chain which is responsible to add the esg score and related transactions.

describe.only("Reliability score Integration tests", function () {
  this.timeout(300 * BLOCK_TIME);

  before(async () => {
    await spawnNodes();
  });

  // Should init
  it("Should test Reliability Score", async () => {
    const { alice } = await init();

    await reliabilityScore(alice);

  });

  after(async () => {
    await killNodes();
  });
});

// Setup the API and Accounts
async function init() {
  const alice = keyring.addFromUri("//Alice", { name: "Alice default" });

  return { alice };
}


// Insert the ESG scores of the User by oracle account(bob) we added above.
async function reliabilityScore(alice: KeyringPair,) {
  console.log(`\n Checking Reliability score of the validator.`);

  const score = await api.query.imOnline.reliabilityScoresMap(alice);
  console.log(`Reliability score of the node: ${score}`);
  expect(score.toString() == "0");


  return true;
}
