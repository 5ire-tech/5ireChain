import { expect } from "chai";
import { BLOCK_TIME } from "../utils/constants";
import { killNodes, polkadotApi as api, spawnNodes } from "../utils/util";
import { Keyring, WsProvider } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { waitForEvent } from "../utils/setup";

// Keyring needed to sign using Alice account
const keyring = new Keyring({ type: "sr25519" });

// This script contains the integration test for the ESG pallet.
// ESG pallet is the pallet in 5ire-chain which is responsible to add the esg score and related transactions.

describe("ESG Pallet Integration tests", function () {
  this.timeout(300 * BLOCK_TIME);

  before(async () => {
    await spawnNodes();
  });

  // Should init
  it("Should test ESG Pallet", async () => {
    const { alice, bob, charlie } = await init();

    const esgData = [
      {
        account: "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y",
        score: "43",
      },
    ];

    const jsonData = JSON.stringify(esgData);

    await registerOracle(alice, bob);

    await insertEsgScores(bob, charlie, jsonData);

    await deRegisterOracle(alice, bob);
  });

  after(async () => {
    await killNodes();
  });
});

// Setup the API and Accounts
async function init() {
  const alice = keyring.addFromUri("//Alice", { name: "Alice default" });
  const bob = keyring.addFromUri("//Bob", { name: "Bob default" });
  const charlie = keyring.addFromUri("//Charlie", { name: "Charlie default" });

  return { alice, bob, charlie };
}

// Register the Bob account as oracle in ESG pallet from ALICE(sudo account).
async function registerOracle(alice: KeyringPair, bob: KeyringPair) {
  console.log(`\n: Registering Oracle`);

  const transaction = await api.tx.esgScore.registerAnOracle(bob.address, true);

  const unsub = await api.tx.sudo
    .sudo(transaction.method.toHex())
    .signAndSend(alice, { tip: 200, nonce: -1 }, (result) => {
      console.log(`Oracle Registration is ${result.status}`);
      if (result.status.isInBlock) {
        console.log(
          `Oracle Registration at blockHash ${result.status.asInBlock}`
        );
        console.log(`Waiting for finalization... (can take a minute)`);
      } else if (result.status.isFinalized) {
        const data = JSON.stringify(result.events);
        console.log(data);
      }
    });

  await waitForEvent(api, "esgScore", "NewOracleRegistered");
  const oracleAccounts = await api.query.esgScore.sudoOraclesStore();
  expect(oracleAccounts.toString().includes(bob.address.toString()));
  console.log(`Account verified in the oracle storage: ${oracleAccounts}`);

  return true;
}

// Insert the ESG scores of the User by oracle account(bob) we added above.
async function insertEsgScores(
  bob: KeyringPair,
  user: KeyringPair,
  jsonData: string
) {
  console.log(`\n Inserting ESG Score of the user.`);

  const transaction = await api.tx.esgScore.upsertEsgScores(jsonData);

  const unsub = await transaction.signAndSend(
    bob,
    { tip: 200, nonce: -1 },
    (result) => {
      console.log(`Insertion of ESG score is ${result.status}`);
      if (result.status.isInBlock) {
        console.log(
          `Insertion of ESG score at blockHash ${result.status.asInBlock}`
        );
        console.log(`Waiting for finalization... (can take a minute)`);
      } else if (result.status.isFinalized) {
        const data = JSON.stringify(result.events);
        console.log(data);
      }
    }
  );

  await waitForEvent(api, "esgScore", "ESGStored");
  const score = await api.query.esgScore.esgScoresMap(user.address);
  expect(score.toString() == "43");
  console.log(`ESG Score verified in storage: ${score}`);

  return true;
}

// De-Register the Bob account as oracle in ESG pallet from ALICE(sudo account).
async function deRegisterOracle(alice: KeyringPair, bob: KeyringPair) {
  console.log(`\n: De-Registering Oracle`);

  const existingOracleAccounts = await api.query.esgScore.sudoOraclesStore();
  expect(existingOracleAccounts.toString().includes(bob.address.toString()));
  console.log(
    `Account verified in the oracle storage: ${existingOracleAccounts}`
  );

  const transaction = await api.tx.esgScore.deregisterAnOracle(
    bob.address,
    true
  );

  const unsub = await api.tx.sudo
    .sudo(transaction.method.toHex())
    .signAndSend(alice, { tip: 200, nonce: -1 }, (result) => {
      console.log(`Oracle De-Registration is ${result.status}`);
      if (result.status.isInBlock) {
        console.log(
          `Oracle De-Registration at blockHash ${result.status.asInBlock}`
        );
        console.log(`Waiting for finalization... (can take a minute)`);
      } else if (result.status.isFinalized) {
        const data = JSON.stringify(result.events);
        console.log(data);
      }
    });

  await waitForEvent(api, "esgScore", "OracleDeRegistered");
  const oracleAccounts = await api.query.esgScore.sudoOraclesStore();
  expect(!oracleAccounts.toString().includes(bob.address.toString()));
  console.log(`Account verified in the oracle storage: ${oracleAccounts}`);

  return true;
}
