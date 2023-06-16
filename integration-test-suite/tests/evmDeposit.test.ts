import { expect } from 'chai';
import { BLOCK_TIME } from '../utils/constants';
import {killNodes, polkadotApi as api, spawnNodes} from "../utils/util";
import {ApiPromise, Keyring, WsProvider} from "@polkadot/api";
import {addressToEvm} from "@polkadot/util-crypto";
import { KeyringPair } from '@polkadot/keyring/types';
import {waitForEvent} from "../utils/setup";
import {bytesToHex} from "web3-utils";
// Keyring needed to sign using Alice account
const keyring = new Keyring({ type: 'sr25519' });

const ERC20_BYTECODES = require("./contracts/MyToken.json").bytecode;

describe('EVM deposit test', function () {
  this.timeout(300 * BLOCK_TIME);

  before(async () => {
    await spawnNodes();
  });

  // Should do evm deposit
  it('Should do evm deposit', async () => {
    const {alice, bob, aliceEthAccount} = await init();
    await depositInEVM(aliceEthAccount, alice, bob);
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

  // Deposit into evm
  async function depositInEVM(aliceEthAccount: Uint8Array, alice: KeyringPair, bob: KeyringPair) {
    // Retrieve the account balance & nonce for Alice
    // @ts-ignore
    const { data: bobInitialBalance } = await api.query.system.account(bob.address);
    console.log(`bob initial balance is ${bobInitialBalance.free.toHuman()}`);

    const address = aliceEthAccount;

    const deposit = await api.tx.evm.deposit(address, 7000000);

    const transaction = new Promise<{}>(async (resolve, reject) => {
      const unsub = await deposit.signAndSend(bob, {tip: 2000, nonce: -1}, (result) => {
        console.log(`EVM Deposit is ${result.status}`);
        if (result.status.isInBlock) {
          console.log(`EVM Deposit included at blockHash ${result.status.asInBlock}`);
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
    const { data: bobBalance} = await api.query.system.account(bob.address);
    console.log(`bob balance is ${bobBalance.free.toHuman()}`);

    expect(bobBalance.free.toBigInt() < bobInitialBalance.free.toBigInt()).true;
  }
});
