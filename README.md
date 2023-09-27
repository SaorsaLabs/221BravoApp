# 221BravoApp
Home for Data-Detectives in the Internet Computer Ecosystem.

221Bravo is a blockchain explorer primarily focused on the Internet Computer Ecosystem giving users the ability to navigate complex data in a simple and understandable way. 
The app is broken down into several sections:

-	Account and block explorer. Covering ICP and SNS (ICRC) tokens. Members get additional information on each account including linked accounts, user saved names and public known names. 
-	Token Stats covering ICP and SNS (ICRC) tokens. Top holders, daily transaction volumes, mint/ burn, whale moves and other great metrics. 
-	Visual Block Explorer allowing the user to explore the latest ICP/ ICRC transactions – quickly identifying links and relationships between accounts. 
-	Members page featuring exclusive tools and members only chat (using Open Chat)

## Overview of Frontend/ Backend canisters 

![Canister Layout](221BravoApp.png)

### Main App and Backend
The application frontend is built using SvelteKit, Vite, HTML/ CSS with Javascript fetching  data from the various other project canisters. The frontend is hosted in single asset canister (src/frontend)
There are 3 main canisters which provide backend support for the frontend canister. 

The main backend canister (src/backend) retains details of users’ (by NFT address). Users’ address book, public named accounts, and methods for converting Principals into ICP sub-accounts. 

The dynamic content canister (src/dynamic_content) stores details of news items and ecosystem projects displayed on index page and ecosystem page. 

The assets canister (src/assets_canister) acts as a dynamic store for news items and ecosystem project logos. 

### Token Stats Canisters
There are two types of Token Stats Canisters – the ICP version (src/icp_data_processer) and ICRC version (src/icrc_data_processer). Both canisters undertake the same function however due to the differences between ICP accounts and ICRC accounts two variants were created. 

In brief, these canisters fetch transactions from a designated ledger canister and calculate a number of stats from these transactions – for example the largest transactions over the last 24 hours or 30 days. The token stats canister runs a timer which schedules when the canister pulls the latest data and updates the stats. The stats are ‘live’ and no history is retained (see Snapshot canister).  

The frontend canister pulls stats directly from the ICP/ ICRC stats canisters. 

### Token Indexing Canisters
The token indexing canister are again spit into two versions – one for ICP transactions and one for ICRC transactions. Each version requires a pair of canisters. The main canister (src/icrc_super_index) performs the calculations and the transaction store (src/icrc_tx_store) acts as a storage canister for the transactions processed by the indexing canister. 

This canister forms the main backbone of the account searching tool on 221Bravo App. This canister also provides methods to get specific blocks or the latest blocks. 

### Account Tracking and Stats Snapshots

As stated above, the token stats canisters and token indexing canister don’t retain a history of their outputs. To provide historical statistics snapshot canisters (src/icrc_snapshot_canister) and (src/icp_snapshot_canister) were added. These canister run on a user-defined timer which fetches data from the token stats, token index and tracking canister and retains these for querying by the frontend canister. 

As part of the app features there is a requirement to track certain accounts (for example mixers or fraudsters) and flag these to the end user. To enable this functionality a tracking canister (src/tracking_canister) and flagging canister (scr/flags_canister) were added. The frontend canister queries the flagging canister when performing account searches to determine if the account has any flags. Currently there are 5 flags 

-	Genesis Flag – added to each of the ICP Genesis Accounts
-	Fraud Flag – for any accounts which have been identified/ verified as being involved in a scam/ fraud. 
-	Mixer Flag – for any accounts which have been identified as using a mixer service
-	Community Flag – for any accounts flagged by the community
-	SAR Flag – for suspicious activity reports. 


## Using App canisters 
Each canister in the app is designed to be deployed independently of each other. Any types or functions required by the canister will be contained in the canister’s source folder. 

The canisters have been designed to have an initial ‘admin’ which is hard-coded into the canister’s init function. All methods on the canister are gated so that only ‘authorised’ principals can get data from them. This can be set to “2vxsx-fae” if you want everyone to be able to call the canister’s functions.

The super index canister is slightly different as it has two levels of authorisation – Admin and authorised. Admin gates control all the core settings and timers etc whilst authorised gates control who can read the data stored in the canister. 

Any canister with a timer is designed to be deployed first, then updated with desired settings before starting the timer function. 

## License
This project is licensed under the MIT license, see LICENSE.md for details. See CONTRIBUTE.md for details about how to contribute to this project. 

## Acknowledgements
The 221Bravo App could not have been created without the help and support of some ICP mega-brains. 

- There are several parts of the application which references code obtained from the Dfinity Github Repo (https://github.com/dfinity). For example account_identifiers.rs within the backend canister forms part of Dfinity’s ic-repl repo. These are all referenced within the code.
- The ICP/ ICRC super indexer canisters utilises IC Stable Memory repo for stable storage of data. (https://github.com/seniorjoinu/ic-stable-memory/tree/master)
- The setup of Sveltekit/ vite was helped by reference to the awesome Juno App (https://github.com/buildwithjuno/juno)
- And not forgetting the mega-brains on the Dfinity developer who help squash some bugs (https://forum.dfinity.org/)

## References
- [Internet Computer](https://internetcomputer.org)

