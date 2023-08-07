<script>
    import Button from '../../shared/button.svelte';
    import LineChart from '../charts/lineChart.svelte';
    import { getICRC_SnapshotQuickStats, getICP_SnapshotQuickStats} from '../../code/fetchStats';

    export let token = "Tokens";
    export let is_icrc = true;
    let numDays = 60;

    let promise = loadStuff();

    let opAccounts = [];
    let opPrincipals = [];
    let labels = [];

    async function loadStuff(){
        if(is_icrc == true || is_icrc == "true"){
           let data = await getICRC_SnapshotQuickStats(token, numDays);
           // process
           let len = data[0]?.total_unique_accounts.length ?? 0;
           let i;
           var timeMilli, date, shortDate; 
           for(i=0; i<len; i++) {
                opAccounts.push(Number(data[0]?.total_unique_accounts[i][0]));
                opPrincipals.push(Number(data[0]?.total_unique_principals[i][0]));
                timeMilli = Number(data[0]?.total_unique_accounts[i][1]) / 1000000;
                date = new Date(timeMilli);
                shortDate = date.toLocaleDateString();
                labels.push(shortDate);
           }
        } else {
          let data = await getICP_SnapshotQuickStats(token, numDays);
           // process
           let len = data[0]?.total_unique_accounts.length ?? 0;
           let i;
           var timeMilli, date, shortDate; 
           for(i=0; i<len; i++) {
                opAccounts.push(Number(data[0]?.total_unique_accounts[i][0]));
                timeMilli = Number(data[0]?.total_unique_accounts[i][1]) / 1000000;
                date = new Date(timeMilli);
                shortDate = date.toLocaleDateString();
                labels.push(shortDate);
           }
        }
    }
    </script>
    
    <div style="width:100%; padding:10px;">
        {#await promise}
        <p class="cntr">Loading Data... </p>
        {:then}
          {#if is_icrc}
            <h4 style="padding-left:5px; padding-top:5px;">{token} Daily Active Accounts/ Principals </h4>
                <LineChart 
                    dataArray1={opAccounts} 
                    dataArray2={opPrincipals}
                    labelsArray={labels}
                    dataset1Title={"Active Accounts"}
                    dataset2Title={"Active Principals"}
                    reverse={true};
                />
          {:else}
            <h4 style="padding-left:5px; padding-top:5px;">{token} Daily Active Accounts </h4>
            <LineChart 
                dataArray1={opAccounts} 
                labelsArray={labels}
                dataset1Title={"Active Accounts"}
                reverse={true};
            />
          {/if}
        {/await}
    </div>
    
    <style>

        /* Media query for smaller screens */
        @media (max-width: 900px) {
          .box {
            flex-basis: 100%; /* Boxes will take up 100% of the container width */
          }
          .box3 {
            flex-basis: 100%; /* Boxes will take up 100% of the container width */
          }
        }
        .lineBox{
		max-height: 500px;
	}
    </style>