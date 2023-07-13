export PRINCIPAL=$(dfx identity get-principal)

dfx deploy icrc_ledger_test --argument "(variant { Init = record {
    minting_account =  record { owner = principal \"$PRINCIPAL\" };
    transfer_fee = 10_000;
    token_symbol = \"DATA\";
    token_name = \"Data Detective Token\";
    metadata = vec {};
    initial_balances = vec {};
    archive_options = record {
        num_blocks_to_archive = 2000;
        trigger_threshold = 1000;
        controller_id = principal \"$PRINCIPAL\";
    };
}})"
