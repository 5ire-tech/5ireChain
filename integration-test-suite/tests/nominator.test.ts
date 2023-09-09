import { expect } from 'chai';
import { BLOCK_TIME } from '../utils/constants';
import {killNodes,  polkadotApi, spawnNodes} from "../utils/util";
import {Keyring} from "@polkadot/api";
import {sleep, waitForEvent} from "../utils/setup";


// Keyring needed to sign
const keyring = new Keyring({ type: 'sr25519' });

describe('Nominator tests', function () {
  this.timeout(300 * BLOCK_TIME);
  // 4 session.
  this.slow(40 * BLOCK_TIME);

  before(async () => {
    await spawnNodes();
  });

  // Should stake to become a nominator
  it('Should stake to become a nominator', async () => {
    const ferdie = keyring.addFromUri('//Ferdie');
    const bob = keyring.addFromUri('//Bob');

    const initialValidatorsNominated = await polkadotApi.query.staking.nominators(ferdie.address);
    console.log(JSON.stringify(initialValidatorsNominated.toHuman()))
    // @ts-ignore
    expect(initialValidatorsNominated.toHuman() == null).true;

    const controller = polkadotApi.registry.createType("PalletStakingRewardDestination", "Staked");

    const amount = polkadotApi.createType('Balance', '900000000000000000000');
    let bondValidator = polkadotApi.tx.staking.bond(amount, controller);
    const bondValidatorTransaction = new Promise<{ block: string, address: string }>(async (resolve, reject) => {
      const unsub = await bondValidator.signAndSend(ferdie, {tip: 200, nonce: -1}, (result) => {
        console.log(`bond nominator transaction is ${result.status}`);
        if (result.status.isInBlock) {
          console.log(`bond nominator transaction included at blockHash ${result.status.asInBlock}`);
          console.log(`bond nominator transaction waiting for finalization... (can take a minute)`);
        } else if (result.status.isFinalized) {
          console.log( `bond nominator transaction events are ${result.events}`)
          console.log(`bond nominator transaction finalized at blockHash ${result.status.asFinalized}`);
          unsub();
        }
      });
    });
    await waitForEvent(polkadotApi, 'staking', 'Bonded');

    const nominatedAddress = polkadotApi.registry.createType("MultiAddress", bob.address);
    const prefs = polkadotApi.registry.createType("Vec<MultiAddress>", [bob.address]);

    let nominateValidator = polkadotApi.tx.staking.nominate(prefs);
    const nominateValidatorTransaction = new Promise<{ block: string, address: string }>(async (resolve, reject) => {
      const unsub = await nominateValidator.signAndSend(ferdie, {tip: 200, nonce: -1}, (result) => {
        console.log(`nominate validator transaction is ${result.status}`);
        if (result.status.isInBlock) {
          console.log(`nominate validator transaction included at blockHash ${result.status.asInBlock}`);
          console.log(`nominate validator transaction waiting for finalization... (can take a minute)`);
        } else if (result.status.isFinalized) {
          console.log( `nominate validator transaction events are ${result.events}`)
          console.log(`nominate validator transaction finalized at blockHash ${result.status.asFinalized}`);
          unsub();
        }
      });
    });
    await sleep(5000);

    const validatorsNominated = await polkadotApi.query.staking.nominators(ferdie.address);
    console.log(JSON.stringify(validatorsNominated.toHuman()))
    // @ts-ignore
    expect(validatorsNominated.toHuman().targets[0] == bob.address).true
  });

  it('Should unbond funds from a nominator', async () => {
    const ferdie = keyring.addFromUri('//Ferdie');

    const amount = polkadotApi.createType('Balance', '200000000000000000000');
    let unbondValidator = polkadotApi.tx.staking.unbond(amount);
    const unbondValidatorTransaction = new Promise<{ block: string, address: string }>(async (resolve, reject) => {
      const unsub = await unbondValidator.signAndSend(ferdie, {tip: 200, nonce: -1}, (result) => {
        console.log(`unbond nominator transaction is ${result.status}`);
        if (result.status.isInBlock) {
          console.log(`unbond nominator transaction included at blockHash ${result.status.asInBlock}`);
          console.log(`unbond nominator transaction waiting for finalization... (can take a minute)`);
        } else if (result.status.isFinalized) {
          console.log( `unbond nominator transaction events are ${result.events}`)
          console.log(`unbond nominator transaction finalized at blockHash ${result.status.asFinalized}`);
          unsub();
        }
      });
    });
    await waitForEvent(polkadotApi, 'staking', 'Unbonded');
  });

  it('Should chill a nominator', async () => {
    const ferdie = keyring.addFromUri('//Ferdie');

    let call = polkadotApi.tx.staking.chill();
    const callTransaction = new Promise<{ block: string, address: string }>(async (resolve, reject) => {
      const unsub = await call.signAndSend(ferdie, {tip: 200, nonce: -1}, (result) => {
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

    const validatorsNominated = await polkadotApi.query.staking.nominators(ferdie.address);
    console.log(JSON.stringify(validatorsNominated.toHuman()))
    // @ts-ignore
    expect(validatorsNominated.toHuman() == null).true
  });

  after(async () => {
    await killNodes();
  });
});
