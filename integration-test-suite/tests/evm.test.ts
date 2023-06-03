import { expect } from 'chai';
import { BLOCK_TIME } from '../utils/constants';
import {killNodes, polkadotApi, spawnNodes} from "../utils/util";
import {createType, GenericEthereumAccountId, U256, u8} from "@polkadot/types";
import {AccountId, AccountId20, H160, H256} from "@polkadot/types/interfaces";
import {keccak_256} from "js-sha3";
/*import { SecretKey } from 'secp256k1';
import { randomBytes, createPublicKey, createPrivateKey, privateToPublic } from 'crypto';
import { ec  } from 'elliptic';*/
const EC = require('elliptic').ec;
const ec = new EC('secp256k1');
import RLP from 'rlp'
import {hexToBytes, sudoTx, waitForEvent} from "../utils/setup";
import * as fs from "fs";
import child from "child_process";
import {hexToU8a, u8aToHex} from "@polkadot/util";
import {ApiPromise, Keyring, WsProvider} from "@polkadot/api";
import {SignerPayloadRaw, Signer, SignatureOptions} from "@polkadot/types/types/extrinsic";
import {addressToEvm, evmToAddress, mnemonicGenerate} from "@polkadot/util-crypto";
import {KeypairType} from "@polkadot/util-crypto/types";
import {SignerPayloadRawBase} from "@polkadot/types/types";
import * as web3Utils from 'web3-utils';
import * as crypto from '@polkadot/util-crypto';
import { KeyringPair } from '@polkadot/keyring/types';

let wsProvider: WsProvider;
// Keyring needed to sign using Alice account
const keyring = new Keyring({ type: 'sr25519' });

// ByteCode of our ERC20 exemple: copied from ./truffle/contracts/MyToken.json
const ERC20_BYTECODES = require("./contracts/MyToken.json").bytecode;

describe('EVM related tests', function () {
  this.timeout(300 * BLOCK_TIME);
  // 4 session.
  this.slow(40 * BLOCK_TIME);

  before(async () => {
    await spawnNodes();
    wsProvider = new WsProvider('ws://127.0.0.1:9944');
  });

  // Should init and create contracts
  it('Should init and create contracts', async () => {
    const { api, alice, bob, aliceEthAccount } = await init();

    // step 1: Creating the contract from ALICE
    const contractAccount = await step1(aliceEthAccount, api, alice)
  });

  after(async () => {
   await killNodes();
  });
});

interface AccountInfo {
  address: Uint8Array; // 20 bytes
  account_id: Uint8Array; // 32 bytes
  private_key: Uint8Array; // 32  bytes
}

function addressBuild(seed: number): AccountInfo {
  const private_key = new Uint8Array(32);
  private_key.set(new Uint8Array(32).fill(seed + 1));

  const public_key = ec.keyFromPrivate(private_key).getPublic();

  const address = new Uint8Array(20);
  address.set(keccak_256.array(public_key.encode("hex")).slice(0, 20));

  const data = new Uint8Array(32);
  data.set(address.slice(0, 20));

  return {
    private_key,
    account_id: data,
    address,
  };
}

function contractAddress(sender: Uint8Array, nonce: number): Uint8Array {
  const senderRlp = RLP.encode(sender);
  const nonceRlp = RLP.encode(nonce);
  const appendedArrays = appendUint8Arrays(senderRlp, nonceRlp);


  const hash = keccak_256.arrayBuffer(appendedArrays);
  const address = new Uint8Array(hash, 12);
  return address;
}

function appendUint8Arrays(arr1: Uint8Array, arr2: Uint8Array): Uint8Array {
  const combinedLength = arr1.length + arr2.length;
  const combinedArray = new Uint8Array(combinedLength);
  combinedArray.set(arr1, 0);
  combinedArray.set(arr2, arr1.length);
  return combinedArray;
}

function hashTransaction(message: EIP2930UnsignedTransactionMsg): Uint8Array {
  const chainIdRlp = RLP.encode(message.chain_id);
  const nonceRlp = RLP.encode(message.nonce.toU8a());
  const gasPriceRlp = RLP.encode(message.gas_price.toU8a());
  const gasLimitRlp = RLP.encode(message.gas_limit.toU8a());
  const actionRlp = RLP.encode(message.action);
  const valuePriceRlp = RLP.encode(message.value.toU8a());
  const inputRlp = RLP.encode(message.input);
  const accessListRlp = RLP.encode(message.access_list);

  const concatMessage: Uint8Array = Uint8Array.from([
    ...chainIdRlp,
    ...nonceRlp,
    ...gasPriceRlp,
    ...gasLimitRlp,
    ...actionRlp,
    ...valuePriceRlp,
    ...inputRlp,
    ...accessListRlp
  ]);

  const out = new Uint8Array(1 + concatMessage.length);
  out[0] = 1;
  out.set(concatMessage, 1);
  const hash = keccak_256.digest(out);
  return Uint8Array.from(hash);
}




