import { expect } from 'chai';
import { BLOCK_TIME } from '../utils/constants';
import {killNodes, polkadotApi as api, spawnNodes} from "../utils/util";
import {Keyring} from "@polkadot/api";
import {addressToEvm} from "@polkadot/util-crypto";
import { KeyringPair } from '@polkadot/keyring/types';
import {waitForEvent} from "../utils/setup";
import {bytesToHex} from "web3-utils";
// Keyring needed to sign using Alice account
const keyring = new Keyring({ type: 'sr25519' });

describe('EVM withdraw test', function () {
  this.timeout(300 * BLOCK_TIME);

  before(async () => {
    await spawnNodes();
  });

  // Should do evm deposit
  it('Should do evm withdrawal', async () => {
    const {alice, bob, aliceEthAccount} = await init();
    await withdrawInEVM(aliceEthAccount, alice, bob);
  });

  after(async () => {
    await killNodes();
  });

  // Setup the API and Alice Account
  async function init() {
    const alice = keyring.addFromUri('//Alice', {name: 'Alice default'});
    const bob = keyring.addFromUri('//Bob', {name: 'Bob default'});

    const aliceEthAccount = addressToEvm(alice.addressRaw);
    const bobEthAccount = addressToEvm(bob.addressRaw);

    console.log(`alice evm address ${bytesToHex(Array.from(aliceEthAccount))}`)
    console.log(`bob evm address ${bytesToHex(Array.from(bobEthAccount))}`)

    console.log(`alice  address ${bytesToHex(Array.from(alice.addressRaw))}`)
    console.log(`bob address ${bytesToHex(Array.from(bob.addressRaw))}`)

    console.log(`alice  address ${alice.address}`)
    console.log(`bob address ${bob.address}`)
    return {alice, bob, aliceEthAccount};
  }

  // Withdraw in evm
  async function withdrawInEVM(aliceEthAccount: Uint8Array, alice: KeyringPair, bob: KeyringPair) {
    // Retrieve the account balance & nonce for Alice
    // @ts-ignore
    const { data: aliceInitialBalance } = await api.query.system.account(alice.address);
    console.log(`alice initial balance is ${aliceInitialBalance.free.toHuman()}`);

    const address = aliceEthAccount;
    const value = api.createType("Balance", "800000000000000000");

    const bobEthAccount = addressToEvm(bob.addressRaw);

    const withdraw = await api.tx.evm.withdraw(aliceEthAccount, value);

    const transaction = new Promise<{}>(async (resolve, reject) => {
      const unsub = await withdraw.signAndSend(alice, {tip: 200000000, nonce: -1}, (result) => {
        console.log(`EVM Withdrawal is ${result.status}`);
        if (result.status.isInBlock) {
          console.log(`EVM Withdrawal included at blockHash ${result.status.asInBlock}`);
          console.log(`Waiting for finalization... (can take a minute)`);
        } else if (result.status.isFinalized) {
          const data = JSON.stringify(result.events);
          console.log(data);

          const dataStr = JSON.parse(data);

          unsub();
          resolve({});
        }
      });
    });

    await waitForEvent(api, 'balances', 'Transfer');

    // Retrieve the account balance for Alice
    // @ts-ignore
    const { data: aliceBalance} = await api.query.system.account(alice.address);
    console.log(`alice balance is ${aliceBalance.free.toHuman()}`);
    expect(  aliceBalance.free.toBigInt() > aliceInitialBalance.free.toBigInt()).true;
  }
});
