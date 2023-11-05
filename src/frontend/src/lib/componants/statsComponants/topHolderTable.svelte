<script>
    import { onMount } from 'svelte';
	import Pagination from '../../shared/pagination.svelte';
    import Button from '../../shared/button.svelte';
    import Modal from '../../shared/modal.svelte';
    import info from '$lib/images/info.png';
    import search from "$lib/images/search_white.png";
    import CopyButton from '../../shared/copyButton.svelte';
    import SaveButton from '../../shared/saveButton.svelte';
    import { Principal } from '@dfinity/principal';

    export let token = "";
    export let data = [];
    export let isIcrc = false;
    export let showSubAccounts = true;

    // type HolderBalance = record {holder: text; balance: nat};
    // type TopHolderResponse = record {top_accounts: vec HolderBalance; top_principals: vec HolderBalance};

    let sortedData = data;
    let dataLen = sortedData?.length ?? 0;
	const tableHeaders = ['#','Holder','Balance','info'];
    const dataNames = ['count','Holder', 'Balance']; // input data fields
	let selectedHeader = "#";
	let ascendingOrder = false;
    let clickRowData = {};
	
	// SORT BY NUMBER
	const sortByNumber = (colHeaderInput) => {

        // translate col headers to values
        let dataNM;
        if(colHeaderInput == tableHeaders[0]) dataNM = dataNames[0];
        else if (colHeaderInput == tableHeaders[1]) dataNM = dataNames[1]; 
        else if (colHeaderInput == tableHeaders[2]) dataNM = dataNames[2]; 
        else if (colHeaderInput == tableHeaders[3]) dataNM = dataNames[0];


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
        else if (colHeaderInput == tableHeaders[3]) dataNM = dataNames[0];


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
            </td>
        </tr>
    </table>
    <table class="mainTable">
    <tr>
            {#each tableHeaders as header}
                <th 
                    class:highlighted={selectedHeader === header}
                    on:click={() => (header === "#"  || header === "Balance" || header == "Info" ) ? sortByNumber(header) : sortByString(header)}>
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
    {#each currentPageRows as ID}
    <tr>
        <td>{ID.holderID}</td>
        <td style="text-align:left;" class="textAdj">
            <!-- Holder -->
            {#if isIcrc == false}
                {ID.account}
            {:else}
                {#if showSubAccounts == true}
                    {ID.principal}
                    {@html "<br>"}
                    <span class="textAdjSml">Sub-Account: {ID.subAccount}</span>
                {:else}
                    {ID.principal}
                {/if}
            {/if}
        </td>
        <td >{ID.balance}</td>
        <td>
            {#if isIcrc == false}
                <a href="/search/token/{token}?id={ID.account}&sub=''" target="_blank"> 
                    <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> 
                </a>
                <CopyButton text={ID.account} modeLight={true}/>
                <SaveButton text={ID.account} icrcAccount={false} modeLight={true}/>
            {:else}
                <a href="/search/token/{token}?id={ID.principal}&sub={ID.subAccount}" target="_blank"> 
                    <img class="search" src={search} alt="search" width="20px" style="margin-left:5px"/> 
                </a>
                {#if ID.hasSubAccount == true}
                    <CopyButton text={ID.principal+"."+ID.subAccount} modeLight={true}/>
                {:else}
                    <CopyButton text={ID.principal} modeLight={true}/>
                {/if}
                <SaveButton text={ID.principal+"."+ID.subAccount} icrcAccount={true} modeLight={true}/>
            {/if}
            
        </td>
    </tr>	
    {/each}
    </table>
    <Pagination max={maxPage} on:click={pageChange}/>
</div>

<style>
    @media screen and (max-width:800px){
        .textAdj{
            font-size: 0.80rem;
        }
        .textAdjSml{
            font-size: 0.60rem;
            font-style: italic;
            color: rgb(185, 185, 185);
        }  
    }
    @media screen and (max-width:992px){
        .textAdj{
            font-size: 0.85rem;
        }
        .textAdjSml{
            font-size: 0.65rem;
            font-style: italic;
            color: rgb(185, 185, 185);
        }     
    }
    .textAdjSml{
            font-size: 0.8rem;
            font-style: italic;
            color: rgb(185, 185, 185);
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
        background-color: rgba(169, 169, 169, 0.5);
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