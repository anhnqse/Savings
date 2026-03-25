#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, Address, Env, Vec,
};

#[contracttype]
#[derive(Clone)]
pub struct Club {
    pub members: Vec<Address>,
    pub monthly_amount: i128,
    pub current_month: u32,
    pub total_months: u32,
}

#[contracttype]
pub enum DataKey {
    Club,
    Contributions(u32),
    Paid(u32, Address),
}

#[contract]
pub struct SavingsClub;

#[contractimpl]
impl SavingsClub {
    pub fn init(env: Env, members: Vec<Address>, monthly_amount: i128) {
        if members.len() != 5 {
            panic!("Must have exactly 5 members");
        }

        let club = Club {
            members,
            monthly_amount,
            current_month: 0,
            total_months: 5,
        };

        env.storage().instance().set(&DataKey::Club, &club);
    }

    pub fn contribute(env: Env, from: Address) {
        from.require_auth();

        let mut club: Club = env
            .storage()
            .instance()
            .get(&DataKey::Club)
            .unwrap();

        let month = club.current_month;

        // ❗ check member hợp lệ
        if !club.members.contains(&from) {
            panic!("Not a member");
        }

        let paid_key = DataKey::Paid(month, from.clone());
        if env.storage().instance().has(&paid_key) {
            panic!("Already contributed");
        }

        env.storage().instance().set(&paid_key, &true);

        let count: u32 = env
            .storage()
            .instance()
            .get(&DataKey::Contributions(month))
            .unwrap_or(0);

        env.storage()
            .instance()
            .set(&DataKey::Contributions(month), &(count + 1));

        if count + 1 == 5 {
            Self::distribute(env.clone(), &mut club);
        }
    }

    fn distribute(env: Env, club: &mut Club) {
        let month = club.current_month;

        // ✅ FIX get()
        let recipient = club.members.get_unchecked(month);

        let total = club.monthly_amount * 5;

        // ⚠️ MOCK LOGIC (chưa transfer thật)
        // Bạn sẽ thay bằng token contract sau

        // Move next month
        club.current_month += 1;

        env.storage().instance().set(&DataKey::Club, club);
    }

    pub fn get_status(env: Env) -> (u32, u32) {
        let club: Club = env
            .storage()
            .instance()
            .get(&DataKey::Club)
            .unwrap();

        (club.current_month, club.total_months)
    }
}