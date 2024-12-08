# Vesta

<strong>Vesta</strong> is a blockchain-based token vesting solution designed to securely lock tokens and release them gradually over a predefined schedule. It supports multiple beneficiaries, making it an ideal tool for managing team allocations, investor vesting, or other scenarios requiring controlled token distribution.

## Key Features

1. <strong>Token Locking</strong>
   Lock tokens for specific beneficiaries with a clear start and end time.

2. <strong>Gradual Release</strong>
   Tokens are released incrementally based on the elapsed time during the vesting period.

3. <strong>Multiple Beneficiaries</strong>
   Manage vesting schedules for multiple users simultaneously.

4. <strong>Secure Withdrawals</strong>
   Tokens can only be withdrawn after the vesting schedule is met.

5. <strong>On-Chain Transparency</strong>
   All schedules, releases, and withdrawals are recorded on the blockchain, ensuring complete transparency.

## How Vesta Works

1. <strong>Token Deposit</strong>

- The contract manager deposits tokens for a specific beneficiary, defining a total amount, start time, and end time for the vesting period.

2. <strong>Gradual Vesting</strong>

- As time progresses, a portion of the locked tokens becomes available for withdrawal based on the elapsed vesting period.

3. <strong>Token Withdrawal</strong>

- Beneficiaries can withdraw their vested tokens at any time after the vesting period starts, but only up to the vested amount.

4. <strong>Final Release</strong>

- Once the vesting period ends, all remaining locked tokens become available for withdrawal.

## Technologies Used

1. <strong>Rust (Near Protocol):</strong>
   The core smart contract logic is implemented in Rust for the Near blockchain, leveraging its scalability and efficiency.

2. <strong>HTML/CSS:</strong>
   Provides a user-friendly interface for interacting with the vesting contract.

3. <strong>TypeScript:</strong>
   Powers the frontend logic to enable seamless interaction with the smart contract.

4. <strong> Near Protocol:</strong>
   A scalable blockchain platform used for deploying the Vesta contract.

## Setup Instructions

### Prerequisites

<ul>
<li><a href="https://https://www.rust-lang.org/"></a>Rust</li>
<li><a href="https://docs.near.org/tools/near-cli/">NEAR CLI</a></li>
<li>Node.js and NPM</li>
<li>NEAR Testnet Account</li>
</ul>
