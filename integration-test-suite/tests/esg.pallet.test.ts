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

describe.only("ESG Pallet Integration tests", function () {
  this.timeout(300 * BLOCK_TIME);

  before(async () => {
    await spawnNodes();
  });

  // Should init
  it("Should test ESG Pallet", async () => {
    const { alice, bob, charlie, dave } = await init();

    const esgData = [
      {
        account: "5FLSigC9HGRKVhB9FiEo4Y3koPsNmBmLJbpXg2mp1hXcS59Y",
        score: "43",
      },
    ];

    const jsonData = JSON.stringify(esgData);

    await registerOracle(alice, bob);

    await registerOracleBySudoOracle(bob, dave);

    await registerOracleFailedOracleRegisteredAlready(alice, bob);

    await insertEsgScores(bob, charlie, jsonData);

    await insertEsgScoresCallerNotAnOracle(charlie, jsonData);

    await deRegisterOracle(alice, bob);

    await registerOracleFailedCallerNotRootOrSudoOracle( bob, charlie );

    await deRegisterOracleForOracleNotExist(alice, charlie);

    await deRegisterOracleFromBadOrigin(bob, charlie);
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
  const dave = keyring.addFromUri("//Dave", { name: "Dave default" });

  return { alice, bob, charlie, dave };
}

function delay(ms: number) {
    return new Promise( resolve => setTimeout(resolve, ms) );
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

// Register the Dave account as oracle in ESG pallet from BOB(sudo oracle account).
async function registerOracleBySudoOracle(bob: KeyringPair, dave: KeyringPair) {
  console.log(`\n Registering Oracle by the sudo oracle `);

  const transaction = await api.tx.esgScore.registerAnOracle(dave.address, true);

  const unsub = await transaction
    .signAndSend(bob, { tip: 200, nonce: -1 }, (result) => {
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

// Register the Bob account as oracle in ESG pallet failed for non-sudo account.
async function registerOracleFailedCallerNotRootOrSudoOracle( bob: KeyringPair, charlie: KeyringPair) {
  console.log(`\n Registering Oracle failed due to CallerNotRootOrSudoOracle`);

  const transaction = await api.tx.esgScore.registerAnOracle(charlie.address, true);

  const unsub = await transaction
  .signAndSend(bob, { tip: 200, nonce: -1 }, (result) => {
      console.log(`Oracle Registration is ${result.status}`);
      if (result.status.isInBlock) {
        console.log(
          `Oracle Registration at blockHash ${result.status.asInBlock}`
        );
        console.log(`Waiting for finalization... (can take a minute)`);
      } else if (result.status.isFinalized) {
        const data = JSON.stringify(result.events);
        const dataStr = JSON.parse(data);
        const filteredErrorData = dataStr.filter((item: any) => item.event.index === "0x0001");
        expect(filteredErrorData[0].event.data[0].module.error).to.equal("0x06000000");
        console.log(`Error found: ${filteredErrorData[0].event.data[0].module.error}`);

      }
    });

  await delay(12000);
  return true;
}

// Register the Bob account as oracle in ESG pallet failed for already registered account.
async function registerOracleFailedOracleRegisteredAlready( alice: KeyringPair, bob: KeyringPair) {
  console.log(`\n Registering Oracle failed due to OracleRegisteredAlready`);

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
        const dataStr = JSON.parse(data)
        const filteredErrorData = dataStr.filter((item: any) => item.event.index === "0x1400");
        expect(filteredErrorData[0].event.data[0].err.module.error).to.equal("0x05000000");
        console.log(`Error found: ${filteredErrorData[0].event.data[0].err.module.error}`);

      }
    });
    await delay(12000);

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

// Insert the ESG scores of the User by non oracle account(Charlie) we added above.
async function insertEsgScoresCallerNotAnOracle(
  charlie: KeyringPair,
  jsonData: string
) {
  console.log(`\n Inserting ESG Score of the user from CallerNotAnOracle.`);

  const transaction = await api.tx.esgScore.upsertEsgScores(jsonData);

  const unsub = await transaction.signAndSend(
    charlie,
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
        const dataStr = JSON.parse(data);
        const filteredErrorData = dataStr.filter((item: any) => item.event.index === "0x0001");
        expect(filteredErrorData[0].event.data[0].module.error).to.equal("0x04000000");
        console.log(`Error found: ${filteredErrorData[0].event.data[0].module.error}`);

      }
    }
  );
  await delay(12000);
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

  const transaction = api.tx.esgScore.deregisterAnOracle(
    bob.address,
    true
  );

  await sudoTx(api, transaction);

  await waitForEvent(api, "esgScore", "OracleDeRegistered");
  const oracleAccounts = await api.query.esgScore.sudoOraclesStore();
  expect(!oracleAccounts.toString().includes(bob.address.toString()));
  console.log(`Account verified in the oracle storage: ${oracleAccounts}`);

  return true;
}

// De-Register the Bob account as oracle in ESG pallet from ALICE(sudo account).
async function deRegisterOracleForOracleNotExist(alice: KeyringPair, charlie: KeyringPair) {
  console.log(`\nDe-Registering Oracle for OracleNotExist`);

  const transaction = api.tx.esgScore.deregisterAnOracle(
    charlie.address,
    true
  );

 const unsub = await api.tx.sudo
   .sudo(transaction.method.toHex())
   .signAndSend(alice, { tip: 200, nonce: -1 }, (result) => {
       console.log(`Oracle De-Registration for OracleNotExist is ${result.status}`);
       if (result.status.isInBlock) {
         console.log(
           `Oracle Registration at blockHash ${result.status.asInBlock}`
         );
         console.log(`Waiting for finalization... (can take a minute)`);
       } else if (result.status.isFinalized) {
         const data = JSON.stringify(result.events);
         console.log(data);
         const dataStr = JSON.parse(data)
         const filteredErrorData = dataStr.filter((item: any) => item.event.index === "0x1400");
         expect(filteredErrorData[0].event.data[0].err.module.error).to.equal("0x03000000");
         console.log(`Error found: ${filteredErrorData[0].event.data[0].err.module.error}`);
       }
     });

     await delay(12000);

  return true;
}

// De-Register the charlie account as oracle in ESG pallet from charlie(non sudo account).
async function deRegisterOracleFromBadOrigin(bob: KeyringPair, charlie: KeyringPair) {
  console.log(`\nDe-Registering Oracle for FromBadOrigin`);


  const transaction = api.tx.esgScore.deregisterAnOracle(
    charlie.address,
    true
  );

 const unsub = await transaction
   .signAndSend(bob, { tip: 200, nonce: -1 }, (result) => {
       console.log(`Oracle De-Registration for OracleNotExist is ${result.status}`);
       if (result.status.isInBlock) {
         console.log(
           `Oracle Registration at blockHash ${result.status.asInBlock}`
         );
         console.log(`Waiting for finalization... (can take a minute)`);
       } else if (result.status.isFinalized) {
         const data = JSON.stringify(result.events);
         console.log(data);
         const dataStr = JSON.parse(data)
         const filteredErrorData = dataStr.filter((item: any) => item.event.index === "0x0001");
         expect(filteredErrorData[0].event.data[0].badOrigin === null);
         console.log(`Error found: ${filteredErrorData[0].event.data[0].badOrigin}`);
       }
     });

     await delay(12000);

  return true;
}
