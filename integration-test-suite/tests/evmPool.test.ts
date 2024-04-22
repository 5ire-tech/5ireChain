import Web3 from "web3";
import {
  BLOCK_TIME,
  GENESIS_ACCOUNTS,
  GENESIS_ACCOUNT_0_PRIVATE_KEY,
  SECONDS,
} from "../utils/constants";
import {
  customRequest,
  killNodeForTestEVM,
  polkadotApi,
  spawnNodeForTestEVM,
} from "../utils/util";
import { sleep, waitForEvent } from "../utils/setup";

import { expect, use } from "chai";
import { step } from "mocha-steps";
import { Wallet, ethers } from "ethers";
import chaiAsPromised from "chai-as-promised";

let web3: Web3;

const TEST_ACCOUNT = "0xdd33Af49c851553841E94066B54Fd28612522901";
const TEST_ACCOUNT_2 = GENESIS_ACCOUNTS[1];
const TEST_ACCOUNT_PRIVATE_KEY =
  "0x4ca933bffe83185dda76e7913fc96e5c97cdb7ca1fbfcc085d6376e6f564ef71";
const TRANFER_VALUE = "1"; // 1 5IRE must be higher than ExistentialDeposit
//const GAS_PRICE = "0x3B9ACA00"; // 1000000000

const ERC20_ABI = require("./contracts/MyToken.json").abi;
const ERC20_BYTECODES = require("./contracts/MyToken.json").bytecode;
let contractAddress: string;

describe("EVM related Pool using web3js/ethersjs", function () {
  this.timeout(100 * BLOCK_TIME);
  before(async () => {
    use(chaiAsPromised);
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
      GENESIS_ACCOUNT_0_PRIVATE_KEY
    );
    const receipt = await web3.eth.sendSignedTransaction(
      txSign.rawTransaction as string
    );
    await sleep(1 * SECONDS);
    contractAddress = receipt.contractAddress || "";

    await sleep(40 * SECONDS);
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
        GENESIS_ACCOUNT_0_PRIVATE_KEY
      )
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
      GENESIS_ACCOUNT_0_PRIVATE_KEY
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
      "latest"
    );
    let gasPriceTx = web3.utils.toWei("15", "gwei");
    let tx = await createRawTransferLegacy(
      GENESIS_ACCOUNTS[0],
      TEST_ACCOUNT,
      "1",
      10_000_000_000,
      web3.utils.toHex(gasPriceTx),
      nonce,
      GENESIS_ACCOUNT_0_PRIVATE_KEY
    );

    let result = await customRequest(web3, "eth_sendRawTransaction", [
      tx.rawTransaction,
    ]);
    expect(result?.error?.message).to.be.equal("exceeds block gas limit");
  });

  it("EVM RPC pool error - max priority fee per gas higher than max fee per gas", async function () {
    const nonce = await web3.eth.getTransactionCount(
      GENESIS_ACCOUNTS[0],
      "latest"
    );

    expect(
      createRawTransferEIP1559(
        GENESIS_ACCOUNTS[0],
        TEST_ACCOUNT,
        "1",
        21000,
        100_000_000_000,
        200_000_000_000,
        nonce,
        GENESIS_ACCOUNT_0_PRIVATE_KEY
      )
    ).rejectedWith(Error, "maxFeePerGas cannot be less than maxPriorityFeePerGas");
  });
});

async function createRawTransferLegacy(
  from: string,
  to: string,
  amount: string,
  gas: number,
  gasPrice: string,
  nonce: number,
  privateKey: string
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
  privateKey: string
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
