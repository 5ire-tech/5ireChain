import { expect } from 'chai';
import { BLOCK_TIME } from '../utils/constants';
import {killNodes, polkadotApi as api, spawnNodes} from "../utils/util";
import {ApiPromise, Keyring, WsProvider} from "@polkadot/api";
import {addressToEvm} from "@polkadot/util-crypto";
import { KeyringPair } from '@polkadot/keyring/types';
import {waitForEvent} from "../utils/setup";

let wsProvider: WsProvider;
// Keyring needed to sign using Alice account
const keyring = new Keyring({ type: 'sr25519' });

// ByteCode of our ERC20 exemple: copied from ./truffle/contracts/MyToken.json
const ERC20_BYTECODES = require("./contracts/MyToken.json").bytecode;

describe('EVM related tests', function () {
  this.timeout(300 * BLOCK_TIME);

  before(async () => {
    await spawnNodes();
    wsProvider = new WsProvider('ws://127.0.0.1:9944');
  });

  // Should init and create contracts
  it('Should init and create contracts', async () => {
    const {  alice, bob, aliceEthAccount } = await init();

    await createContract(aliceEthAccount, alice)
  });

  after(async () => {
   await killNodes();
  });
});

// Setup the API and Alice Account
async function init() {
  console.log(`Initiating the API (ignore message "Unable to resolve type B..." and "Unknown types found...")`);

  // Initiate the polkadot API.
  const api = await ApiPromise.create({
    provider: wsProvider,
    types: {
      // mapping the actual specified address format
      Address: "AccountId",
      // mapping the lookup
      LookupSource: "AccountId",
      Account: {
        nonce: "U256",
        balance: "U256"
      },
      Transaction: {
        nonce: "U256",
        action: "String",
        gas_price: "u64",
        gas_limit: "u64",
        value: "U256",
        input: "Vec<u8>",
        signature: "Signature"
      },
      Signature: {
        v: "u64",
        r: "H256",
        s: "H256"
      }
    }
  });
  console.log(`Initialiation done`);
  console.log(`Genesis at block: ${api.genesisHash.toHex()}`);

  const alice = keyring.addFromUri('//Alice', { name: 'Alice default' });
  const bob = keyring.addFromUri('//Bob', { name: 'Bob default' });

  const aliceEthAccount = addressToEvm(alice.addressRaw);
  return { api, alice, bob, aliceEthAccount };
}

// Create the ERC20 contract from ALICE
async function createContract(evmAddress:any, alice: KeyringPair) {

  console.log(`\nStep 1: Creating Smart Contract`);

  const transaction = await api.tx.evm.create(evmAddress, ERC20_BYTECODES, 0, 10000000, 10000000, 1000000, 0, null);

  const contract = new Promise<{ block: string, address: string }>(async (resolve, reject) => {
    const unsub = await transaction.signAndSend(alice, {tip: 200, nonce: -1}, (result) => {
      console.log(`Contract creation is ${result.status}`);
      if (result.status.isInBlock) {
        console.log(`Contract included at blockHash ${result.status.asInBlock}`);
        console.log(`Waiting for finalization... (can take a minute)`);
      } else if (result.status.isFinalized) {
        const data = JSON.stringify(result.events);
        console.log(data);

        const dataStr = JSON.parse(data);

        const filteredData = dataStr.filter((item: any) => item.event.index === "0x3a01");
        const contractAddress = filteredData[0].event.data[0];
        expect(contractAddress).not.undefined;

        console.log(`Contract address: ${contractAddress}`);
        unsub();
        resolve({
          block: result.status.asFinalized.toString(),
          address: contractAddress
        });
      }
    });

    await waitForEvent(api, 'evm', 'Created')

  });
  return contract;
}

