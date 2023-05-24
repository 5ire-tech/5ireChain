import { expect } from 'chai';
import { BLOCK_TIME } from '../utils/constants';
import {killNodes, polkadotApi, spawnNodes} from "../utils/util";
import {createType, U256, u8} from "@polkadot/types";
import {H160, H256} from "@polkadot/types/interfaces";
import {keccak_256} from "js-sha3";
/*import { SecretKey } from 'secp256k1';
import { randomBytes, createPublicKey, createPrivateKey, privateToPublic } from 'crypto';
import { ec  } from 'elliptic';*/
const EC = require('elliptic').ec;
const ec = new EC('secp256k1');
import RLP from 'rlp'
import {hexToBytes, sudoTx, sudoTx2} from "../utils/setup";
import * as fs from "fs";
import child from "child_process";
import {u8aToHex} from "@polkadot/util";
import {Keyring} from "@polkadot/api";
import {SignerPayloadRaw, Signer} from "@polkadot/types/types/extrinsic";
import {mnemonicGenerate} from "@polkadot/util-crypto";
import {KeypairType} from "@polkadot/util-crypto/types";

describe('EVM related tests', function () {
  this.timeout(300 * BLOCK_TIME);
  // 4 session.
  this.slow(40 * BLOCK_TIME);

  before(async () => {
    await spawnNodes()
  });

  // This test executes EVM contract transaction
  it('Should execute EVM contract', async () => {
    console.log("Executing EVM contract test");

    const alice = addressBuild(1);

    const nonce = polkadotApi.registry.createType("U256", "1");
    const max_priority_fee_per_gas = polkadotApi.registry.createType("U256", "1");
    const max_fee_per_gas = polkadotApi.registry.createType("U256", "1");
    const gas_limit = polkadotApi.registry.createType("U256", "0x100000");
    const value = polkadotApi.registry.createType("U256", "0");

    const gitRoot = child
      .execSync('git rev-parse --show-toplevel')
      .toString()
      .trim();
    const contractPath = `${gitRoot}/integration-test-suite/tests/contracts/erc20_contract_bytecode.txt`;
    // Read the file synchronously
    const erc20ContractBytecode = fs.readFileSync(contractPath, 'utf8');

    const EIP2930UnsignedTransactionMsgObj: EIP2930UnsignedTransactionMsg = {
      chain_id: 999,
      nonce: nonce,
      gas_price: max_fee_per_gas,
      gas_limit: gas_limit,
      action: 'TransactionAction::Create',
      value: value,
      input: Uint8Array.from(hexToBytes(erc20ContractBytecode.trimEnd())),
      access_list: new Uint8Array()
    };

    const hashedTransaction = hashTransaction(EIP2930UnsignedTransactionMsgObj);
    const signature = ec.sign(hashedTransaction, alice.private_key);


    const EIP2930Transaction = polkadotApi.createType(
      'EIP2930Transaction',
      {
        chain_id: 999,
        nonce: nonce,
        gas_price: max_fee_per_gas,
        gas_limit: gas_limit,
        action: 'Create',
        value: value,
        input: Uint8Array.from(hexToBytes(erc20ContractBytecode.trimEnd())),
        access_list: [],
        odd_y_parity: signature.recoveryParam != 0,
        r: signature.r.toArray(32),
        s: signature.s.toArray(32)
      }
    );

    const EIP2930TransactionType = polkadotApi.createType(
      'TransactionV2',
      {
        EIP2930: {
          chain_id: 999,
          nonce: nonce,
          gas_price: max_fee_per_gas,
          gas_limit: gas_limit,
          action: 'Create',
          value: value,
          input: Uint8Array.from(hexToBytes(erc20ContractBytecode.trimEnd())),
          access_list: [],
          odd_y_parity: signature.recoveryParam != 0,
          r: signature.r.toArray(32),
          s: signature.s.toArray(32)
        },
      }
    );

    const ethCall =
      polkadotApi.tx.ethereum.transact(
       EIP2930TransactionType
      );

    //await sudoTx2(u8aToHex(alice.address), polkadotApi, ethCall);
    await sudoTx(polkadotApi, ethCall);


    /*const mnemonic = mnemonicGenerate();
    const keyring = new Keyring({type: 'ed25519'});
    const signer = keyring.addFromUri(mnemonic);*/

   /* await polkadotApi.tx.sudo
      .sudo(ethCall)*/

   const keyring = new Keyring({ type: 'ethereum' });

    const sender = keyring.addFromUri('hello ethereum know');


    /*const txHash = await ethCall
      .signAndSend(sender);*/

    /*await polkadotApi.tx.ethereum
      .transact(EIP2930TransactionType)
      .signAndSend(signer, ({ status, events, dispatchError }) => {
        // status would still be set, but in the case of error we can shortcut
        // to just check it (so an error would indicate InBlock or Finalized)
        console.log("Status: ", status);
        if (status.isFinalized) {
          console.log("status is finalized" + JSON.stringify(status.toJSON()));
          //unsub();
       //   resolve();
        }

        if (dispatchError) {
          if (dispatchError.isModule) {
            // for module errors, we have the section indexed, lookup
            const decoded = polkadotApi.registry.findMetaError(dispatchError.asModule);
            const { docs, name, section } = decoded;

            console.log(`Module error here ${section}.${name}: ${docs.join(' ')}`);
          } else {
            // Other, CannotLookup, BadOrigin, no extra info
            console.log("Some other error " + dispatchError.toString());
          }
        }

        if(events) {
          console.log(`events here ${events}`);
        } else {
          console.log(`no events`);
        }
      });*/

    //Gets the pending ethereum block
    const currentBlockStr = await polkadotApi.query.ethereum.pending();
    const currentBlock = currentBlockStr.toJSON();
    console.log(currentBlock)

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


/*const myTransaction = createType('TransactionAction', TransactionAction.Create);

enum TransactionAction {
  Call,
  Create,
}*/
