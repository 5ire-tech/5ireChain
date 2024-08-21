import Web3 from "web3";
import {
  BLOCK_TIME,
  SECONDS,
  GENESIS_ACCOUNTS,
  TEST_CONTRACT_ADDRESS,
  TEST_ACCOUNT,
  ETH_BLOCK_GAS_LIMIT,
  GENESIS_ACCOUNT_0_PRIVATE_KEY,
  INVALID_OPCODE_BYTECODE,
} from "../utils/constants";
import {
  customRequest,
  killNodeForTestEVM,
  spawnNodeForTestEVM,
} from "../utils/util";
import { sleep } from "../utils/setup";

import { expect } from "chai";
let web3: Web3;

const TRANSFER_VALUE = "1";
const ERC20_ABI = require("./contracts/MyToken.json").abi;
const ERC20_BYTECODES = require("./contracts/MyToken.json").bytecode;

describe("EVM related Gas using web3js/ethersjs", function () {
  this.timeout(100 * BLOCK_TIME);
  let contractAddess;
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
      }),
    );
    await sleep(3 * SECONDS);
  });
  after(async () => {
    await killNodeForTestEVM();
  });

  it("estimate gas for contract creation", async function () {
    this.timeout(40000);
    let gasEstimation = await web3.eth.estimateGas({
      from: GENESIS_ACCOUNTS[0],
      data: ERC20_BYTECODES,
    });
    expect(gasEstimation).to.eq(935922);
  });

  it("estimate gas for contract call", async function () {
    const erc20Contract = new web3.eth.Contract(
      ERC20_ABI,
      TEST_CONTRACT_ADDRESS,
    );

    let amount = web3.utils.toWei(TRANSFER_VALUE, "ether");
    let gasEstimation = await erc20Contract.methods
      .transfer(TEST_ACCOUNT, amount)
      .estimateGas({ from: GENESIS_ACCOUNTS[0] });
    expect(gasEstimation).to.eq(22371);
  });

  it("estimate gas with gasPrice value is 0x0 ", async function () {
    let result = await web3.eth.estimateGas({
      from: GENESIS_ACCOUNTS[0],
      data: ERC20_BYTECODES,
      gasPrice: "0x0",
    });
    expect(result).to.equal(935922);
    result = await web3.eth.estimateGas({
      from: GENESIS_ACCOUNTS[0],
      data: ERC20_BYTECODES,
    });
    expect(result).to.equal(935922);
  });

  it("tx gas limit below ETH_BLOCK_GAS_LIMIT", async function () {
    const erc20Contract = new web3.eth.Contract(ERC20_ABI);

    const deployTx = erc20Contract.deploy({
      data: ERC20_BYTECODES,
      arguments: [],
    });

    const gasPrice = await web3.eth.getGasPrice();

    const tx = await web3.eth.accounts.signTransaction(
      {
        from: GENESIS_ACCOUNTS[0],
        data: deployTx.encodeABI(),
        gas: ETH_BLOCK_GAS_LIMIT - 1,
        gasPrice,
      },
      GENESIS_ACCOUNT_0_PRIVATE_KEY,
    );
    const createReceipt = await customRequest(web3, "eth_sendRawTransaction", [
      tx.rawTransaction,
    ]);
    expect((createReceipt as any).transactionHash).to.be.not.null;
    expect((createReceipt as any).blockHash).to.be.not.null;
  });

  it("tx gas limit equal ETH_BLOCK_GAS_LIMIT", async function () {
    const erc20Contract = new web3.eth.Contract(ERC20_ABI);

    const deployTx = erc20Contract.deploy({
      data: ERC20_BYTECODES,
      arguments: [],
    });

    const gasPrice = await web3.eth.getGasPrice();

    const tx = await web3.eth.accounts.signTransaction(
      {
        from: GENESIS_ACCOUNTS[0],
        data: deployTx.encodeABI(),
        gas: ETH_BLOCK_GAS_LIMIT,
        gasPrice,
      },
      GENESIS_ACCOUNT_0_PRIVATE_KEY,
    );
    const createReceipt = await customRequest(web3, "eth_sendRawTransaction", [
      tx.rawTransaction,
    ]);
    expect((createReceipt as any).transactionHash).to.be.not.null;
    expect((createReceipt as any).blockHash).to.be.not.null;
  });

  it("tx gas limit larger ETH_BLOCK_GAS_LIMIT", async function () {
    const erc20Contract = new web3.eth.Contract(ERC20_ABI);

    const deployTx = erc20Contract.deploy({
      data: ERC20_BYTECODES,
      arguments: [],
    });

    const gasPrice = await web3.eth.getGasPrice();

    const tx = await web3.eth.accounts.signTransaction(
      {
        from: GENESIS_ACCOUNTS[0],
        data: deployTx.encodeABI(),
        gas: ETH_BLOCK_GAS_LIMIT + 1,
        gasPrice,
      },
      GENESIS_ACCOUNT_0_PRIVATE_KEY,
    );
    const createReceipt = await customRequest(web3, "eth_sendRawTransaction", [
      tx.rawTransaction,
    ]);
    expect((createReceipt as any).error.message).to.equal(
      "exceeds block gas limit",
    );
  });

  it("EVM related Invalid opcode Estimate Gas using web3js/ethersjs", async function () {
    const tx = await web3.eth.accounts.signTransaction(
      {
        from: GENESIS_ACCOUNTS[0],
        data: INVALID_OPCODE_BYTECODE,
        value: "0x00",
        gas: "0x100000",
      },
      GENESIS_ACCOUNT_0_PRIVATE_KEY,
    );
    const txHash = await customRequest(web3, "eth_sendRawTransaction", [
      tx.rawTransaction,
    ]);
    await sleep(3000);
    contractAddess = (await web3.eth.getTransactionReceipt(txHash.result))
      .contractAddress;
    expect(contractAddess).to.not.null;

    let estimate = await web3.eth.estimateGas({
      from: GENESIS_ACCOUNTS[0],
      to: contractAddess,
      data: "0x28b5e32b", // selector for the contract's `call` method
    });
    expect(estimate).to.equal(85699);
  });
});
