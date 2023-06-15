import { expect } from 'chai';
import { BLOCK_TIME } from '../utils/constants';
import {killNodes, polkadotApi, spawnNodes} from "../utils/util";
import {ApiPromise, Keyring, WsProvider} from "@polkadot/api";
import {sudoTx, waitForEvent} from "../utils/setup";
import { addressToEvm } from '@polkadot/util-crypto';
import { Web3 } from "web3";


// Keyring needed to sign using Alice account
const keyring = new Keyring({ type: 'sr25519' });

describe('Native token tests', function () {
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

    //sending Transaction
    var result =await web3.eth.sendTransaction({from: AliceaddressString, to: BobaddressString, value: web3.utils.toWei("10", "ether")})
    .then(function (receipt){
            console.log("Status is:",receipt.status)
    });
    console.log(result);
    console.log("Balance:",bobBalance);
    console.log("Alice EVM Balance:", AliceBalance);

    //Sign the Transaction

    

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


   