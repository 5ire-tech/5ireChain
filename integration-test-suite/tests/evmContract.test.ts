import Web3 from "web3";
import {
  BLOCK_TIME,
  SECONDS,
} from "../utils/constants";
import {
  customRequest,
  killNodeForTestEVM,
  polkadotApi,
  spawnNodeForTestEVM,
} from "../utils/util";
import { sleep, waitForEvent } from "../utils/setup";

import { expect } from "chai";
import { step } from "mocha-steps";
import Keyring from "@polkadot/keyring";
import { Wallet, ethers } from "ethers";
let web3: Web3;
let aliceEthAccount: Wallet;

const ERC20_ABI = require("./contracts/MyToken.json").abi;
const ERC20_BYTECODES = require("./contracts/MyToken.json").bytecode;
const TEST_ACCOUNT = "0xdd33Af49c851553841E94066B54Fd28612522901";
let contractAddress: string;
// Keyring needed to sign using Alice account
const keyring = new Keyring({ type: "sr25519" });

async function init() {
  const alice = keyring.addFromUri("//Alice", { name: "Alice default" });
  // create eth accout from ethers
  const aliceEthAccount = ethers.Wallet.fromMnemonic(
    "bottom drive obey lake curtain smoke basket hold race lonely fit walk"
  );

  //swap native to evm balance 10 coin
  const amount = polkadotApi.createType("Balance", "10000000000000000000");
  const transaction = polkadotApi.tx.evm.deposit(
    aliceEthAccount.address,
    amount
  );

  const unsub = await transaction.signAndSend(alice, (result) => {
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

  await waitForEvent(polkadotApi, "balances", "Transfer");

  return aliceEthAccount;
}

describe("EVM related Balance", function () {
  this.timeout(100 * BLOCK_TIME);
  before(async () => {
    await spawnNodeForTestEVM();
    // Create instance web3
    web3 = new Web3(
      new Web3.providers.WebsocketProvider("ws://127.0.0.1:9944", {
        reconnect: {
          auto: true,
          delay: 5000, // ms
          maxAttempts: 5,
          onTimeout: false,
        },
      })
    );
    aliceEthAccount = await init();
    await sleep(40 * SECONDS);
  });
  after(async () => {
    await killNodeForTestEVM();
  });

  step("create the contract", async function () {
    this.timeout(40000);
    const erc20Contract = new web3.eth.Contract(ERC20_ABI);

    const deployTx = erc20Contract.deploy({
      data: ERC20_BYTECODES,
      arguments: [],
    });

    const gas = await deployTx.estimateGas({ from: aliceEthAccount.address });

    const gasPrice = await web3.eth.getGasPrice();

    const txSign = await web3.eth.accounts.signTransaction(
      {
        from: aliceEthAccount.address,
        data: deployTx.encodeABI(),
        gasPrice,
        gas,
      },
      aliceEthAccount.privateKey
    );

    let receipt = await customRequest(web3, "eth_sendRawTransaction", [
      txSign.rawTransaction,
    ]);
    await sleep(2 * SECONDS);
    const latestBlock = await web3.eth.getBlock("latest");
    expect(latestBlock.transactions.length).to.equal(1);

    const txHash = latestBlock.transactions[0];
    const tx = await web3.eth.getTransaction(txHash);

    expect(tx.hash).to.equal(txHash);
    const rep = await web3.eth.getTransactionReceipt(txHash);
    contractAddress = rep.contractAddress || "";
  });

  step("call sign transaction the method", async function () {
    const contract = new web3.eth.Contract(ERC20_ABI, contractAddress, {
      from: aliceEthAccount.address,
      gasPrice: "0x3B9ACA00",
    });
    let amountTransfer = web3.utils.toWei("1", "ether");
    const data = contract.methods
      .transfer(TEST_ACCOUNT, amountTransfer)
      .encodeABI();
    const signedTx = await web3.eth.accounts.signTransaction(
      {
        to: contractAddress,
        data,
        gas: 200000,
      },
      aliceEthAccount.privateKey
    );
    await customRequest(web3, "eth_sendRawTransaction", [
      signedTx.rawTransaction,
    ]);
    await sleep(2 * SECONDS);
    expect(await contract.methods.balanceOf(TEST_ACCOUNT).call()).to.eq(
      amountTransfer
    );
  });

  step("call query the method", async function () {
    const contract = new web3.eth.Contract(ERC20_ABI, contractAddress, {
      from: aliceEthAccount.address,
      gasPrice: "0x3B9ACA00",
    });
    let expectedTotalSupply = BigInt(2 ** 256) - BigInt(1);

    expect(await contract.methods.totalSupply().call()).to.eq(
      expectedTotalSupply.toString()
    );
  });
});