interface EIP2930UnsignedTransaction {
  nonce: U256; //4 bytes
  gas_price: U256;// 4 bytes
  gas_limit: U256; // 4 bytes
  action: 'TransactionAction::Create';
  value: U256;
  input: Uint8Array;
}

interface EIP2930UnsignedTransactionMsg {
  chain_id: number;
  nonce: U256; //4 bytes
  gas_price: U256;// 4 bytes
  gas_limit: U256; // 4 bytes
  action: 'TransactionAction::Create';
  value: U256;
  input: Uint8Array;
  access_list: Uint8Array;
}

interface EIP2930Transaction {
  nonce: U256; //4 bytes
  max_priority_fee_per_gas: U256;// 4 bytes
  max_fee_per_gas: U256; // 4 bytes
  gas_limit: U256; // 4 bytes
  action: 'TransactionAction::Create';
  value: U256;
  input: Uint8Array;
  access_list: Uint8Array;
  odd_y_parity: boolean;
  r: Uint8Array; // 32 bytes
  s: Uint8Array; //32 bytes
}

function padZeroes(numberArray: number[]): number[] {
  const targetLength = 33;
  const currentLength = numberArray.length;

  if (currentLength >= targetLength) {
    return numberArray;
  }

  const diff = targetLength - currentLength;
  const paddedZeroes = Array(diff).fill(0);
  return paddedZeroes.concat(numberArray);
}


function bytesToHex(bytes: Uint8Array | number[]): string {
  return Array.from(bytes)
    .map((byte) => byte.toString(16).padStart(2, '0'))
    .join('');
}


/*const myTransaction = createType('TransactionAction', TransactionAction.Create);

enum TransactionAction {
  Call,
  Create,
}*/

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

  // @ts-ignore
  const { nonce, data: balance } = await api.query.system.account(alice.address);
  console.log(`Alice Substrate Account: ${alice.address} ${alice.addressRaw} ${hexToU8a(alice.address)}`);
  console.log(`Alice Substrate Account (nonce: ${nonce}) balance, free: ${balance.free.toHuman()}`);

  const aliceEvmAccount = `0x${crypto.blake2AsHex(crypto.decodeAddress(alice.addressRaw), 256).substring(26)}`;
  const aliceEthAccount = addressToEvm(alice.addressRaw);
  console.log(`Alice EVM Account: ${aliceEvmAccount} ${hexToU8a(aliceEvmAccount)}`);
  console.log(`Alice ETH Account: ${aliceEthAccount}`);
  const aliceSubstrate= evmToAddress(aliceEthAccount, 0, 'keccak');
  console.log(`Alice Account: ${aliceSubstrate} ${hexToU8a(aliceSubstrate)}`);
  console.log(`Alice Account 2: ${evmToAddress(aliceEthAccount)} ${hexToU8a(evmToAddress(aliceEthAccount))}`);
  const evmData = (await api.query.evm.accountCodes(aliceEthAccount)) as any;
  console.log(`Alice EVM Account (data: ${evmData})`);


  // Create a extrinsic, transferring 12345 units to Bob
  const transfer = api.tx.balances.transfer(aliceEthAccount, 800000000000000);

  // Sign and send the transaction using our account
  const hash = await transfer.signAndSend(alice, {tip: 100, nonce: -1});

  console.log('Transfer sent with hash', hash.toHex());

  // Create a extrinsic, transferring 12345 units to Bob
 /* const transfer2 = api.tx.balances.transfer(evmToAddress(aliceEthAccount, 0, 'keccak'), 12345000000);

  // Sign and send the transaction using our account
  const hash2 = await transfer2.signAndSend(alice);

  console.log('Transfer sent with hash', hash2.toHex());*/
  return { api, alice, bob, aliceEthAccount };
}

// Create the ERC20 contract from ALICE
async function step1(evmAddress:any, api: ApiPromise, alice: KeyringPair) {

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
        console.log(`contract address from filter ${contractAddress}`);
        expect(contractAddress).not.undefined;

        console.log(`Contract finalized at blockHash ${result.status.asFinalized}`);
        console.log(`Contract address: ${contractAddress}`);
        unsub();
        resolve({
          block: result.status.asFinalized.toString(),
          address: contractAddress
        });
      }
    });

    //await waitForEvent(polkadotApi, 'evm', 'Created')

  });
  return contract;
}

