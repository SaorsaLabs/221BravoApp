dfx identity use SaorsaDev

# Mint into 10 sub accounts (0-9)
for ((i=0; i<10; i++))
do
  # Generate SubAccount Vec
  number=$i
  padded_number=$(printf "%032d" "$number")
  semicolon_string=$(sed 's/./&;/g' <<< "$padded_number")
  output=${semicolon_string::-1}  # Remove the last semicolon
  modified_string="{$output}"

  dfx canister call icrc_ledger_test icrc1_transfer '(record {
  to = record {
    owner = principal "2vxsx-fae"; 
    subaccount = opt vec '$modified_string'};
  amount=1_000_000_000
  },)'

  echo "Mint sent to account $i"
done

dfx identity use anonymous

# Transfers
for ((i=0; i<9; i++))
do
  # Generate SubAccount Vec
  number=$i
  padded_number=$(printf "%032d" "$number")
  semicolon_string=$(sed 's/./&;/g' <<< "$padded_number")
  output=${semicolon_string::-1}  # Remove the last semicolon
  modified_string="{$output}"
  dfx canister call icrc_ledger_test icrc1_transfer '(record {
    to = record {
            owner = principal "2vxsx-fae"; 
            subaccount = opt vec {0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;'$i'}};  
    memo=null;
    fee=null;
    created_at_time=null;
    amount=1000'$i'0;
    })'

  echo "Sentfrom SubAccount 0 to SubAccount $number"
done

for ((i=9; i>0; i--))
do
  # Generate SubAccount Vec
  number=$i
  padded_number=$(printf "%032d" "$number")
  semicolon_string=$(sed 's/./&;/g' <<< "$padded_number")
  output=${semicolon_string::-1}  # Remove the last semicolon
  modified_string="{$output}"
  dfx canister call icrc_ledger_test icrc1_transfer '(record {
    to = record {
            owner = principal "2vxsx-fae"; 
            subaccount = opt vec {0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;'$i'};  
    memo=null;
    fee=null;
    created_at_time=null;
    amount=100'$i'00;
    })'

  echo "Sentfrom SubAccount $number to SubAccount 0"
done

# BURN TRANSACTIONS
for ((i=0; i<9; i++))
do
  # Generate SubAccount Vec
  number=$i
  dfx canister call icrc_ledger_test icrc1_transfer '(record 
  {to=record {owner=principal "e3uc3-o4g2j-bdkhp-yi4p4-wzfdy-glkas-zlhqf-n2jm2-ehxiv-fnjkc-2ae"; 
  subaccount=null}; 
  fee=null; 
  memo=null; 
  from_subaccount= opt vec {0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;0;'$i'}}; 
  created_at_time=null; 
  amount=200000})'
  echo "Burn Transaction complete  - $i"
done


dfx identity use SaorsaDev







# record {
#     minting_account :  record { owner = principal \"e3uc3-o4g2j-bdkhp-yi4p4-wzfdy-glkas-zlhqf-n2jm2-ehxiv-fnjkc-2ae\"  };
#     fee_collector_account : opt Account;
#     transfer_fee : nat64;
#     token_symbol : text;
#     token_name : text;
#     metadata : vec record { text; MetadataValue };
#     initial_balances : vec record { Account; nat64 };
#     archive_options : record {
#         num_blocks_to_archive : nat64;
#         trigger_threshold : nat64;
#         max_message_size_bytes : opt nat64;
#         cycles_for_archive_creation : opt nat64;
#         node_max_memory_size_bytes : opt nat64;
#         controller_id : principal;
#     };
# };