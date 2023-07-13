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

# Transfers (0-8 transfer to account 9)
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

# BURN TRANSACTION
dfx canister call icrc_ledger_test icrc1_transfer '(record 
{to=record {owner=principal "e3uc3-o4g2j-bdkhp-yi4p4-wzfdy-glkas-zlhqf-n2jm2-ehxiv-fnjkc-2ae"; 
subaccount=null}; 
fee=null; 
memo=null; 
from_subaccount=null; 
created_at_time=null; 
amount=200000})'
echo "Burn Transaction complete"

dfx identity use SaorsaDev
