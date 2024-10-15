import Web3 from "web3";
import {
    ALITH_PRIVATE_KEY,
    BALTATHAR_ADDRESS,
    BALTATHAR_PRIVATE_KEY,
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

import { readFileSync } from 'fs';
import { join } from 'path';

let web3: Web3;

const BATCH_CONTRACT = "0x0000000000000000000000000000000000001000";

const BATCH_ABI = require("./contracts/batch/Batch.json");

const ERC20_ABI = require("./contracts/MyToken.json");

const ERC20_BYTECODES = readFileSync(join(__dirname, './contracts/erc20_contract_bytecode.txt'), 'utf8').trim();


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

    step("batchAll: approve and transferFrom ERC20 tokens", async function () {
        this.timeout(60000);

        const approveAmount = web3.utils.toWei("50", "ether");
        const transferAmount = web3.utils.toWei("50", "ether");

        const approveTx = erc20Contract.methods.approve(BALTATHAR_ADDRESS, approveAmount).encodeABI();
        const transferFromTx = erc20Contract.methods.transferFrom(alith.address, CHARLETH_ADDRESS, transferAmount).encodeABI();

        const signedTx = await web3.eth.accounts.signTransaction(
            {
                to: erc20Address,
                data: approveTx,
                gas: 1000000,
            },
            ALITH_PRIVATE_KEY,
        );
        await customRequest(web3, "eth_sendRawTransaction", [
            signedTx.rawTransaction,
        ]);
        await sleep(4 * SECONDS);

        const baltatharAllowance = await erc20Contract.methods.allowance(alith.address, BALTATHAR_ADDRESS).call();
        expect(baltatharAllowance).to.equal(approveAmount);

        const batchTx = batchContract.methods.batchAll(
            [erc20Address],
            [0],
            [transferFromTx],
            [300000]
        );


        const txSign = await web3.eth.accounts.signTransaction(
            {
                from: alith.address,
                to: BATCH_CONTRACT,
                data: batchTx.encodeABI(),
                gas: 1000000,
            },
            BALTATHAR_PRIVATE_KEY,
        );

        let receipt = await customRequest(web3, "eth_sendRawTransaction", [
            txSign.rawTransaction,
        ]);

        await sleep(3 * SECONDS);

        //Check balances

        const charlethBalance = await erc20Contract.methods.balanceOf(CHARLETH_ADDRESS).call();
        expect(charlethBalance).to.equal(transferAmount);
    });

    step("batchAll: fail due to OutOfFund", async function () {
        this.timeout(30000);

        const baltatharInitialBalance = await web3.eth.getBalance(BALTATHAR_ADDRESS);
        const transferAmount = web3.utils.toWei((parseFloat(web3.utils.fromWei(baltatharInitialBalance, 'ether')) + 1).toString(), 'ether');

        const batchTx = batchContract.methods.batchAll(
            [CHARLETH_ADDRESS, CHARLETH_ADDRESS],
            [transferAmount, transferAmount],
            ["0x", "0x"],
            [21000, 21000]
        );


        try {
            const txSign = await web3.eth.accounts.signTransaction(
                {
                    from: BALTATHAR_ADDRESS,
                    to: BATCH_CONTRACT,
                    data: batchTx.encodeABI(),
                    gas: 300000,
                    value: "0x",
                },
                BALTATHAR_PRIVATE_KEY,
            );

            await customRequest(web3, "eth_sendRawTransaction", [txSign.rawTransaction]);
            await sleep(3 * SECONDS);
            throw new Error("Transaction should have failed due to insufficient funds");
        } catch (error: unknown) {
            if (error instanceof Error) {
                expect(error.message).to.include("insufficient funds");
                console.log("Transaction failed as expected due to insufficient funds");
            } else {
                throw error; 
            }
        }

        const baltatharFinalBalance = await web3.eth.getBalance(BALTATHAR_ADDRESS);
        expect(BigInt(baltatharFinalBalance)< BigInt(baltatharInitialBalance)).to.be.true;
    });



});


