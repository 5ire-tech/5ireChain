import Web3 from "web3";
import {
  BLOCK_TIME,
  GENESIS_ACCOUNTS,
  GENESIS_ACCOUNT_0_PRIVATE_KEY,
  SECONDS,
  TEST_ACCOUNT,
} from "../utils/constants";
import {
  customRequest,
  killNodeForTestEVM,
  spawnNodeForTestEVM,
} from "../utils/util";
import { sleep } from "../utils/setup";

import { expect } from "chai";
import { readFileSync } from "fs";
import { join } from "path";

let web3: Web3;

const ERC20_ABI = require("./contracts/MyToken.json");

const ERC20_BYTECODES = readFileSync(join(__dirname, './contracts/erc20_contract_bytecode.txt'), 'utf8').trim();

let contractAddress: string;

describe("EVM related Pool using web3js/ethersjs", function () {
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
      }),
    );

    const erc20Contract = new web3.eth.Contract(ERC20_ABI);

    const deployTx = erc20Contract.deploy({
      data: ERC20_BYTECODES,
      arguments: [],
    });

    const gas = await deployTx.estimateGas({ from: GENESIS_ACCOUNTS[0] });

    const gasPrice = await web3.eth.getGasPrice();

    const txSign = await web3.eth.accounts.signTransaction(
      {
        from: GENESIS_ACCOUNTS[0],
        data: deployTx.encodeABI(),
        gasPrice,
        gas,
      },
      GENESIS_ACCOUNT_0_PRIVATE_KEY,
    );
    const receipt = await web3.eth.sendSignedTransaction(
      txSign.rawTransaction as string,
    );
    await sleep(1 * SECONDS);
    contractAddress = receipt.contractAddress || "";

    await sleep(20 * SECONDS);
  });
  after(async () => {
    await killNodeForTestEVM();
  });

  it("Transaction Cost discards due to gas too low", async function () {
    const contract = new web3.eth.Contract(ERC20_ABI, contractAddress, {
      from: GENESIS_ACCOUNTS[0],
      gasPrice: "0x3B9ACA00",
    });
    let amountTransfer = web3.utils.toWei("1", "ether");
    const data = contract.methods
      .transfer(TEST_ACCOUNT, amountTransfer)
      .encodeABI();

    // Issue: Intrinsic gas too low
    expect(
      web3.eth.accounts.signTransaction(
        {
          to: contractAddress,
          data,
          // we intentionally set gas too low
          gas: 2000,
        },
        GENESIS_ACCOUNT_0_PRIVATE_KEY,
      ),
    ).to.throw;
  });

  it("EVM RPC pool error - already known", async function () {
    const nonce = await web3.eth.getTransactionCount(GENESIS_ACCOUNTS[0]);
    let gasPrice = await web3.eth.getGasPrice();
    let tx = await createRawTransferLegacy(
      GENESIS_ACCOUNTS[0],
      TEST_ACCOUNT,
      "1",
      21000,
      web3.utils.toHex(gasPrice),
      nonce,
      GENESIS_ACCOUNT_0_PRIVATE_KEY,
    );
    await web3.eth.sendSignedTransaction(tx.rawTransaction as string);
    let result = await customRequest(web3, "eth_sendRawTransaction", [
      tx.rawTransaction,
    ]);
    expect(result?.error?.message).to.be.equal("already known");
  });

  it("EVM RPC pool error - exceeds block gas limit", async function () {
    const nonce = await web3.eth.getTransactionCount(
      GENESIS_ACCOUNTS[0],
      "latest",
    );
    let gasPriceTx = web3.utils.toWei("15", "gwei");
    let tx = await createRawTransferLegacy(
      GENESIS_ACCOUNTS[0],
      TEST_ACCOUNT,
      "1",
      10_000_000_000,
      web3.utils.toHex(gasPriceTx),
      nonce,
      GENESIS_ACCOUNT_0_PRIVATE_KEY,
    );

    let result = await customRequest(web3, "eth_sendRawTransaction", [
      tx.rawTransaction,
    ]);
    expect(result?.error?.message).to.be.equal("exceeds block gas limit");
  });

  it("EVM RPC pool error - max priority fee per gas higher than max fee per gas", async function () {
    const nonce = await web3.eth.getTransactionCount(
      GENESIS_ACCOUNTS[0],
      "latest",
    );

    // Throw: maxFeePerGas cannot be less than maxPriorityFeePerGas
    expect(
      createRawTransferEIP1559(
        GENESIS_ACCOUNTS[0],
        TEST_ACCOUNT,
        "1",
        21000,
        100_000_000_000,
        200_000_000_000,
        nonce,
        GENESIS_ACCOUNT_0_PRIVATE_KEY,
      ),
    ).Throw;
  });
});

async function createRawTransferLegacy(
  from: string,
  to: string,
  amount: string,
  gas: number,
  gasPrice: string,
  nonce: number,
  privateKey: string,
): Promise<any> {
  return new Promise((resolve, reject) => {
    const transaction = {
      from: from,
      to: to,
      value: web3.utils.toWei(amount, "ether"),
      gas: gas,
      gasPrice: gasPrice,
      nonce: nonce,
    };

    web3.eth.accounts
      .signTransaction(transaction, privateKey)
      .then((signedTx) => resolve(signedTx))
      .catch((error) => reject(new Error(error.message)));
  });
}

async function createRawTransferEIP1559(
  from: string,
  to: string,
  amount: string,
  gas: number,
  maxFeePerGas: number,
  maxPriorityFeePerGas: number,
  nonce: number,
  privateKey: string,
): Promise<any> {
  return new Promise((resolve, reject) => {
    const transaction = {
      from: from,
      to: to,
      value: web3.utils.toWei(amount, "ether"),
      gas: gas,
      maxFeePerGas: maxFeePerGas,
      maxPriorityFeePerGas: maxPriorityFeePerGas,
      nonce: nonce,
    };

    web3.eth.accounts
      .signTransaction(transaction, privateKey)
      .then((signedTx) => resolve(signedTx))
      .catch((error) => reject(new Error(error.message)));
  });
}
