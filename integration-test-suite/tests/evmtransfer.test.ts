import { expect } from 'chai';
import { BLOCK_TIME } from '../utils/constants';
import {killNodes, polkadotApi, spawnNodes} from "../utils/util";
import {ApiPromise, Keyring, WsProvider} from "@polkadot/api";
import {sudoTx, waitForEvent} from "../utils/setup";
import { addressToEvm } from '@polkadot/util-crypto';
import { Web3 } from "web3";


// Keyring needed to sign using Alice account
const keyring = new Keyring({ type: 'sr25519' });

describe('EVM token tests', function () {
  this.timeout(300 * BLOCK_TIME);
  // 4 session.
  this.slow(40 * BLOCK_TIME);

  before(async () => {
    await spawnNodes();
  });

  // Should transfer EVM token
  it('Should transfer EVM tokens', async () => {
    const {alice, aliceEthAccount, bob, bobEthAccount } = await init();
    const web3 = new Web3(
      new Web3.providers.HttpProvider("http://127.0.0.1:9933")
    );
    const BobaddressString = web3.utils.bytesToHex(bobEthAccount);
    const AliceaddressString = web3.utils.bytesToHex(aliceEthAccount);
    console.log("address 1" , BobaddressString);
    console.log("address 2" , AliceaddressString);
    
;    let bobBalance = await web3.eth.getBalance(BobaddressString);
    let AliceBalance = await web3.eth.getBalance(AliceaddressString);
    console.log("Balance:",bobBalance);
    console.log("Alice EVM Balance:", AliceBalance);
    let expectationBalance = web3.utils.toBigInt(0);
    //assert that bob initial evm balance is 0
    expect(bobBalance).to.equal(expectationBalance);




    //Tranfer between BobEthAccount and AliceEthAccount

        var originalAmountToSend = '0.01';
        var amountToSend = web3.utils.toWei(originalAmountToSend, 'ether');
        var Tx = require('ethereumjs-tx');
        var privateKey = Buffer.from('e5be9a5092b81bca64be81d212e7f2f9eba183bb7a90954f7b76361f6edb5c0a', 'hex')
        var count= web3.eth.getTransactionCount(AliceaddressString);
        var rawTx = {
        from: AliceaddressString,
        nonce: web3.utils.toHex(count),
        gasPrice: web3.utils.toHex(5000000000),
        gasLimit: web3.utils.toHex(220000),
        to: BobaddressString,
        value: web3.utils.toHex(amountToSend)
        }

        var tx = new Tx(rawTx);
        tx.sign(privateKey);
        var serializedTx = tx.serialize();
        console.log("SerializedTX",serializedTx.toString('hex'));
        // 0xf889808609184e72a00082271094000000000000000000000000000000000000000080a47f74657374320000000000000000000000000000000000000000000000000000006000571ca08a8bbf888cfa37bbf0bb965423625641fc956967b81d12e23709cead01446075a01ce999b56a8a88504be365442ea61239198e23d1fce7d00fcfc5cd3b44b7215f
        var result = await web3.eth.sendSignedTransaction('0x' + serializedTx.toString('hex'));
        console.log("Result", result);
    

  });
  // Setup the API and Alice Account
    async function init() {
        const alice = keyring.addFromUri('//Alice', { name: 'Alice default' });
        const bob = keyring.addFromUri('//Bob', { name: 'Bob default' });
    
        const aliceEthAccount = addressToEvm(alice.addressRaw);
        const bobEthAccount = addressToEvm(bob.addressRaw);
        return { alice, aliceEthAccount, bob, bobEthAccount };
    }


  after(async () => {
    await killNodes();
  });
});


   