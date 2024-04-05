import { expect } from "chai";
import { BLOCK_TIME } from "../utils/constants";
import { killNodes, polkadotApi as api, spawnNodes } from "../utils/util";
import { Keyring } from "@polkadot/api";
import { addressToEvm } from "@polkadot/util-crypto";
import { KeyringPair } from "@polkadot/keyring/types";
import { sleep, waitForEvent } from "../utils/setup";
import { bytesToHex } from "web3-utils";
import { step } from "mocha-steps";
import Web3 from "web3";

// Keyring needed to sign using Alice account
const keyring = new Keyring({ type: "sr25519" });

const ERC20_BYTECODES = require("./contracts/MyToken.json").bytecode;
let web3: Web3;

let aliceSubstrateAccount: KeyringPair;
let bobSubstrateAccount: KeyringPair;

let aliceEvmAccount: Uint8Array;
let bobEvmAccount: Uint8Array;

describe("EVM related tests", function () {
  this.timeout(300 * BLOCK_TIME);

  before(async () => {
    await spawnNodes();
    web3 = new Web3(
      new Web3.providers.HttpProvider("http://127.0.0.1:9944")
    );

    let accounts = await init();

    aliceSubstrateAccount = accounts.alice;
    bobSubstrateAccount = accounts.bob;
    aliceEvmAccount = accounts.aliceEthAccount;
    bobEvmAccount = accounts.bobEthAccount;

  });
  // Should swap native token to evm token
  step("Swap native tokens to evm tokens ", async () => {
    

    const addressString = web3.utils.bytesToHex(Array.from(aliceEvmAccount));
    let aliceBalance = await web3.eth.getBalance(addressString);
    //let expectationBalance = web3.utils.toBigInt(0);
    let expectationBalance = BigInt(0);
    //assert that bob initial evm balance is 0
    expect(BigInt(aliceBalance)).to.equal(expectationBalance);

    //Create a extrinsic, transferring 10 5ire coin to Bob
    const amount = api.createType("Balance", "10000000000000000000");
    const transaction = api.tx.evm.deposit(aliceEvmAccount, amount);

    const unsub = await transaction.signAndSend(aliceSubstrateAccount, (result) => {
      console.log(`Swap is ${result.status}`);
      if (result.status.isInBlock) {
        console.log(`Swap included at blockHash ${result.status.asInBlock}`);
        console.log(`Waiting for finalization... (can take a minute)`);
      } else if (result.status.isFinalized) {
        console.log(`events are ${result.events}`);
        console.log(`Swap finalized at blockHash ${result.status.asFinalized}`);
        unsub();
      }
    });

    await waitForEvent(api, "balances", "Transfer");
    await sleep(2000);
    let aliceBalanceAfter = await web3.eth.getBalance(addressString);
    let expectationBalanceAfter = web3.utils.toWei('10','ether');
    expect(aliceBalanceAfter).to.equal(expectationBalanceAfter);
  });

  // Should swap evm token to native token
  step("Swap evm tokens to native tokens ", async () => {

    //const addressString = web3.utils.bytesToHex(Array.from(aliceEthAccount));
    // @ts-ignore
    let {data: aliceBalanceBefore} =  await api.query.system.account(aliceSubstrateAccount.address);

    //Create a extrinsic, withdraw 5 5ire coin
    const amount = api.createType("Balance", "5000000000000000000");
    const transaction = await api.tx.evm.withdraw(aliceEvmAccount, amount);

    const unsub = await transaction.signAndSend(aliceSubstrateAccount,  {tip: 200000000, nonce: -1}, (result) => {
      console.log(`Swap is ${result.status}`);
      if (result.status.isInBlock) {
        console.log(`Swap included at blockHash ${result.status.asInBlock}`);
        console.log(`Waiting for finalization... (can take a minute)`);
      } else if (result.status.isFinalized) {
        console.log(`events are ${result.events}`);
        result.events.forEach(({ event: { data, method, section }, phase }) => {
          console.log('\t', phase.toString(), `: ${section}.${method}`, data.toString());
        });
        console.log(`Swap finalized at blockHash ${result.status.asFinalized}`);
        unsub();
      }
    });

    await waitForEvent(api, "balances", "Transfer");
    await sleep(2000);
   // @ts-ignore
    const { data: aliceBalanceAfter} = await api.query.system.account(aliceSubstrateAccount.address);
    expect(  aliceBalanceAfter.free.toBigInt() > aliceBalanceBefore.free.toBigInt()).true;

  });

  // Should create and execute contract
  step("Should create, and execute contracts", async () => {

    const contract = await createContract(aliceEvmAccount, aliceSubstrateAccount);
    await transferToken(
      bytesToHex(Array.from(aliceEvmAccount)),
      bytesToHex(Array.from(bobEvmAccount)),
      bobSubstrateAccount,
      aliceSubstrateAccount,
      contract.address
    );
  });

  after(async () => {
    await killNodes();
  });
});

