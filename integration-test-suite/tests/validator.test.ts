import { expect } from 'chai';
import { BLOCK_TIME } from '../utils/constants';
import {killNodes, polkadotApi as api, polkadotApi, spawnNodes} from "../utils/util";
import {Keyring} from "@polkadot/api";
import {waitForEvent, waitForTheNextSession} from "../utils/setup";
import {BN} from "@polkadot/util";
import {WeightV2} from "@polkadot/types/interfaces";


// Keyring needed to sign using Alice account
const keyring = new Keyring({ type: 'sr25519' });

describe.only('Validator tests', function () {
  this.timeout(300 * BLOCK_TIME);
  // 4 session.
  this.slow(40 * BLOCK_TIME);

  before(async () => {
    await spawnNodes();
  });

  // Should stake to become a validator
  it('Should stake to become a validator', async () => {
    const alice = keyring.addFromUri('//Alice');
    const bob = keyring.addFromUri('//Bob');

    const charlie = keyring.addFromUri('//Charlie');
    const aliceStash = keyring.addFromUri('//Alice//stash');

    let forceNewEra = polkadotApi.tx.staking.forceNewEraAlways();
    const forceNewEraAlwaysCall = polkadotApi.tx.sudo.sudo({
      callIndex: forceNewEra.callIndex,
      args: forceNewEra.args,
    });
    await forceNewEraAlwaysCall.signAndSend(alice, {tip: 200, nonce: -1});

    let increaseValidatorCount = polkadotApi.tx.staking.increaseValidatorCount(2);
    const increaseValidatorCountCall = polkadotApi.tx.sudo.sudo({
      callIndex: increaseValidatorCount.callIndex,
      args: increaseValidatorCount.args,
    });
    await increaseValidatorCountCall.signAndSend(alice, {tip: 200, nonce: -1});

    let currentKey = await  polkadotApi.rpc.author.rotateKeys();
    console.log(`current key for validator is ${currentKey}`);

    const controller = polkadotApi.registry.createType("PalletStakingRewardDestination", "Staked");

    const amount = polkadotApi.createType('Balance', '900000000000000000000');
    let bondValidator = polkadotApi.tx.staking.bond(bob.address, amount, controller);
    const bondValidatorTransaction = new Promise<{ block: string, address: string }>(async (resolve, reject) => {
      const unsub = await bondValidator.signAndSend(bob, {tip: 200, nonce: -1}, (result) => {
        console.log(`bond validator transaction is ${result.status}`);
        if (result.status.isInBlock) {
          console.log(`bond validator transaction included at blockHash ${result.status.asInBlock}`);
          console.log(`bond validator transaction waiting for finalization... (can take a minute)`);
        } else if (result.status.isFinalized) {
          console.log( `bond validator transaction events are ${result.events}`)
          console.log(`bond validator transaction finalized at blockHash ${result.status.asFinalized}`);
          unsub();
        }
      });
    });
    await waitForEvent(polkadotApi, 'staking', 'Bonded');


    const prefs = polkadotApi.registry.createType("PalletStakingValidatorPrefs", {
      commission: 100,
      blocked: false
    });

    let validateValidator = polkadotApi.tx.staking.validate(prefs);
    const validateValidatorTransaction = new Promise<{ block: string, address: string }>(async (resolve, reject) => {
      const unsub = await validateValidator.signAndSend(bob, {tip: 200, nonce: -1}, (result) => {
        console.log(`validate validator transaction is ${result.status}`);
        if (result.status.isInBlock) {
          console.log(`validate validator transaction included at blockHash ${result.status.asInBlock}`);
          console.log(`validate validator transaction waiting for finalization... (can take a minute)`);
        } else if (result.status.isFinalized) {
          console.log( `validate validator transaction events are ${result.events}`)
          console.log(`validate validator transaction finalized at blockHash ${result.status.asFinalized}`);
          unsub();
        }
      });
    });
    await waitForEvent(polkadotApi, 'staking', 'ValidatorPrefsSet');

    let setKeys = polkadotApi.tx.session.setKeys(currentKey, currentKey);
    const setKeysTransaction = new Promise<{ block: string, address: string }>(async (resolve, reject) => {
      const unsub = await setKeys.signAndSend(bob, {tip: 200, nonce: -1}, (result) => {
        console.log(`set keys transaction is ${result.status}`);
        if (result.status.isInBlock) {
          console.log(`set keys transaction included at blockHash ${result.status.asInBlock}`);
          console.log(`set keys transaction waiting for finalization... (can take a minute)`);
        } else if (result.status.isFinalized) {
          console.log( `set keys transaction events are ${result.events}`)
          console.log(`set keys transaction finalized at blockHash ${result.status.asFinalized}`);
          unsub();
        }
      });
    });
    await waitForEvent(polkadotApi, 'staking', 'ValidatorPrefsSet');

    // wait for the next 2 eras to confirm that Bob is a validator
    for (let i=0; i<24; i++) {
      await waitForTheNextSession(polkadotApi);
    }

  });

  it('Should chill a validator', async () => {
    const alice = keyring.addFromUri('//Alice');
    const bob = keyring.addFromUri('//Bob');

    const charlie = keyring.addFromUri('//Charlie');
    const aliceStash = keyring.addFromUri('//Alice//stash');

    let call = polkadotApi.tx.staking.chill();
    const callTransaction = new Promise<{ block: string, address: string }>(async (resolve, reject) => {
      const unsub = await call.signAndSend(bob, {tip: 200, nonce: -1}, (result) => {
        console.log(`chill transaction is ${result.status}`);
        if (result.status.isInBlock) {
          console.log(`chill transaction included at blockHash ${result.status.asInBlock}`);
          console.log(`chill transaction waiting for finalization... (can take a minute)`);
        } else if (result.status.isFinalized) {
          console.log( `chill transaction events are ${result.events}`)
          console.log(`chill transaction finalized at blockHash ${result.status.asFinalized}`);
          unsub();
        }
      });
    });
    await waitForEvent(polkadotApi, 'staking', 'Chilled');

    // wait for the next 2 eras to confirm that Bob is a validator
    for (let i=0; i<24; i++) {
      await waitForTheNextSession(polkadotApi);
    }

  });

  after(async () => {
    await killNodes();
  });
});
