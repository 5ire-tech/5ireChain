import { expect } from 'chai';
import { BLOCK_TIME } from '../utils/constants';
import {killNodes, polkadotApi, spawnNodes} from "../utils/util";
import {Keyring} from "@polkadot/api";
import {addressToEvm} from "@polkadot/util-crypto";
import { KeyringPair } from '@polkadot/keyring/types';
import {bytesToHex} from "web3-utils";
import { Web3 } from "web3";
import {sleep} from "../utils/setup";


// Keyring needed to sign using Alice account
const keyring = new Keyring({ type: 'sr25519' });
const ERC20_BYTECODES = require("./contracts/MyToken.json").bytecode;

describe('EVM withdraw test', function () {
  this.timeout(300 * BLOCK_TIME);

  before(async () => {
    await spawnNodes();
  });

  // Should do evm deposit
  it('Should test negative scenerios of Swap EVM to Native', async () => {
    const {alice, bob, aliceEthAccount} = await init();
    const web3 = new Web3(
        new Web3.providers.HttpProvider("http://127.0.0.1:9933")
      );
    await BalanceLow(aliceEthAccount, alice);
    await BadOrigin(aliceEthAccount, bob);


  });

  after(async () => {
    await killNodes();
  });
});
  // Setup the API and Alice Account
  async function init() {
    const alice = keyring.addFromUri('//Alice', {name: 'Alice default'});
    const bob = keyring.addFromUri('//Bob', {name: 'Bob default'});

    const aliceEthAccount = addressToEvm(alice.addressRaw);
    const bobEthAccount = addressToEvm(bob.addressRaw);
    return {alice, bob, aliceEthAccount};
  }

  // Negative test for Error BalanceLow
  async function BalanceLow(aliceEthAccount: Uint8Array, alice: KeyringPair) {
    const amount = polkadotApi.createType("Balance", "1000000000000000000000000000000000");
    const transaction = await polkadotApi.tx.evm.withdraw(aliceEthAccount, amount);

    const unsub = await transaction.signAndSend(alice,  {tip: 200000000, nonce: -1}, (result) => {
      console.log(`Swap is ${result.status}`);
      if (result.status.isInBlock) {
        console.log(`Swap included at blockHash ${result.status.asInBlock}`);
        console.log(`Waiting for finalization... (can take a minute)`);
      } else if (result.status.isFinalized) {
        console.log(`events are ${result.events}`);
        console.log(`Swap finalized at blockHash ${result.status.asFinalized}`);

        const data = JSON.stringify(result.events);
        const dataStr = JSON.parse(data);
        const filteredData = dataStr.filter((item: any) => item.event.index === "0x0001");
        expect(filteredData[0].event.data[0].module.error).to.equal("0x02000000");
        console.log(`Error found: ${filteredData[0].event.data[0].module.error}`);

        unsub();
      }
    });
    await sleep(12000);
  }

   // Negative test for Error BadOrigin
   async function BadOrigin(aliceEthAccount: Uint8Array, alice: KeyringPair) {
    const amount = polkadotApi.createType("Balance", "1000000000000000000000000000000000");
    const transaction = await polkadotApi.tx.evm.withdraw(aliceEthAccount, amount);

    const unsub = await transaction.signAndSend(alice,  {tip: 200000000, nonce: -1}, (result) => {
      console.log(`Swap is ${result.status}`);
      if (result.status.isInBlock) {
        console.log(`Swap included at blockHash ${result.status.asInBlock}`);
        console.log(`Waiting for finalization... (can take a minute)`);
      } else if (result.status.isFinalized) {
        console.log(`events are ${result.events}`);
        console.log(`Swap finalized at blockHash ${result.status.asFinalized}`);

        const data = JSON.stringify(result.events);
        const dataStr = JSON.parse(data);
        const filteredData = dataStr.filter((item: any) => item.event.index === "0x0001");
        expect(filteredData[0].event.data[0].badOrigin === null);
        console.log(`Error found: ${filteredData[0].event.data[0].badOrigin}`);

        unsub();
      }
    });
    await sleep(12000);
  }
  