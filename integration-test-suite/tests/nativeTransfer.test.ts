import { expect } from 'chai';
import { BLOCK_TIME, alith, baltathar } from '../utils/constants';
import {killNodes, polkadotApi as api, polkadotApi, spawnNodes} from "../utils/util";
import {waitForEvent} from "../utils/setup";



describe('Native token tests', function () {
  this.timeout(300 * BLOCK_TIME);
  // 4 session.
  this.slow(40 * BLOCK_TIME);

  before(async () => {
    await spawnNodes();
  });

  // Should transfer native token
  it('Should transfer native tokens', async () => {


    // Retrieve the account balance & nonce for Alice
    // @ts-ignore
    const { nonce: aliceInitialNonce, data: aliceInitialBalance } = await polkadotApi.query.system.account(alith.address);
    // Retrieve the account balance & nonce for Bob
    // @ts-ignore
    const { nonce: bobInitialNonce, data: bobInitialBalance } = await polkadotApi.query.system.account(baltathar.address);
    // assert that alice initial balance is same as bob initial balance
    expect(aliceInitialBalance.free.toBigInt() == bobInitialBalance.free.toBigInt()).true;

    // Create a extrinsic, transferring 12345 units to Bob
    const amount = polkadotApi.createType('Balance', '900000000000000000000');
    const transaction = polkadotApi.tx.balances.transfer(baltathar.address, amount);

    const transfer = new Promise<{ block: string, address: string }>(async (resolve, reject) => {
      const unsub = await transaction.signAndSend(alith, {tip: 200, nonce: -1}, (result) => {
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
    });

    await waitForEvent(api, 'balances', 'Transfer');

    // @ts-ignore
    const { nonce: aliceNonce, data: aliceBalance } = await polkadotApi.query.system.account(alith.address);

    // Retrieve the account balance & nonce for Bob
    // @ts-ignore
    const { nonce: bobNonce, data: bobBalance } = await polkadotApi.query.system.account(baltathar.address);
    expect(aliceInitialBalance.free.toBigInt() > aliceBalance.free.toBigInt()).true;
    expect(bobInitialBalance.free.toBigInt() < bobBalance.free.toBigInt()).true;

    expect(bobBalance.free.toBigInt() > aliceBalance.free.toBigInt()).true;
  });


  after(async () => {
    await killNodes();
  });
});