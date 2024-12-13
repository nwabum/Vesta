use near_sdk::collections::UnorderedMap;
use near_sdk::{env, near_bindgen, AccountId, Balance, Promise};
use std::collections::HashMap;

#[near_bindgen]
#[derive(Default)]
pub struct TokenVesting {
    // Stores the vested tokens for each beneficiary
    beneficiaries: UnorderedMap<AccountId, VestingSchedule>,
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct VestingSchedule {
    total_amount: Balance,
    released_amount: Balance,
    start_time: u64,
    end_time: u64,
}

#[near_bindgen]
impl TokenVesting {
    #[init]
    pub fn new() -> Self {
        Self {
            beneficiaries: UnorderedMap::new(b"b"),
        }
    }

    // Deposit tokens for a beneficiary with a vesting schedule
    pub fn deposit(
        &mut self,
        beneficiary: AccountId,
        total_amount: Balance,
        start_time: u64,
        end_time: u64,
    ) {
        assert!(
            end_time > start_time,
            "End time must be greater than start time"
        );
        assert!(
            total_amount > 0,
            "Total amount must be greater than zero"
        );

        let schedule = VestingSchedule {
            total_amount,
            released_amount: 0,
            start_time,
            end_time,
        };

        self.beneficiaries.insert(&beneficiary, &schedule);
        env::log_str(&format!("Tokens vested for {}", beneficiary));
    }

    // Withdraw vested tokens after the vesting period
    pub fn withdraw(&mut self, beneficiary: AccountId) {
        let mut schedule = self
            .beneficiaries
            .get(&beneficiary)
            .expect("No vesting schedule found for the beneficiary");

        let current_time = env::block_timestamp();
        assert!(
            current_time >= schedule.start_time,
            "Vesting period has not started yet"
        );

        let vested_amount = self.calculate_vested_amount(&schedule, current_time);
        assert!(
            vested_amount > schedule.released_amount,
            "No tokens available for withdrawal"
        );

        let releasable_amount = vested_amount - schedule.released_amount;
        schedule.released_amount = vested_amount;

        self.beneficiaries.insert(&beneficiary, &schedule);

        Promise::new(beneficiary).transfer(releasable_amount);
        env::log_str(&format!(
            "Released {} tokens to {}",
            releasable_amount, schedule.total_amount
        ));
    }

    // Calculate vested tokens based on the current time
    fn calculate_vested_amount(&self, schedule: &VestingSchedule, current_time: u64) -> Balance {
        if current_time >= schedule.end_time {
            return schedule.total_amount;
        } else if current_time <= schedule.start_time {
            return 0;
        } else {
            let elapsed_time = current_time - schedule.start_time;
            let total_time = schedule.end_time - schedule.start_time;
            (schedule.total_amount * elapsed_time as u128) / total_time as u128
        }
    }
}

#[near_bindgen]
impl TokenVesting {
    // Get current vesting schedule for a beneficiary
    pub fn get_vesting_schedule(&self, beneficiary: AccountId) -> Option<VestingSchedule> {
        self.beneficiaries.get(&beneficiary)
    }

    // Get remaining withdrawable amount for a beneficiary
    pub fn get_remaining_amount(&self, beneficiary: AccountId) -> Balance {
        let schedule = self
            .beneficiaries
            .get(&beneficiary)
            .expect("No vesting schedule found for the beneficiary");

        let current_time = env::block_timestamp();
        let vested_amount = self.calculate_vested_amount(&schedule, current_time);
        vested_amount - schedule.released_amount
    }
}


pub fn partial_withdraw(&mut self, beneficiary: AccountId, amount: Balance) {
    let mut schedule = self
        .beneficiaries
        .get(&beneficiary)
        .expect("No vesting schedule found for the beneficiary");

    let current_time = env::block_timestamp();
    assert!(
        current_time >= schedule.start_time,
        "Vesting period has not started yet"
    );

    let vested_amount = self.calculate_vested_amount(&schedule, current_time);
    assert!(
        vested_amount > schedule.released_amount,
        "No tokens available for withdrawal"
    );

    let releasable_amount = vested_amount - schedule.released_amount;
    assert!(amount <= releasable_amount, "Requested amount exceeds available balance");

    schedule.released_amount += amount;
    self.beneficiaries.insert(&beneficiary, &schedule);

    Promise::new(beneficiary).transfer(amount);
    env::log_str(&format!("Released {} tokens to {}", amount, beneficiary));
}

#[near_bindgen]
impl TokenVesting {
    // Get current vesting schedule for a beneficiary
    pub fn get_vesting_schedule(&self, beneficiary: AccountId) -> Option<VestingSchedule> {
        self.beneficiaries.get(&beneficiary)
    }

    // Get remaining withdrawable amount for a beneficiary
    pub fn get_remaining_amount(&self, beneficiary: AccountId) -> Balance {
        let schedule = self
            .beneficiaries
            .get(&beneficiary)
            .expect("No vesting schedule found for the beneficiary");

        let current_time = env::block_timestamp();
        let vested_amount = self.calculate_vested_amount(&schedule, current_time);
        vested_amount - schedule.released_amount
    }
}
