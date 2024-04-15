import Web3 from "web3";
import { BLOCK_TIME, SECONDS, GENESIS_ACCOUNTS, TEST_CONTRACT_ADDRESS, TEST_ACCOUNT } from "../utils/constants";
import {
  customRequest,
  killNodeForTestEVM,
  polkadotApi,
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
    await sleep(40 * SECONDS);
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
    expect(gasEstimation).to.eq(894198);
  });

  it("estimate gas for contract call", async function () {
    const erc20Contract = new web3.eth.Contract(
      ERC20_ABI, TEST_CONTRACT_ADDRESS
    );

    let amount = web3.utils.toWei(TRANSFER_VALUE, "ether");
    let gasEstimation = await erc20Contract.methods
      .transfer(TEST_ACCOUNT, amount)
      .estimateGas({ from: GENESIS_ACCOUNTS[0] });
    expect(gasEstimation).to.eq(21632);
  });

  it("estimate gas with gasPrice value is 0x0 ", async function () {
    let result = await web3.eth.estimateGas({
        from: GENESIS_ACCOUNTS[0],
        data: ERC20_BYTECODES,
        gasPrice: "0x0",
    });
    expect(result).to.equal(894198);
    result = await web3.eth.estimateGas({
        from: GENESIS_ACCOUNTS[0],
        data: ERC20_BYTECODES,
    });
    expect(result).to.equal(894198);
});

});
