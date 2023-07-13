<script>
    import { onMount } from 'svelte';
	import Pagination from '../shared/pagination.svelte';
    import Button from '../shared/button.svelte';
    import Modal from '../shared/modal.svelte';
    import TxPopupTable from '../shared/txPopupTable.svelte';
    import {DEFAULT_SUBACCOUNT} from '../code/constants.js';

    export let txData = [];
    export let usePrincipal = true;
    export let popupType = 'noPrincipalBlock'; // standard  noPrincipal noPrincipalBlock principalBlock icrcBlock
    export let is_icrc = false;

    if(is_icrc) popupType = "icrcBlock";

    let sortedData = [];
    sortedData = txData;
    let dataLen = 0;
    dataLen = sortedData?.length ?? 0;
	const tableHeaders = ['Block','Date/Time','From','To','Value','info'];
    const dataNames = ['block', 'date', 'time', 'fromShortID', 'toShortID',
                     'toPrincipal', 'toAccount', 'fromPrincipal', 'fromAccount',
                     'value','hash','token','type'];
	let selectedHeader = "Block";
	let ascendingOrder = false;
    let clickRowData = {};
	
	// SORT BY NUMBER
	const sortByNumber = (colHeaderInput) => {

        // translate col headers to values
        let dataNM;
        if(colHeaderInput == tableHeaders[0]) dataNM = dataNames[0];
        else if (colHeaderInput == tableHeaders[1]) dataNM = dataNames[0]; // use block for date/time
        else if (colHeaderInput == tableHeaders[2] && usePrincipal)  dataNM = dataNames[7];
        else if (colHeaderInput == tableHeaders[2] && !usePrincipal) dataNM = dataNames[8];  
        else if (colHeaderInput == tableHeaders[3] && usePrincipal)  dataNM = dataNames[5];
        else if (colHeaderInput == tableHeaders[3] && !usePrincipal) dataNM = dataNames[6];
        else if (colHeaderInput == tableHeaders[4]) dataNM = dataNames[9];
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
        else if (colHeaderInput == tableHeaders[1]) dataNM = dataNames[0]; // use block for date/time
        else if (colHeaderInput == tableHeaders[2] && usePrincipal)  dataNM = dataNames[7];
        else if (colHeaderInput == tableHeaders[2] && !usePrincipal) dataNM = dataNames[8];  
        else if (colHeaderInput == tableHeaders[3] && usePrincipal)  dataNM = dataNames[5];
        else if (colHeaderInput == tableHeaders[3] && !usePrincipal) dataNM = dataNames[6];
        else if (colHeaderInput == tableHeaders[4]) dataNM = dataNames[9];
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
    <table>
        <tr>
            <td>
                <div class="radioPadding textLeft">
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
                </div>
            </td>
            <td class="textRight">
                 Results : {dataLen}
            </td>
        </tr>
    </table>

    <table id="myTable">
    <tr>
            {#each tableHeaders as header}
            <th 
                class:highlighted={selectedHeader === header}
                on:click={() => (header === "Block" || header === "Value" || header === "Date/Time" ) ? sortByNumber(header) : sortByString(header)}>
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
                <td>{TX.block}</td>
                <td>{TX.date}{@html "<br>"}{TX.time}</td>
                <td>
                    {TX.fromShortID}
                    {#if TX.fromAccount != DEFAULT_SUBACCOUNT && is_icrc == true}
                        {@html "<br>"}
                        {TX.fromShortSA}
                    {/if}
                </td>
                <td>
                    {TX.toShortID}
                    {#if TX.toAccount != DEFAULT_SUBACCOUNT &&  is_icrc == true}
                        {@html "<br>"}
                        {TX.toShortSA}
                    {/if}
                </td>
                <td>{TX.value}</td>
                <td>
                    <Button type="grey" slim={true} flat={true} noBG={false} on:click={() => infoButton(TX)} >ⓘ info</Button>
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