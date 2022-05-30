use crate::error::ErrorCode;
use pyth_sdk_solana::{load_price_feed_from_account_info, PriceFeed, Price};
use solana_maths::Decimal;

pub const MAX_HEALTH_FACTOR: u128 = 75;
pub const MIN_HEALTH_FACTOR: u128 = 70;
pub const OPTIMAL_HEALTH_FACTOR: u128 = 70;

#[derive(PartialEq)]
pub enum Health {
    Vegan = 0, // All good
    Vegetarian = 1, // Too much cheese -> repay assets
    Keto = 2, // Need some pasta -> borrow more assets
}