import { expect } from 'chai';
import { BLOCK_TIME } from '../utils/constants';
import {killNodes, polkadotApi, spawnNodes} from "../utils/util";
import {ApiPromise, Keyring, WsProvider} from "@polkadot/api";
import {sudoTx} from "../utils/setup";

let wsProvider: WsProvider;
// Keyring needed to sign using Alice account
const keyring = new Keyring({ type: 'sr25519' });

describe('Native token tests', function () {
  this.timeout(300 * BLOCK_TIME);
  // 4 session.
  this.slow(40 * BLOCK_TIME);

  before(async () => {
    await spawnNodes();
  });

  // Should transfer native token
  it('Should transfer native tokens', async () => {
    const alice = keyring.addFromUri('//Alice');
    const bob = keyring.addFromUri('//Dave');
    const BOB = '5DTestUPts3kjeXSTMyerHihn1uwMfLj8vU8sqF7qYrFabHE';

    // Retrieve the account balance & nonce for Alice
    // @ts-ignore
    const { nonce: aliceInitialNonce, data: aliceInitialBalance } = await polkadotApi.query.system.account(alice.address);
    console.log(`balance of Alice ${aliceInitialBalance.free} and a nonce of ${aliceInitialNonce}`);

    // Retrieve the account balance & nonce for Bob
    // @ts-ignore
    const { nonce: bobInitialNonce, data: bobInitialBalance } = await polkadotApi.query.system.account(bob.address);
    console.log(`balance of Bob ${bobInitialBalance.free} and a nonce of ${bobInitialNonce}`);

    // assert that alice initial balance is same as bob initial balance
    //expect(aliceInitialBalance == bobInitialBalance).true;

    // Create a extrinsic, transferring 12345 units to Bob
    const transaction = polkadotApi.tx.balances.transfer(bob.address, 9000);

    /*const transfer = new Promise<{ block: string, address: string }>(async (resolve, reject) => {
      const unsub = await transaction.signAndSend(alice, {tip: 200, nonce: -1}, (result) => {
        console.log(`transfer is ${result.status}`);
        if (result.status.isInBlock) {
          console.log(`transfer included at blockHash ${result.status.asInBlock}`);
          console.log(`Waiting for finalization... (can take a minute)`);
        } else if (result.status.isFinalized) {
          console.log( `events are ${result.events}`)
          console.log(`Transfer finalized at blockHash ${result.status.asFinalized}`);
          unsub();
        }
      });
    });*/

    await sudoTx(polkadotApi, transaction);

    // @ts-ignore
    const { nonce: aliceNonce, data: aliceBalance } = await polkadotApi.query.system.account(alice.address);
    console.log(`balance of Alice ${aliceBalance.free} and a nonce of ${aliceNonce}`);

    // Retrieve the account balance & nonce for Bob
    // @ts-ignore
    const { nonce: bobNonce, data: bobBalance } = await polkadotApi.query.system.account(bob.address);
    console.log(`balance of Bob ${bobBalance.free} and a nonce of ${bobNonce}`);

    expect(aliceInitialBalance.free > aliceBalance.free).true;
    expect(bobInitialBalance.free < bobBalance.free).true;

    expect(bobBalance.free > aliceBalance.free).true;

  });


  after(async () => {
    await killNodes();
  });
});
