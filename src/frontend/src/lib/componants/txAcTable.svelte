<script>
    import { onMount } from 'svelte';
	import Pagination from '../shared/pagination.svelte';
    import Button from '../shared/button.svelte';
    import Modal from '../shared/modal.svelte';
    import TxPopupTable from '../shared/txPopupTable.svelte';
    import deposit from '$lib/images/incoming.png';
    import payment from '$lib/images/outgoing.png';
    import info from '$lib/images/info.png';

    export let txData = [];
    export let popupType = 'noPrincipal'; // standard noPrincipal noPrincipalBlock principalBlock icrc
    export let is_icrc = false;

    if(is_icrc) popupType = "icrc";

    let sortedData = txData;
    let dataLen = sortedData?.length ?? 0;
    let isMaxedOut = false;
    if (dataLen >= 1000) isMaxedOut = true; // max 1k transactions delivered
	const tableHeaders = ['#','Date/Time','Account','Direction','Value','info'];
    const dataNames = ['counter', 'date', 'time', 'shortID', 'direction', 'value', 'hash', 'block', 'longID'];
	let selectedHeader = "#";
	let ascendingOrder = false;
    let clickRowData = {};
	
	// SORT BY NUMBER
	const sortByNumber = (colHeaderInput) => {

        // translate col headers to values
        let dataNM;
        if(colHeaderInput == tableHeaders[0]) dataNM = dataNames[0];
        else if (colHeaderInput == tableHeaders[1]) dataNM = dataNames[7]; // use block for date/time
        else if (colHeaderInput == tableHeaders[2]) dataNM = dataNames[8]; 
        else if (colHeaderInput == tableHeaders[3]) dataNM = dataNames[4];
        else if (colHeaderInput == tableHeaders[4]) dataNM = dataNames[5];
        else if (colHeaderInput == tableHeaders[5]) dataNM = dataNames[0];

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
        else if (colHeaderInput == tableHeaders[1]) dataNM = dataNames[7]; // use block for date/time
        else if (colHeaderInput == tableHeaders[2]) dataNM = dataNames[8]; 
        else if (colHeaderInput == tableHeaders[3]) dataNM = dataNames[4];
        else if (colHeaderInput == tableHeaders[4]) dataNM = dataNames[5];
        else if (colHeaderInput == tableHeaders[5]) dataNM = dataNames[0];

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
        paginate(sortedData);
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
    <table style="width:100%;">
        <tr>
            <td class="textLeft">
                    Per Page:
                    <label>
                        <input type=radio bind:group={itemsPerPage} value={10} on:change={() => paginate(sortedData)}>
                        10
                    </label>
                    
                    <label>
                        <input type=radio bind:group={itemsPerPage} value={25} on:change={() => paginate(sortedData)}>
                        25
                    </label>
                    
                    <label>
                        <input type=radio bind:group={itemsPerPage} value={50} on:change={() => paginate(sortedData)}>
                        50
                    </label>
                    <label>
                        <input type=radio bind:group={itemsPerPage} value={100} on:change={() => paginate(sortedData)}>
                        100
                    </label>
            </td>
            <td class="textRight">
                 Results : {dataLen}
                 {#if isMaxedOut == true}
                    {@html "<br>"}
                    <span class="text-warning">*Most recent 1,000 transactions. Older transactions not shown!</span>
                 {/if}
            </td>
        </tr>
    </table>
    <table class="mainTable">
    <tr>
            {#each tableHeaders as header}
            <th 
                class:highlighted={selectedHeader === header}
                on:click={() => (header === "#" || header === "Value" || header === "Date/Time" ) ? sortByNumber(header) : sortByString(header)}>
                    {header}

                {#if header === selectedHeader}	
                    <!-- svelte-ignore a11y-click-events-have-key-events -->
                    <span class="order-icon" on:click={() => ascendingOrder = !ascendingOrder}>
                        {@html ascendingOrder ? "&#9661;" : "&#9651;"}
                    </span>		
                {/if}	
            </th>	
            {/each}
    </tr>
        {#each currentPageRows as TX}
            <tr>
                <td>{TX.counter}</td>
                <td>{TX.date}{@html "<br>"}{TX.time}</td>
                <td>
                    {#if is_icrc == false}
                        {TX.shortID}
                    {:else}
                        {TX.shortID}
                        {@html "<br>"}
                        {#if TX?.longSubID != "0000000000000000000000000000000000000000000000000000000000000000" 
                            && TX.shortID != "Mint"
                            && TX.shortID != "Burn"}
                        {TX.shortSA}
                        {/if}
                    {/if}
                </td>
                <td>
                    {#if TX.direction == 'in'}
                    <img class="direction" src={deposit} alt="direction" width="70px"/>
                    {:else}
                    <img class="direction" src={payment} alt="direction" width="70px"/>
                    {/if}
                </td>
                <td>
                    {TX.value}
                </td>
                <td>
                    <Button type="grey" slim={true} flat={true} noBG={false} on:click={() => infoButton(TX)} >
                        <img class="info" src={info} alt="info" width="25px" style="padding-bottom:2px"/>
                        info
                    </Button>
                </td>
            </tr>	
        {/each}
    </table>
    <Pagination max={maxPage} on:click={pageChange}/>
</div>
	
<Modal open={showPopup} title={"Transaction:"} size={"large"} onClosed={() => onPopupClose()}> 
    <TxPopupTable data={clickRowData} type={popupType}/>
</Modal>

<style>
    .mainAlign{
		padding-bottom: 10px;
	}
	.mainTable {
		border-spacing: 0;
		width: 100%;
        margin-bottom: 10px;
	}
	th {
		text-transform: uppercase;
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
</style>