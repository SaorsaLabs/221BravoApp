<script>
    import { onMount } from 'svelte';
	import Pagination from '../shared/pagination.svelte';
    import Button from '../shared/button.svelte';
    import Modal from '../shared/modal.svelte';
    import info from '$lib/images/info.png';
    import search from "$lib/images/search.png";
    import CopyButton from '../shared/copyButton.svelte';
    import SaveButton from '../shared/saveButton.svelte';

    export let data = [];
    export let is_icrc = false; 

    let sortedData = data;
    let dataLen = sortedData?.length ?? 0;
	const tableHeaders = ['#','Account','Transactions','Gross','Net','info'];
    const dataNames = ['count','ID', 'transactions', 'grossValue', 'netValue','name','shortID', 'token'];
	let selectedHeader = "#";
	let ascendingOrder = false;
    let clickRowData = {};
    let isMaxedOut = false;
	if (dataLen >= 10000) { isMaxedOut = true; }
	// SORT BY NUMBER
	const sortByNumber = (colHeaderInput) => {

        // translate col headers to values
        let dataNM;
        if(colHeaderInput == tableHeaders[0]) dataNM = dataNames[0];
        else if (colHeaderInput == tableHeaders[1]) dataNM = dataNames[1]; // use block for date/time
        else if (colHeaderInput == tableHeaders[2]) dataNM = dataNames[2]; 
        else if (colHeaderInput == tableHeaders[3]) dataNM = dataNames[3];
        else if (colHeaderInput == tableHeaders[4]) dataNM = dataNames[4];
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
        else if (colHeaderInput == tableHeaders[1]) dataNM = dataNames[1]; // use block for date/time
        else if (colHeaderInput == tableHeaders[2]) dataNM = dataNames[2]; 
        else if (colHeaderInput == tableHeaders[3]) dataNM = dataNames[3];
        else if (colHeaderInput == tableHeaders[4]) dataNM = dataNames[4];
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
    let itemsPerPage = 10;
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
                    <span class="text-warning">*Showing first 10,000 links</span>
                {/if}
            </td>
        </tr>
    </table>
    <table class="mainTable">
    <tr>
            {#each tableHeaders as header}
                <th 
                    class:highlighted={selectedHeader === header}
                    on:click={() => (header === "#" || header === "Transactions" || header === "Gross" || header == "Net" ) ? sortByNumber(header) : sortByString(header)}>
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
        <td>{TX.count}</td>
        <td>
            {#if is_icrc == false}
                {#if !TX?.name}
                {TX.shortID}
                {:else}
                <span class="text-info">{TX.name}</span>
                {/if}
            {:else}
                {#if !TX?.name}
                {TX.shortID}
                {:else}
                <span class="text-info">{TX.name}</span>
                {/if}
                {@html "<br>"}
                {#if TX?.shortSA != "N/A"}
                {TX.shortSA}
                {/if}
            {/if}
        </td>
        <td>{TX.transactions}</td>
        <td>{TX.grossValue}</td>
        <td>
            {#if TX.netValue > 0}
            <span style="color:limegreen; font-weight:bolder">{TX.netValue}</span>
            {:else}
            <span style="color:orangered; font-weight:bolder">{TX.netValue}</span>
            {/if}
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
	
<Modal open={showPopup} title={"Linked Account:"} size={"large"} onClosed={() => onPopupClose()}> 
    {#if is_icrc == false}
    <span class="fontSpan">
        {clickRowData.ID}
        <a href="/search/ID/{clickRowData.token}?id={clickRowData.ID}&sub=''" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
        <CopyButton text={clickRowData.ID}/>
        <SaveButton text={clickRowData.ID} icrcAccount={false}/>
        {@html "<br>"}   
    </span>
        {#if clickRowData?.name}
        <span class="fontSpan"> {clickRowData?.name} </span>
        {/if}

        {:else}
        <span class="fontSpan textAdj">
            Principal :
            {clickRowData.splitID}
            <a href="/search/ID/{clickRowData.token}?id={clickRowData.splitID}&sub={clickRowData.splitSA}" target="_blank"> <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> </a>
            <CopyButton text={clickRowData.ID}/>
            <SaveButton text={clickRowData.ID} icrcAccount={false}/>
            {@html "<br>"}  
                {#if clickRowData?.shortSA != "N/A"}
                Sub-Account :
                {clickRowData.splitSA}
                {/if}
        </span>
        {/if}
</Modal>

<style>
    @media screen and (max-width:800px){
        .textAdj{
            font-size: 0.80rem;
        }   
    }
    @media screen and (max-width:992px){
        .textAdj{
            font-size: 0.85rem;
        }   
    }

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
    .fontSpan{
        margin-left: 3px;
        font-size: 12px;
        }
    @media (min-width: 992px) 
    {
        .fontSpan{
        margin-left: 10px;
        font-size: 16px;
        }
    }
</style>