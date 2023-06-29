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

const ERC20_BYTECODES = require("./contracts/MyToken.json").bytecode;

describe('EVM related tests', function () {
  this.timeout(300 * BLOCK_TIME);

  before(async () => {
    await spawnNodes();
  });

  // Should init and create contracts
  it('Should init, create, and execute contracts', async () => {
    const {  alice, bob, aliceEthAccount, bobEthAccount } = await init();

    console.log(`Bob Eth Account is ${bobEthAccount}`);

    const contract = await createContract(aliceEthAccount, alice);
    await transferToken(bytesToHex(Array.from(aliceEthAccount)), bytesToHex(Array.from(bobEthAccount)), bob, alice, contract.address);
    await approve(bytesToHex(Array.from(aliceEthAccount)),bytesToHex(Array.from(bobEthAccount)), alice, contract.address);
    await balanceOf(bytesToHex(Array.from(aliceEthAccount)),alice, contract.address);
    await totalSupply(bytesToHex(Array.from(aliceEthAccount)),alice, contract.address);

  });

  after(async () => {
   await killNodes();
  });
});

// Setup the API and Alice Account
async function init() {
  const alice = keyring.addFromUri('//Alice', { name: 'Alice default' });
  const bob = keyring.addFromUri('//Bob', { name: 'Bob default' });

  const aliceEthAccount = addressToEvm(alice.addressRaw);
  const bobEthAccount = addressToEvm(bob.addressRaw);
  return { alice, bob, aliceEthAccount, bobEthAccount };
}

