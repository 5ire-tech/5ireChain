import Web3 from "web3";
import {
  ALITH_PRIVATE_KEY,
  BLOCK_TIME,
  SECONDS,
  alith,
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
let web3: Web3;

const ERC20_ABI = require("./contracts/MyToken.json").abi;
const ERC20_BYTECODES = require("./contracts/MyToken.json").bytecode;
const TEST_ACCOUNT = "0xdd33Af49c851553841E94066B54Fd28612522901";
let contractAddress: string;

describe("EVM related Contract using web3js/ethersjs", function () {
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
    await sleep(20 * SECONDS);
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

    const gas = await deployTx.estimateGas({ from: alith.address });

    const gasPrice = await web3.eth.getGasPrice();

    const txSign = await web3.eth.accounts.signTransaction(
      {
        from: alith.address,
        data: deployTx.encodeABI(),
        gasPrice,
        gas,
      },
      ALITH_PRIVATE_KEY
    );

    let receipt = await customRequest(web3, "eth_sendRawTransaction", [
      txSign.rawTransaction,
    ]);
    await sleep(3 * SECONDS);
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
      from: alith.address
    });
    let amountTransfer = web3.utils.toWei("1", "ether");
    const data = contract.methods
      .transfer(TEST_ACCOUNT, amountTransfer)
      .encodeABI();

    const signedTx = await web3.eth.accounts.signTransaction(
      {
        to: contractAddress,
        data,
        gas:1000000,
  
      },
      ALITH_PRIVATE_KEY
    );
    await customRequest(web3, "eth_sendRawTransaction", [
      signedTx.rawTransaction,
    ]);
    await sleep(4 * SECONDS);

    expect(await contract.methods.balanceOf(TEST_ACCOUNT).call()).to.eq(
      amountTransfer
    );

  });

  step("call query the method", async function () {
    const contract = new web3.eth.Contract(ERC20_ABI, contractAddress, {
      from: alith.address
    });
    let expectedTotalSupply = BigInt(2 ** 256) - BigInt(1);

    expect(await contract.methods.totalSupply().call()).to.eq(
      expectedTotalSupply.toString()
    );
  });
});
