1) Download ICRC - Insructions here - https://github.com/dfinity/ic/tree/master/rs/rosetta-api/icrc1/ledger
2) icrc_download.sh containts the commands. 
3) run icrc_setup.sh using your chosen dfx identity
4) open icrc_state.sh and change the first and last lines to match your chosen dfx identity
5) run icrc_state.sh

results
20 transactions in canister state 
10 mint, 9 transfer, 1 burn

uses principal "2vxsx-fae" and first 10 subaccounts. 
eg  subaccount 0 = 0000000000000000000000000000000000000000000000000000000000000000
    subaccount 1 = 0000000000000000000000000000000000000000000000000000000000000001
    ...
    subaccount 9 = 0000000000000000000000000000000000000000000000000000000000000009

Totals   
sub0 =  998909640
sub1 = 1000100010
sub2 = 1000100020
sub3 = 1000100030
sub4 = 1000100040
sub5 = 1000100050
sub6 = 1000100060 (3)
sub7 = 1000100070 (2)
sub8 = 1000100080 (1) largest sub
sub9 = 1000000000

total held by 2vxsx-fae = 9999710000 (1) largest principal