// Create the ERC20 contract from ALICE
async function createContract(evmAddress:any, alice: KeyringPair) {

  console.log(`\n: Creating Smart Contract`);

  const source = evmAddress;
  const init = ERC20_BYTECODES;
  const value = 0;
  const gasLimit = 100_000_00;
  const maxFeePerGas = 100_000_000_000;
  const maxPriorityFeePerGas: BigInt =  BigInt(100_000_000);
  const nonce = 0;
  const accessList = null;

  const transaction = await api.tx.evm.create(source, init, value, gasLimit, maxFeePerGas, maxPriorityFeePerGas, nonce, accessList);

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

// Transfer tokens to Bob
async function transferToken(aliceEthAccount: string, bobEthAccount: string, bob: KeyringPair, alice: KeyringPair, contractAddress: string) {
  console.log(`Transfering Tokens to Bob EVM Account: ${bobEthAccount}`);

  console.log(`Preparing transfer of 0xdd`);
  const transferFnCode = `a9059cbb000000000000000000000000`;
  const tokensToTransfer = `00000000000000000000000000000000000000000000000000000000000000dd`;
  const inputCode = `0x${transferFnCode}${bobEthAccount.substring(2)}${tokensToTransfer}`;
  console.log(`Sending call input: ${inputCode}`);
  const gasLimit = 100_000_00;
  const maxFeePerGas = 100_000_000_000;
  const maxPriorityFeePerGas: BigInt =  BigInt(100_000_000);
  const nonce = 1;
  const accessList = null;
  const transaction = await api.tx.evm.call(aliceEthAccount, contractAddress, inputCode, 0, gasLimit, maxFeePerGas, maxPriorityFeePerGas, nonce, accessList);

  const data = new Promise<{  }>(async (resolve, reject) => {
    const unsub = await transaction.signAndSend(alice, (result) => {
      console.log(`Transfer is ${result.status}`);
      if (result.status.isInBlock) {
        console.log(`Transfer included at blockHash ${result.status.asInBlock}`);
        console.log(`Waiting for finalization... (can take a minute)`);
      } else if (result.status.isFinalized) {
        console.log(`Transfer finalized at blockHash ${result.status.asFinalized}`);
        const data = JSON.stringify(result.events);
        console.log(data);
        unsub();
        resolve({});
      }
    });
  });

  await waitForEvent(api, 'evm', 'Executed');

  return data;
}

//Balance of Bob
async function balanceOf(aliceEthAccount: string, alice: KeyringPair, contractAddress: string) {

  console.log(`Balance of Alice EVM Account: ${aliceEthAccount}`);
  const BalanceOfFnCode = `70a08231000000000000000000000000`;
  const inputCode = `0x${BalanceOfFnCode}${aliceEthAccount.substring(2)}`;
  console.log(`Sending call input: ${inputCode}`);
  const gasLimit = 100_000_00;
  const maxFeePerGas = 100_000_000_000;
  const maxPriorityFeePerGas: BigInt =  BigInt(100_000_000);
  const nonce = 3;
  const accessList = null;
  const transaction = await api.tx.evm.call(aliceEthAccount, contractAddress, inputCode, 0, gasLimit, maxFeePerGas, maxPriorityFeePerGas, nonce, accessList);

  const data = new Promise<{  }>(async (resolve, reject) => {
    const unsub = await transaction.signAndSend(alice, (result) => {
      console.log(`BalanceOf is ${result.status}`);
      if (result.status.isInBlock) {
        console.log(`BalanceOf included at blockHash ${result.status.asInBlock}`);
        console.log(`Waiting for finalization... (can take a minute)`);
      } else if (result.status.isFinalized) {
        console.log(`BalanceOf finalized at blockHash ${result.status.asFinalized}`);
        const data = JSON.stringify(result.events);
        console.log(data);
        unsub();
        resolve({});
      }
    });
  });
  await waitForEvent(api, 'evm', 'Executed');

  return data;
}

//Total Supply
async function totalSupply(aliceEthAccount: string, alice: KeyringPair, contractAddress: string) {

  
  const totalSupplyFnCode = `18160ddd`;
  const inputCode = `0x${totalSupplyFnCode}`;
  console.log(`Sending call input: ${inputCode}`);
  const gasLimit = 100_000_00;
  const maxFeePerGas = 100_000_000_000;
  const maxPriorityFeePerGas: BigInt =  BigInt(100_000_000);
  const nonce = 4;
  const accessList = null;
  const transaction = await api.tx.evm.call(aliceEthAccount, contractAddress, inputCode, 0, gasLimit, maxFeePerGas, maxPriorityFeePerGas, nonce, accessList);

  const data = new Promise<{  }>(async (resolve, reject) => {
    const unsub = await transaction.signAndSend(alice, (result) => {
      console.log(`totalSupply is ${result.status}`);
      if (result.status.isInBlock) {
        console.log(`totalSupply included at blockHash ${result.status.asInBlock}`);
        console.log(`Waiting for finalization... (can take a minute)`);
      } else if (result.status.isFinalized) {
        console.log(`totalSupply finalized at blockHash ${result.status.asFinalized}`);
        const data = JSON.stringify(result.events);
        console.log(data);
        unsub();
        resolve({});
      }
    });
  });
  await waitForEvent(api, 'evm', 'Executed');

  return data;
}



// Approve
async function approve(aliceEthAccount: string, bobEthAccount: string, alice: KeyringPair, contractAddress: string) {

  const approveFnCode = `daea85c5000000000000000000000000`;
  const tokensToApprove = `00000000000000000000000000000000000000000000000000000000000000dd`;
  const inputCode = `0x${approveFnCode}${aliceEthAccount.substring(2)}${tokensToApprove}`;
  console.log(`Sending call input: ${inputCode}`);
  const gasLimit = 100_000_00;
  const maxFeePerGas = 100_000_000_000;
  const maxPriorityFeePerGas: BigInt =  BigInt(100_000_000);
  const nonce = 2;
  const accessList = null;
  const transaction = await api.tx.evm.call(aliceEthAccount, contractAddress, inputCode, 0, gasLimit, maxFeePerGas, maxPriorityFeePerGas, nonce, accessList);

  const data = new Promise<{  }>(async (resolve, reject) => {
    const unsub = await transaction.signAndSend(alice, (result) => {
      console.log(`approve is ${result.status}`);
      if (result.status.isInBlock) {
        console.log(`Apprpve included at blockHash ${result.status.asInBlock}`);
        console.log(`Waiting for finalization... (can take a minute)`);
      } else if (result.status.isFinalized) {
        console.log(`Approve finalized at blockHash ${result.status.asFinalized}`);
        const data = JSON.stringify(result.events);
        console.log(data);

        unsub();

        resolve({});

      }
    });
  });

  await waitForEvent(api, 'evm', 'Executed');

  return data;
}
