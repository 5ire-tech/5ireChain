import { expect } from "chai";
import { BLOCK_TIME } from "../utils/constants";
import {
  killNodes,
  polkadotApi as api,
  spawnNodes,
  polkadotApi,
} from "../utils/util";
import { Keyring, WsProvider } from "@polkadot/api";
import { KeyringPair } from "@polkadot/keyring/types";
import { sleep } from "../utils/setup";
import { sudoTx, waitForEvent, waitNfinalizedBlocks } from "../utils/setup";
// Keyring needed to sign using Alice account
const keyring = new Keyring({ type: "sr25519" });

// We should test within 5 eras  ( 200 blocks)

describe.only("Reward Distribution tests", function () {
  this.timeout(300 * BLOCK_TIME);

  before(async () => {
    await spawnNodes();
  });

  it("Should test Reward Distribution with Reliability score ", async () => {
    const { alice, aliceStash } = await init();
    const eraZero = (await getCurrentEra()).toString();
    const reliabilityZero = await getReliabilityScore(aliceStash);
    console.log("Reliability Zero:", reliabilityZero);
    await waitNfinalizedBlocks(polkadotApi, 45, 1000);
    const eraZeroValidatorsReward = await getErasValidatorReward(eraZero);
    console.log("eraZeroValidatorsReward:", eraZeroValidatorsReward.toHuman());

   


    const eraOne = (await getCurrentEra()).toString();
    const reliabilityOne = await getReliabilityScore(aliceStash);
    console.log("Reliability One:", reliabilityOne);
    
    await waitNfinalizedBlocks(polkadotApi, 45, 1000);
    const eraOneValidatorsReward = await getErasValidatorReward(eraOne);

    console.log("eraOneValidatorsReward:", eraOneValidatorsReward.toHuman());

    expect(BigInt(eraOneValidatorsReward?.toString()) > BigInt(eraZeroValidatorsReward?.toString())).true;

    const eraTwo = (await getCurrentEra()).toString();
    const reliabilityTwo = await getReliabilityScore(aliceStash);
    console.log("Reliability Two:", reliabilityTwo);
    await waitNfinalizedBlocks(polkadotApi, 45, 1000);
    const eraTwoValidatorsReward = await getErasValidatorReward(eraTwo);
    console.log("eraTwoValidatorsReward:{}", eraTwoValidatorsReward.toHuman());

    expect(BigInt(eraTwoValidatorsReward?.toString()) > BigInt(eraOneValidatorsReward?.toString())).true;


    await waitNfinalizedBlocks(polkadotApi, 2, 1000);
  });


  it("Should test Reward Distribution with Reliability score and sustainability score ", async () => {
    const { alice, aliceStash } = await init();

    const esgData = [
      {
        account: aliceStash.address,
        score: "50",
      },
    ];

    const jsonData = JSON.stringify(esgData);
    await registerOracle(alice, aliceStash);
    await insertEsgScores(aliceStash, aliceStash, jsonData);

    const eraZero = (await getCurrentEra()).toString();
    const reliabilityZero = await getReliabilityScore(aliceStash);
    console.log("Reliability Zero:", reliabilityZero);
    await waitNfinalizedBlocks(polkadotApi, 45, 1000);
    const eraZeroValidatorsReward = await getErasValidatorReward(eraZero);
    console.log("eraZeroValidatorsReward:", eraZeroValidatorsReward.toHuman());

   


    const eraOne = (await getCurrentEra()).toString();
    const reliabilityOne = await getReliabilityScore(aliceStash);
    console.log("Reliability One:", reliabilityOne);
    
    await waitNfinalizedBlocks(polkadotApi, 45, 1000);
    const eraOneValidatorsReward = await getErasValidatorReward(eraOne);

    console.log("eraOneValidatorsReward:", eraOneValidatorsReward.toHuman());

    expect(BigInt(eraOneValidatorsReward?.toString()) > BigInt(eraZeroValidatorsReward?.toString())).true;

    const eraTwo = (await getCurrentEra()).toString();
    const reliabilityTwo = await getReliabilityScore(aliceStash);
    console.log("Reliability Two:", reliabilityTwo);
    await waitNfinalizedBlocks(polkadotApi, 45, 1000);
    const eraTwoValidatorsReward = await getErasValidatorReward(eraTwo);
    console.log("eraTwoValidatorsReward:{}", eraTwoValidatorsReward.toHuman());

    expect(BigInt(eraTwoValidatorsReward?.toString()) > BigInt(eraOneValidatorsReward?.toString())).true;


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

  return { alice, aliceStash };
}

async function getReliabilityScore(
  aliceStash: KeyringPair
) {

  const reliabilityScores = await api.query.imOnline.reliabilityScoresMap(
    aliceStash.address
  );
  return reliabilityScores;
}

async function getErasValidatorReward(era: string) {
  

  const reward = await api.query.staking.erasValidatorReward(era);
  return reward;
}

async function getCurrentEra() {
  const currentEra = await api.query.session.currentIndex();
  console.log("\n: Current Era:", currentEra);
  return currentEra;
}

export async function insertEsgScores(
  aliceStash: KeyringPair,
  user: KeyringPair,
  jsonData: string
) {
  console.log(`\n Inserting ESG Score of the user.`);

  const transaction = await api.tx.esgScore.upsertEsgScores(jsonData);

  const unsub = await transaction.signAndSend(
    aliceStash,
    { tip: 200, nonce: -1 },
    (result) => {
      if (result.status.isInBlock) {
      } else if (result.status.isFinalized) {
        const data = JSON.stringify(result.events);
      }
    }
  );

  await waitForEvent(api, "esgScore", "ESGStored");
  const score = await api.query.esgScore.esgScoresMap(user.address);
  console.log(`ESG Score verified in storage: ${score}`);
}

export async function registerOracle(
  alice: KeyringPair,
  aliceStash: KeyringPair
) {
  console.log(`\n: Registering Oracle`);

  const transaction = await api.tx.esgScore.registerAnOracle(
    aliceStash.address,
    true
  );

  const unsub = await api.tx.sudo
    .sudo(transaction.method.toHex())
    .signAndSend(alice, { tip: 200, nonce: -1 }, (result) => {
      if (result.status.isInBlock) {
      } else if (result.status.isFinalized) {
        const data = JSON.stringify(result.events);
      }
    });

  await waitForEvent(api, "esgScore", "NewOracleRegistered");
  const oracleAccounts = await api.query.esgScore.sudoOraclesStore();
  console.log(`Account verified in the oracle storage: ${oracleAccounts}`);
}
