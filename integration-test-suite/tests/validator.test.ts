import { expect } from 'chai';
import { BLOCK_TIME, alith, baltathar } from '../utils/constants';
import {killNodes, polkadotApi as api, polkadotApi, spawnNodes} from "../utils/util";
import {sleep, waitForEvent, waitForTheNextSession, waitNfinalizedBlocks} from "../utils/setup";
import {WeightV2} from "@polkadot/types/interfaces";


describe('Validator tests', function () {
  this.timeout(300 * BLOCK_TIME);
  // 4 session.
  this.slow(40 * BLOCK_TIME);

  before(async () => {
    await spawnNodes();
  });

  // Should stake to become a validator
  it('Should stake to become a validator', async () => {


    const initialValidators = await polkadotApi.query.session.validators();
    // @ts-ignore
    expect(initialValidators.length == 1).true;

    let forceNewEra = polkadotApi.tx.staking.forceNewEraAlways();
    const forceNewEraAlwaysCall = polkadotApi.tx.sudo.sudo({
      callIndex: forceNewEra.callIndex,
      args: forceNewEra.args,
    });
    await forceNewEraAlwaysCall.signAndSend(alith, {tip: 200, nonce: -1});

    let increaseValidatorCount = polkadotApi.tx.staking.increaseValidatorCount(2);
    const increaseValidatorCountCall = polkadotApi.tx.sudo.sudo({
      callIndex: increaseValidatorCount.callIndex,
      args: increaseValidatorCount.args,
    });
    await increaseValidatorCountCall.signAndSend(alith, {tip: 200, nonce: -1});

    let currentKey = await  polkadotApi.rpc.author.rotateKeys();
    console.log(`current key for validator is ${currentKey}`);

    const controller = polkadotApi.registry.createType("PalletStakingRewardDestination", "Staked");

    const amount = polkadotApi.createType('Balance', '900000000000000000000');
    let bondValidator = polkadotApi.tx.staking.bond(amount, controller);
    const bondValidatorTransaction = new Promise<{ block: string, address: string }>(async (resolve, reject) => {
      const unsub = await bondValidator.signAndSend(baltathar, {tip: 200, nonce: -1}, (result) => {
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
      commission: 100_000_000,
      blocked: false
    });

    let validateValidator = polkadotApi.tx.staking.validate(prefs);
    const validateValidatorTransaction = new Promise<{ block: string, address: string }>(async (resolve, reject) => {
      const unsub = await validateValidator.signAndSend(baltathar, {tip: 200, nonce: -1}, (result) => {
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
      const unsub = await setKeys.signAndSend(baltathar, {tip: 200, nonce: -1}, (result) => {
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

    // wait 2 eras 
    await waitNfinalizedBlocks(polkadotApi, 100, 1000);

    const validators = await polkadotApi.query.session.validators();
    // @ts-ignore
    console.log(`validators are ${validators.length}`);
    // @ts-ignore
    expect(validators.length > initialValidators.length).true;
    // @ts-ignore
    expect(validators.length == 2).true;
    // @ts-ignore
    expect(validators.toHuman()[0] == baltathar.address).true;
  });

  it('Should chill a validator', async () => {

    let call = polkadotApi.tx.staking.chill();
    const callTransaction = new Promise<{ block: string, address: string }>(async (resolve, reject) => {
      const unsub = await call.signAndSend(baltathar, {tip: 200, nonce: -1}, (result) => {
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

  });

  after(async () => {
    await killNodes();
  });
});