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
let rewardAddress: string;
describe("Reward Distribution tests", function () {
  this.timeout(300 * BLOCK_TIME);

  before(async () => {
    await spawnNodes();
    const { alice, aliceStash } = await init();
    const rewardAccount = await getRewardAccount();
    rewardAddress = rewardAccount.toString();
    await transfer(alice, rewardAddress )

  });

  it("Should test Reward Distribution with Reliability score ", async () => {
    const { alice, aliceStash } = await init();
    const reliabilityZero = await getReliabilityScore(aliceStash);
    console.log("Reliability Score in Era 0 without Esg Score:", reliabilityZero);
    await waitNfinalizedBlocks(polkadotApi, 45, 1000);
    const eraZeroValidatorsReward = await getErasValidatorReward(aliceStash.address);
    console.log("Validator Reward in Era 0 without Esg Score:", eraZeroValidatorsReward.toHuman());

    const reliabilityOne = await getReliabilityScore(aliceStash);
    console.log("Reliability Score in Era 1 without Esg Score:", reliabilityOne);

    await waitNfinalizedBlocks(polkadotApi, 45, 1000);
    const eraOneValidatorsReward = await getErasValidatorReward(aliceStash.address);

    console.log("Validator Reward in Era 1 without Esg Score:", eraOneValidatorsReward.toHuman());

    expect(
      BigInt(eraOneValidatorsReward?.toString()) >
        BigInt(eraZeroValidatorsReward?.toString())
    ).true;

    const reliabilityTwo = await getReliabilityScore(aliceStash);
    console.log("Reliability Score in Era 2 without Esg Score:", reliabilityTwo);
    await waitNfinalizedBlocks(polkadotApi, 45, 1000);
    const eraTwoValidatorsReward = await getErasValidatorReward(aliceStash.address);
    console.log("Validator Reward in Era 2 without Esg Score:{}", eraTwoValidatorsReward.toHuman());

    expect(
      BigInt(eraTwoValidatorsReward?.toString()) >
        BigInt(eraOneValidatorsReward?.toString())
    ).true;

    await waitNfinalizedBlocks(polkadotApi, 2, 1000);
    // @ts-ignore
    const {data: rewardBalanceBeforeClaimByValidator} =  await api.query.system.account(rewardAddress);
    expect(rewardBalanceBeforeClaimByValidator.free.toBigInt()).to.equal(BigInt("1000000000000000000000"));

    await getReward(alice, aliceStash.address);
    await waitNfinalizedBlocks(polkadotApi, 45, 1000);
    const eraThreeValidatorsReward = await getErasValidatorReward(aliceStash.address);
    console.log("Validator Reward in Era 3 without Esg Score:{}", eraThreeValidatorsReward.toHuman());
    // @ts-ignore
    const {data: rewardBalanceAfterClaimByValidator} =  await api.query.system.account(rewardAddress);
    expect(rewardBalanceAfterClaimByValidator.free.toBigInt()).to.equal(BigInt("688000000000000000000"));


  });

  after(async () => {
    await killNodes();
  });
});

describe("Reward Distribution tests with Reliability score and sustainability score", function () {
  this.timeout(300 * BLOCK_TIME);

  before(async () => {
    await spawnNodes();
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

    const reliabilityZero = await getReliabilityScore(aliceStash);
    console.log("Reliability Score in Era 0 within Esg Score:", reliabilityZero);
    await waitNfinalizedBlocks(polkadotApi, 45, 1000);
    const eraZeroValidatorsReward = await getErasValidatorReward(aliceStash.address);
    console.log("Validator Reward in Era 0 within Esg Score:", eraZeroValidatorsReward.toHuman());

    const reliabilityOne = await getReliabilityScore(aliceStash);
    console.log("Reliability Score in Era 1 within Esg Score:", reliabilityOne);

    await waitNfinalizedBlocks(polkadotApi, 45, 1000);
    const eraOneValidatorsReward = await getErasValidatorReward(aliceStash.address);

    console.log("Validator Reward in Era 1 within Esg Score:", eraOneValidatorsReward.toHuman());

    expect(
      BigInt(eraOneValidatorsReward?.toString()) >
        BigInt(eraZeroValidatorsReward?.toString())
    ).true;

    const reliabilityTwo = await getReliabilityScore(aliceStash);
    console.log("Reliability Score in Era 2 within Esg Score:", reliabilityTwo);
    await waitNfinalizedBlocks(polkadotApi, 45, 1000);
    const eraTwoValidatorsReward = await getErasValidatorReward(aliceStash.address);
    console.log("Validator Reward in Era 2 within Esg Score:", eraTwoValidatorsReward.toHuman());

    expect(
      BigInt(eraTwoValidatorsReward?.toString()) >
        BigInt(eraOneValidatorsReward?.toString())
    ).true;

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

async function getReliabilityScore(aliceStash: KeyringPair) {
  const reliabilityScores = await api.query.imOnline.reliabilityScoresMap(
    aliceStash.address
  );
  return reliabilityScores;
}

async function getErasValidatorReward(validator: string) {
  const reward = await api.query.reward.validatorRewardAccounts(validator);
  return reward;
}

async function getRewardAccount() {
  const rewardAccount = await api.query.reward.rewardAccount();
  return rewardAccount;
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


export async function transfer(
  alice: KeyringPair,
  rewardAccount: string,
) {
  console.log(`\n Transfering coin to reward account.`);
  // Transfer 100 5ire to reward Account
  const transaction = polkadotApi.tx.balances.transfer(rewardAccount, "1000000000000000000000");

  const unsub = await transaction.signAndSend(
    alice,
    { tip: 200, nonce: -1 },
    (result) => {
      if (result.status.isInBlock) {
      } else if (result.status.isFinalized) {
        const data = JSON.stringify(result.events);
      }
    }
  );

  await waitForEvent(api, "balances", "Transfer");

}

export async function getReward(
  alice: KeyringPair,
  validator: string,
) {
  console.log(`\n Transfering coin to reward account.`);
  // Transfer 100 5ire to reward Account
  const transaction = polkadotApi.tx.reward.getRewards(validator);

  const unsub = await transaction.signAndSend(
    alice,
    { tip: 200, nonce: -1 },
    (result) => {
      if (result.status.isInBlock) {
      } else if (result.status.isFinalized) {
        const data = JSON.stringify(result.events);
      }
    }
  );

  await waitForEvent(api, "reward", "Rewarded");

}

