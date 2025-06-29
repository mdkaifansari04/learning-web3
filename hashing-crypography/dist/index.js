"use strict";
var __importDefault = (this && this.__importDefault) || function (mod) {
    return (mod && mod.__esModule) ? mod : { "default": mod };
};
Object.defineProperty(exports, "__esModule", { value: true });
const crypto_1 = __importDefault(require("crypto"));
const checkIfHashStartsWithZero = (input) => {
    let flag = 1;
    let nonce = 0;
    let ledger;
    while (flag) {
        const hash = crypto_1.default
            .createHash("sha256")
            .update(input + nonce)
            .digest("hex");
        nonce++;
        if (hash.startsWith("0000")) {
            flag = 0;
            ledger = input + nonce;
            break;
        }
    }
    console.log("Ledger :", ledger);
};
const findNonce = (input) => {
    let nonce = 0;
    while (true) {
        const hash = crypto_1.default
            .createHash("sha256")
            .update(input + nonce)
            .digest("hex");
        nonce++;
        if (hash.startsWith("000000000000")) {
            console.log("Nonce : ", nonce);
            break;
        }
    }
};
findNonce("harkirat => Raman | Rs 100 Ram => Ankit | Rs 10");
// checkIfHashStartsWithZero("kaif");
