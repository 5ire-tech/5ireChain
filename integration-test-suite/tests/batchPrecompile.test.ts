import Web3 from "web3";
import {
    ALITH_PRIVATE_KEY,
    BALTATHAR_ADDRESS,
    BLOCK_TIME,
    CHARLETH_ADDRESS,
    SECONDS,
    alith,
} from "../utils/constants";
import {
    customRequest,
    killNodeForTestEVM,
    spawnNodeForTestEVM,
} from "../utils/util";
import { sleep } from "../utils/setup";

import { expect } from "chai";
import { step } from "mocha-steps";
let web3: Web3;

const BATCH_CONTRACT = "0x0000000000000000000000000000000000001000";

const BATCH_ABI = require("./contracts/batch/Batch.json");

const ERC20_ABI = require("./contracts/MyToken.json").abi;
const ERC20_BYTECODES = require("./contracts/MyToken.json").bytecode;


describe("EVM related Contract using web3js/ethersjs", function () {
    this.timeout(100 * BLOCK_TIME);

    let batchContract: any;
    let erc20Contract: any;
    let erc20Address: string;


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
        await sleep(5 * SECONDS);

        const MyToken = new web3.eth.Contract(ERC20_ABI);
        const deployTx = MyToken.deploy({
            data: ERC20_BYTECODES,
            arguments: []
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
            ALITH_PRIVATE_KEY,
        );

        let receipt = await customRequest(web3, "eth_sendRawTransaction", [
            txSign.rawTransaction,
        ]);
        await sleep(3 * SECONDS);

        const txReceipt = await web3.eth.getTransactionReceipt(receipt.result);
        erc20Address = txReceipt.contractAddress as string;

        erc20Contract = new web3.eth.Contract(ERC20_ABI, erc20Address);


        // load Batch contract
        batchContract = new web3.eth.Contract(BATCH_ABI, BATCH_CONTRACT, {
            from: alith.address,
        });


    });
    after(async () => {
        await killNodeForTestEVM();
    });

    step("call batchAll Precompile with transfer native tokens", async function () {
        this.timeout(40000);

        const transferAmount = web3.utils.toWei("1", "ether");

        const baltatharBalanceBefore = await web3.eth.getBalance(BALTATHAR_ADDRESS);
        const charlethBalanceBefore = await web3.eth.getBalance(CHARLETH_ADDRESS);


        const batchTx = batchContract.methods.batchAll(
            [BALTATHAR_ADDRESS, CHARLETH_ADDRESS],
            [transferAmount, transferAmount],
            ["0x", "0x"],
            [21000, 21000]
        );

        const gas = await batchTx.estimateGas({ from: alith.address });
        const gasPrice = await web3.eth.getGasPrice();

        const txSign = await web3.eth.accounts.signTransaction(
            {
                from: alith.address,
                to: BATCH_CONTRACT,
                data: batchTx.encodeABI(),
                gasPrice,
                gas,
                value: "0x",
            },
            ALITH_PRIVATE_KEY,
        );

        let receipt = await customRequest(web3, "eth_sendRawTransaction", [
            txSign.rawTransaction,
        ]);

        await sleep(3 * SECONDS);
        const baltatharBalanceAfter = await web3.eth.getBalance(BALTATHAR_ADDRESS);
        const charlethBalanceAfter = await web3.eth.getBalance(CHARLETH_ADDRESS);

        expect(BigInt(baltatharBalanceAfter) - BigInt(baltatharBalanceBefore)).to.equal(BigInt(transferAmount));
        expect(BigInt(charlethBalanceAfter) - BigInt(charlethBalanceBefore)).to.equal(BigInt(transferAmount));

    });




});


