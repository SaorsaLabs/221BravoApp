
#  NOTE - DEPLOY ALL data_processer, super_index and tx_store canisters FIRST!! 
tokenID="NUANCE"
tokenLedger="rxdbk-dyaaa-aaaaq-aabtq-cai"
dataProcesserCanister="burde-oqaaa-aaaak-qcmwa-cai"
superIndexerCanister="btqfq-diaaa-aaaak-qcmwq-cai"
txStore="b2tom-vaaaa-aaaak-qcmxa-cai"

# auth snapshot canister (DONT CHANGE!)
dfx canister call icrc_data_processer --network ic add_authorised '("7hcof-tiaaa-aaaak-qcjfa-cai")'
# name
dfx canister call icrc_data_processer --network ic set_canister_name '("Stats Canister - '${tokenID}' - 221Bravo App")'
# Set timescales
dfx canister call icrc_data_processer --network ic set_stats_timescales '(24: nat64, 30: nat64)'
# Set Token Ledger Canister 
dfx canister call icrc_data_processer --network ic set_target_canister '( "'${tokenLedger}'" )'
# Set timer
dfx canister call icrc_data_processer --network ic check_and_start_processing_timer '(900: nat64)'
# remove 2vxsx-fae as auth
dfx canister call icrc_data_processer --network ic remove_authorised '("2vxsx-fae")'

# Add token to snapshot collection
dfx canister call icrc_snapshot_canister --network ic add_collection '("'$tokenID'","'$dataProcesserCanister'")'

# name tx store
dfx canister call icrc_tx_store --network ic set_canister_name '("'$tokenID' TX Store - 221Bravo App")'
# add super index as auth/ admin
dfx canister call icrc_tx_store --network ic add_authorised '("'$superIndexerCanister'")'
dfx canister call icrc_tx_store --network ic add_admin '("'$superIndexerCanister'")'

# name super index
dfx canister call icrc_super_indexer --network ic set_canister_name '("'$tokenID' - Super Indexer - 221Bravo App")'
# set_target_canister
dfx canister call icrc_super_indexer --network ic set_target_canister '("'$tokenLedger'","'$txStore'","'$superIndexerCanister'")'
# auth frontend + tracking canister
dfx canister call icrc_super_indexer --network ic add_authorised '("ztewi-mzfkq-w57f2-xtl6i-kacap-n2gg6-dxyzu-p3oql-aikxf-rsivy-aqe")'
dfx canister call icrc_super_indexer --network ic add_authorised '("gnfso-uqaaa-aaaak-qclzq-cai")'
# start processing timer
dfx canister call icrc_super_indexer --network ic check_and_start_processing_timer '(300: nat64)'

