import crypto from "crypto";

const checkIfHashStartsWithZero = (input: string) => {
  let flag = 1;
  let nonce = 0;
  let ledger;
  while (flag) {
    const hash = crypto
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

const findNonce = (input: string) => {
  let nonce = 0;
  while (true) {
    const hash = crypto
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
