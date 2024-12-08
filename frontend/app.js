const nearAPI = require("near-api-js");

const { connect, WalletConnection, Contract, utils } = nearAPI;

async function initContract() {
  const near = await connect({
    networkId: "testnet",
    keyStore: new nearAPI.keyStores.BrowserLocalStorageKeyStore(),
    nodeUrl: "https://rpc.testnet.near.org",
    walletUrl: "https://wallet.testnet.near.org",
    helperUrl: "https://helper.testnet.near.org",
  });

  const wallet = new WalletConnection(near);
  const contract = new Contract(
    wallet.account(),
    "your-contract-account.testnet",
    {
      viewMethods: [],
      changeMethods: ["deposit", "withdraw"],
    }
  );

  return { wallet, contract };
}

document
  .getElementById("vesting-form")
  .addEventListener("submit", async (e) => {
    e.preventDefault();

    const { contract } = await initContract();
    const beneficiary = document.getElementById("beneficiary").value;
    const amount = utils.format.parseNearAmount(
      document.getElementById("amount").value
    );
    const startTime = new Date(
      document.getElementById("start-time").value
    ).getTime();
    const endTime = new Date(
      document.getElementById("end-time").value
    ).getTime();

    await contract.deposit({
      beneficiary,
      total_amount: amount,
      start_time: startTime,
      end_time: endTime,
    });
    alert("Tokens vested successfully!");
  });

document
  .getElementById("withdraw-form")
  .addEventListener("submit", async (e) => {
    e.preventDefault();

    const { contract } = await initContract();
    const beneficiary = document.getElementById("withdraw-beneficiary").value;

    await contract.withdraw({ beneficiary });
    alert("Tokens withdrawn successfully!");
  });
