import express from "express";
import { ethers } from "ethers";

// Setup prover

let sampleContract : null | any  = null;
const provider = new ethers.providers.JsonRpcProvider("http://127.0.0.1:8545", 0x314);
const wallet = new ethers.Wallet("b9012ef7fc6f65d381f235dea00a1ecb587d5c2ba65392dce4cbb93487f39866", provider);
const withdrawWallet = new ethers.Wallet("bf9099bdcf0d818def83168a16d6e21ada1c143708c0b6be12bd7fbb17b785cd", provider);

console.log("Setting up web server");
let app = express();

app.set("view engine", "ejs");

app.get("/", async (req, res) => {
    res.render("index");
});

app.get("/create_account", async (req, res) => {
    const nullifier = ethers.utils.hexZeroPad(`0x${req.query.nullifier}`, 32).slice(2);
    const secret = ethers.utils.hexZeroPad(`0x${req.query.secret}`, 32).slice(2);
    const value = String(`0x${req.query.value}`);
    const address = ethers.utils.keccak256('0x' + nullifier + secret)
    
    const tx = await wallet.sendTransaction({
        to: address.slice(0, 42),
        value: value
      });
    console.log("Nullifier:", nullifier, "Secret:", secret, "Value:", value);
    console.log("Sending transaction to unspendable address:", address);
    res.render("create_address", { "address": address.slice(0, 42)});
});

app.get("/withdraw", async (req, res) => {
    const nullifier = ethers.utils.hexZeroPad(`0x${req.query.nullifier}`, 32).slice(2);
    const value = String(`0x${req.query.value}`);
    const stateRoot = ethers.utils.hexZeroPad(`0x${req.query.state_root}`, 32).slice(2);
    const proof = ethers.utils.hexZeroPad(`0x${req.query.proof}`, 32).slice(2);
    
    const tx = await withdrawWallet.sendTransaction({
        to: "0x4200000000000000000000000000000000000000",
        value: value,
        data: '0x' + nullifier + stateRoot + proof,
      });
    console.log("Withdrawing. Value:", value, "Data:", '0x' + nullifier + stateRoot + proof);
    res.render("create_address", { "address": "0x241D76926cdb4cBE1b6C2d8c3C8C61ba16E0149E"});
});

app.listen(8080);
console.log("Done");