// Setup the API and Alice Account
async function init() {
  const alice = keyring.addFromUri("//Alice", { name: "Alice default" });
  const bob = keyring.addFromUri("//Bob", { name: "Bob default" });

  const aliceEthAccount = addressToEvm(alice.addressRaw);
  const bobEthAccount = addressToEvm(bob.addressRaw);
  return { alice, bob, aliceEthAccount, bobEthAccount };
}

// Create the ERC20 contract from ALICE
async function createContract(evmAddress: any, alice: KeyringPair) {
  console.log(`\n: Creating Smart Contract`);

  const source = evmAddress;
  const init = ERC20_BYTECODES;
  const value = 0;
  const gasLimit = 100_000_0;
  const maxFeePerGas = 100_000_000_000;
  const maxPriorityFeePerGas: BigInt = BigInt(100_000_000);
  const nonce = 0;
  const accessList = null;

  const transaction = await api.tx.evm.create(
    source,
    init,
    value,
    gasLimit,
    maxFeePerGas,
    maxPriorityFeePerGas,
    nonce,
    accessList
  );

  const contract = new Promise<{ block: string; address: string }>(
    async (resolve, reject) => {
      const unsub = await transaction.signAndSend(
        alice,
        { tip: 200, nonce: -1 },
        (result) => {
          console.log(`Contract creation is ${result.status}`);
          if (result.status.isInBlock) {
            console.log(
              `Contract included at blockHash ${result.status.asInBlock}`
            );
            console.log(`Waiting for finalization... (can take a minute)`);
          } else if (result.status.isFinalized) {
            const data = JSON.stringify(result.events);
            console.log(data);

            const dataStr = JSON.parse(data);

            const filteredData = dataStr.filter(
              (item: any) => item.event.index === "0x3601"
            );
            const contractAddress = filteredData[0].event.data[0];
            expect(contractAddress).not.undefined;
            console.log(`Contract address: ${contractAddress}`);
            unsub();
            resolve({
              block: result.status.asFinalized.toString(),
              address: contractAddress,
            });
          }
        }
      );

      await waitForEvent(api, "evm", "Created");
    }
  );
  return contract;
}

// Transfer tokens to Bob
async function transferToken(
  aliceEthAccount: string,
  bobEthAccount: string,
  bob: KeyringPair,
  alice: KeyringPair,
  contractAddress: string
) {
  console.log(`Transfering Tokens to Bob EVM Account: ${bobEthAccount}`);

  console.log(`Preparing transfer of 0xdd`);
  const transferFnCode = `a9059cbb000000000000000000000000`;
  const tokensToTransfer = `00000000000000000000000000000000000000000000000000000000000000dd`;
  const inputCode = `0x${transferFnCode}${bobEthAccount.substring(
    2
  )}${tokensToTransfer}`;
  console.log(`Sending call input: ${inputCode}`);
  const gasLimit = 100_000_0;
  const maxFeePerGas = 100_000_000_000;
  const maxPriorityFeePerGas: BigInt = BigInt(100_000_000);
  const nonce = 1;
  const accessList = null;
  const transaction = await api.tx.evm.call(
    aliceEthAccount,
    contractAddress,
    inputCode,
    0,
    gasLimit,
    maxFeePerGas,
    maxPriorityFeePerGas,
    nonce,
    accessList
  );

  const data = new Promise<{}>(async (resolve, reject) => {
    const unsub = await transaction.signAndSend(alice, (result) => {
      console.log(`Transfer is ${result.status}`);
      if (result.status.isInBlock) {
        console.log(
          `Transfer included at blockHash ${result.status.asInBlock}`
        );
        console.log(`Waiting for finalization... (can take a minute)`);
      } else if (result.status.isFinalized) {
        console.log(
          `Transfer finalized at blockHash ${result.status.asFinalized}`
        );
        const data = JSON.stringify(result.events);
        console.log(data);
        unsub();
        resolve({});
      }
    });
  });

  await waitForEvent(api, "evm", "Executed");

  return data;
}
