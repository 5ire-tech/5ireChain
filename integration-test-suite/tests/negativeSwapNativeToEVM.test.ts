import { expect } from 'chai';
import { BLOCK_TIME } from '../utils/constants';
import {killNodes, polkadotApi as api, spawnNodes} from "../utils/util";
import {Keyring} from "@polkadot/api";
import {addressToEvm} from "@polkadot/util-crypto";
import { KeyringPair } from '@polkadot/keyring/types';
import {sleep} from "../utils/setup";
import {bytesToHex} from "web3-utils";
// Keyring needed to sign using Alice account
const keyring = new Keyring({ type: 'sr25519' });


describe.only('Negative Swap Native to EVM', function () {
  this.timeout(300 * BLOCK_TIME);

  before(async () => {
    await spawnNodes();
  });

  // Balance low deposit to evm
  it('Should not  Swap Native to EVM due to insufficient native balance  ', async () => {
    const {alice, bob, aliceEthAccount} = await init();
    await BalanceLowDepositToEvm(aliceEthAccount, bob);

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
});
  // Error BalanceLow while Swap Native to EVM
  async function BalanceLowDepositToEvm(aliceEthAccount: Uint8Array, bob: KeyringPair) {
    // Retrieve the account balance & nonce for bob
    // @ts-ignore
    const { data: bobInitialBalance } = await api.query.system.account(bob.address);
    console.log(`bob initial balance is ${bobInitialBalance.free.toHuman()}`);

    const address = aliceEthAccount;
    const amount = api.createType("Balance", "1000000000000000000000000000000000");
    const deposit = await api.tx.evm.deposit(address,amount);
      const unsub = await deposit.signAndSend(bob, {tip: 2000, nonce: -1}, (result) => {
        console.log(`Swap Native to EVM is ${result.status}`);
        if (result.status.isInBlock) {
          console.log(`Swap Native to EVM included at blockHash ${result.status.asInBlock}`);
          console.log(`Waiting for finalization... (can take a minute)`);
        } else if (result.status.isFinalized) {
          const data = JSON.stringify(result.events);
          const dataStr = JSON.parse(data);
          const filteredData = dataStr.filter((item: any) => item.event.index === "0x0001");
          expect(filteredData[0].event.data[0].arithmetic).to.equal("Underflow");
          unsub();
        }
      });
    await sleep(12000);
    }
