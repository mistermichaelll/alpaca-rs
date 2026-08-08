use alpaca_rs::Alpaca;

#[test]
fn test_account_deserialization() {
    // A sample JSON body exactly as it comes from Alpaca
    let raw_json = r#"{
                "id": "12345",
                "admin_configurations": {},
                "user_configurations": null,
                "account_number": "987654321",
                "status": "active",
                "crypto_status": "approved",
                "options_approved_level": 1,
                "options_trading_level": 1,
                "currency": "USD",
                "buying_power": "10000.00",
                "regt_buying_power": "10000.00",
                "effective_buying_power": "10000.00",
                "non_marginable_buying_power": "5000.00",
                "options_buying_power": "2000.00",
                "cash": "10000.00",
                "accrued_fees": "0.00",
                "portfolio_value": "10000.00",
                "trading_blocked": false,
                "transfers_blocked": false,
                "account_blocked": false,
                "created_at": 1672531200,
                "trade_suspended_by_user": false,
                "multiplier": "1.0",
                "shorting_enabled": true,
                "last_equity": "10000.00",
                "long_market_value": "5000.00",
                "short_market_value": "0.00",
                "position_market_value": "5000.00",
                "initial_margin": "1000.00",
                "maintenance_margin": "1000.00",
                "last_maintenance_margin": "1000.00",
                "sma": "1000.00",
                "balance_asof": 2023-01-01T00:00:00Z,
                "crypto_tier": 1,
                "intraday_adjustments": "0.00",
                "pending_reg_taf_fees": "0.00"
            }"#;

    let account: Account = serde_json::from_str(raw_json).expect("Failed to parse sample JSON");
    assert_eq!(account.id, "12345");
    assert_eq!(account.currency, "USD");
}
