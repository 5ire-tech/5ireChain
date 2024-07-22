import Web3 from "web3";
import { BLOCK_TIME, SECONDS } from "../utils/constants";
import {
  customRequest,
  killNodeForTestEVM,
  spawnNodeForTestEVM,
} from "../utils/util";
import { sleep } from "../utils/setup";

import { expect } from "chai";
let web3: Web3;


describe("EVM related Fee using web3js/ethersjs", function () {
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

  it("get fee genesis for evm chain", async function () {
    this.timeout(20000);

    const gasPrice = await web3.eth.getGasPrice();
    // we configure in runtime 
    expect(BigInt(gasPrice)).to.eq(250000000000n);
  });

  it("Fee History should return error on non-existent blocks", async function () {
		this.timeout(100000);
		let result = customRequest(web3, "eth_feeHistory", ["0x0", "0x7", []])
			.then(() => {
				return Promise.reject({
					message: "Execution succeeded but should have failed",
				});
			})
			.catch((err) => expect(err.message).to.equal("Error getting header at BlockId::Number(1)"));
	});
});