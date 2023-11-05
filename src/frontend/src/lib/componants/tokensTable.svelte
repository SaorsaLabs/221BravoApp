<script>
    import { onMount } from 'svelte';
	import Pagination from '../shared/pagination.svelte';
    import Button from '../shared/button.svelte';
    import Loading from '../shared/loading.svelte';
    import { getTokenTableData } from '../code/fetchStats.js';
    import TokenLogos from '../shared/tokenLogos.svelte';

    let promise = loadStuff();
    let tokenData;
    async function loadStuff(){
        tokenData = await getTokenTableData();
        tokenData.sort((a, b) => b.active - a.active);
        sortedData = tokenData;
        paginate(sortedData);
    }

    let sortedData = [];
    
    let dataLen = 0;
    dataLen = sortedData?.length ?? 0;
	const tableHeaders = ['','Token','Total Holders','24hr Transaction Count','24hr Active Accounts','[-]'];
    const dataNames = ['logo', 'token', 'holders', 'transactions', 'active'];
	let selectedHeader = "Token";
	let ascendingOrder = false;
    let clickRowData = {};
	
	// SORT BY NUMBER
	const sortByNumber = (colHeaderInput) => {

        // translate col headers to values
        let dataNM;
        if(colHeaderInput == tableHeaders[0]) dataNM = dataNames[0];
        else if (colHeaderInput == tableHeaders[1]) dataNM = dataNames[1]; 
        else if (colHeaderInput == tableHeaders[2]) dataNM = dataNames[2];
        else if (colHeaderInput == tableHeaders[3]) dataNM = dataNames[3];
        else if (colHeaderInput == tableHeaders[4]) dataNM = dataNames[4];
        else if (colHeaderInput == tableHeaders[5]) dataNM = dataNames[3];

		sortedData = sortedData.sort((obj1, obj2) => {
			return ascendingOrder ? Number(obj1[dataNM]) - Number(obj2[dataNM])
			: Number(obj2[dataNM]) - Number(obj1[dataNM])
		});
		selectedHeader = colHeaderInput;
        paginate(sortedData);
	}
	
	// SORT BY STRINGs
	const sortByString = (colHeaderInput) => {
        // translate col headers to values
        let dataNM;
        if(colHeaderInput == tableHeaders[0]) dataNM = dataNames[0];
        else if (colHeaderInput == tableHeaders[1]) dataNM = dataNames[1]; 
        else if (colHeaderInput == tableHeaders[2]) dataNM = dataNames[2];
        else if (colHeaderInput == tableHeaders[3]) dataNM = dataNames[3];
        else if (colHeaderInput == tableHeaders[4]) dataNM = dataNames[4];
        else if (colHeaderInput == tableHeaders[5]) dataNM = dataNames[3];

		sortedData = sortedData.sort((obj1, obj2) => {
			if (obj1[dataNM] < obj2[dataNM]) {
					return -1;
			} else if (obj1[dataNM] > obj2[dataNM]) {
				return 1;
			}
			return 0; //string code values are equal		
		});
		if (!ascendingOrder) {
			sortedData = sortedData.reverse()
		}
		selectedHeader = colHeaderInput;
        paginate(sortedData);
	}

    //pagination
    let page = 0;
    let totalPages = [];
    let itemsPerPage = 25;
    let currentPageRows = [];
    let maxPage;

    $: currentPageRows = totalPages.length > 0 ? totalPages[page] : [];
    const paginate = (items) => {
    const pages = Math.ceil(items.length / itemsPerPage);
    maxPage = pages;
    const paginatedItems = Array.from({ length: pages }, (_, index) => {
      const start = index * itemsPerPage;
      return items.slice(start, start + itemsPerPage);
    });
    totalPages = [...paginatedItems];
  };

    function pageChange(event){
        page = event.detail.page-1; //-1 to match array
    }

    function infoButton(row){
        clickRowData = row;
        onShowPopup();
    }

    onMount(async () => {
        
	});

    // MODAL STUFF
	let showPopup = false;
	const onShowPopup = (ev) => {
		showPopup = true;
	}
	const onPopupClose = (data) => {
		showPopup = false;
		//console.log(data); <-- any return data from modal
	}
</script>

<div class="mainAlign">
    {#await promise}
        <Loading/>
    {:then} 
        <table id="myTable">
        <tr>
        <!-- HEAD  -->
        {#each tableHeaders as header}
        <th 
            class:highlighted={selectedHeader === header}
            on:click={() => (header === "Total Holders" || header === "24hr Transaction Count" || header === "24hr Active Accounts" ) ? sortByNumber(header) : sortByString(header)}>
                {header}

            {#if header === selectedHeader}	
                <!-- svelte-ignore a11y-click-events-have-key-events -->
                <span class="order-icon" on:click={() => ascendingOrder = !ascendingOrder}>
                    {@html ascendingOrder ? "&#9661;" : "&#9651;"}
                </span>		
            {/if}	
        </th>	
        {/each}

        <!-- CONTENT currentPageRows -->
        </tr>
            {#each currentPageRows as TKN} 
                <tr>
                    <td>
                       <div style="width: 30px;">
                            <TokenLogos token={TKN.token} width={'50px'}/>
                       </div>
                    </td>
                    <td>
                        {TKN.shortName}
                    </td>
                    <td>{TKN.holders}</td>
                    <td>{TKN.transactions}</td>
                    <td>{TKN.active}</td>
                    <td>
                        <a href="./explore/stats/token/{TKN.link}/"><Button type="blueTP" slim={true} flat={true} noBG={false} >Token {@html "<br>"} Stats</Button></a> 
                        <a href="./explore/visualblocks/token/{TKN.link}/"><Button type="blueTP" slim={true} flat={true} noBG={false} >Visual {@html "<br>"} Explorer</Button></a>
                        <a href="./search/token/{TKN.link}/"><Button type="blueTP" slim={true} flat={true} noBG={false} > Search {@html "<br>"} Token </Button></a>
                    </td>
                </tr>	
            {/each}
        </table>
    <Pagination max={maxPage} on:click={pageChange}/>
    {/await}
</div>
	
<style>
    .mainAlign{
        padding: 10px;
	}
    .radioPadding{
        padding-left: 5px;
        padding-bottom: 5px;
    }
	table {
		border-spacing: 0;
		width: 100%;
        margin-bottom: 10px;
	}
	
	th {
		/* text-transform: uppercase; */
        font-size: larger;
		cursor: pointer;
        background-color: rgba(61, 61, 61, 0.5);
	}
	
	.order-icon {
		color: hsl(53, 100%, 45%);
	}
	
	.highlighted {
		color: hsl(53, 100%, 45%);
	}
	
	th, td {
		text-align: center;
		padding: 5px;
	}

	tr:nth-child(even) {
		background-color: rgba(192, 190, 155, 0.1)
	}
    .textLeft{
        text-align: left;
    }
    .textRight{
        text-align: right;
    }
    .linkButtons{
        padding: 2px;
    }
</style>