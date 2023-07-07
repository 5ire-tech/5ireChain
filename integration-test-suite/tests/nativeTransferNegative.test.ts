import { expect } from 'chai';
import { BLOCK_TIME } from '../utils/constants';
import {killNodes, polkadotApi as api, polkadotApi, spawnNodes} from "../utils/util";
import {Keyring} from "@polkadot/api";
import {waitForEvent} from "../utils/setup";
import {encodeAddress} from "@polkadot/util-crypto";


// Keyring needed to sign using Alice account
const keyring = new Keyring({ type: 'sr25519' });

describe('Negative Native token tests', function () {
  this.timeout(300 * BLOCK_TIME);
  // 4 session.
  this.slow(40 * BLOCK_TIME);

  before(async () => {
    await spawnNodes();
  });

  it('Insufficient Balance while transferring native token', async () => {
    const alice = keyring.addFromUri('//Alice');
    const bob = keyring.addFromUri('//Bob');

    // Retrieve the account balance & nonce for Alice
    // @ts-ignore
    const { nonce: aliceInitialNonce, data: aliceInitialBalance } = await polkadotApi.query.system.account(alice.address);
    // Retrieve the account balance & nonce for Bob
    // @ts-ignore
    const { nonce: bobInitialNonce, data: bobInitialBalance } = await polkadotApi.query.system.account(bob.address);
    // assert that alice initial balance is same as bob initial balance
    expect(aliceInitialBalance.free.toBigInt() == bobInitialBalance.free.toBigInt()).true;

    // Create a extrinsic, transferring 12345 units to Bob
    const amount = polkadotApi.createType('Balance', '90000000000000000000000000000');
    const transaction = polkadotApi.tx.balances.transfer(bob.address, amount);

    const transfer = new Promise<{ block: string }>(async (resolve, reject) => {
      const unsub = await transaction.signAndSend(alice, {tip: 200, nonce: -1}, (result) => {
        console.log(`transfer is ${result.status}`);
        if (result.status.isInBlock) {
          console.log(`transfer included at blockHash ${result.status.asInBlock}`);
          console.log(`Waiting for finalization... (can take a minute)`);
        } else if (result.status.isFinalized) {
          console.log( `events are ${result.events}`)
          console.log(`Transfer finalized at blockHash ${result.status.asFinalized}`);

          const data = JSON.stringify(result.events);
          const dataStr = JSON.parse(data);

          const filteredData = dataStr.filter((item: any) => item.event.index === "0x0001");
          expect(filteredData[0].event.data[0].module.index == 6).true; //EVM
          expect(filteredData[0].event.data[0].module.error == '0x02000000').true; //InvalidNonce, index 5

          unsub();
          resolve({
            block: result.status.asFinalized.toString(),
          });
        }
      });
    });

    return transfer;
  });

  it('Existential Deposit error while transferring native token', async () => {
    const alice = keyring.addFromUri('//Alice');
    const bob = keyring.addFromUri('//Bob');

    // Create a extrinsic, transferring 12345 units to Bob
    const amount = polkadotApi.createType('Balance', '900000000000000000');

    const publicKey = new Uint8Array([...Array(32)].map(() => Math.floor(Math.random() * 256)));

    const randomAddress = encodeAddress(publicKey);
    const transaction = polkadotApi.tx.balances.transfer(randomAddress, amount);

    const transfer = new Promise<{ block: string }>(async (resolve, reject) => {
      const unsub = await transaction.signAndSend(alice, {tip: 200, nonce: -1}, (result) => {
        console.log(`transfer is ${result.status}`);
        if (result.status.isInBlock) {
          console.log(`transfer included at blockHash ${result.status.asInBlock}`);
          console.log(`Waiting for finalization... (can take a minute)`);
        } else if (result.status.isFinalized) {
          console.log( `events are ${result.events}`)
          console.log(`Transfer finalized at blockHash ${result.status.asFinalized}`);

          const data = JSON.stringify(result.events);
          const dataStr = JSON.parse(data);

          const filteredData = dataStr.filter((item: any) => item.event.index === "0x0001");
          expect(filteredData[0].event.data[0].module.index == 6).true;
          expect(filteredData[0].event.data[0].module.error == '0x03000000').true; //ExistentialDepositError

          unsub();
          resolve({
            block: result.status.asFinalized.toString(),
          });
        }
      });
    });

    return transfer;
  });


  after(async () => {
    await killNodes();
  });
});
