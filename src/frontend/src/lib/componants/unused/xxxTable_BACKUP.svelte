<script>
	import { personData } from '../code/personData.js';
    import { onMount } from 'svelte';
	import Pagination from '../../shared/pagination.svelte.js';
	
    let sortedPersonData = personData;
	const tableHeaders = ['#','Account','Type','Direction','Value','More'];//Object.keys(personData[0])
	let selectedHeader = "id";
	let ascendingOrder = true;
	
	// SORT BY NUMBER
	const sortByNumber = (colHeader) => {
		sortedPersonData = sortedPersonData.sort((obj1, obj2) => {
			return ascendingOrder ? Number(obj1[colHeader]) - Number(obj2[colHeader])
			: Number(obj2[colHeader]) - Number(obj1[colHeader])
		});
		selectedHeader = colHeader;
        paginate(sortedPersonData);
	}
	
	// SORT BY STRINGs
	const sortByString = (colHeader) => {
		sortedPersonData = sortedPersonData.sort((obj1, obj2) => {
			if (obj1[colHeader] < obj2[colHeader]) {
					return -1;
			} else if (obj1[colHeader] > obj2[colHeader]) {
				return 1;
			}
			return 0; //string code values are equal		
		});
		if (!ascendingOrder) {
			sortedPersonData = sortedPersonData.reverse()
		}
		selectedHeader = colHeader;
        paginate(sortedPersonData);
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

    //console.log("paginatedItems are", paginatedItems);
    totalPages = [...paginatedItems];
  };

    function pageChange(event){
        page = event.detail.page-1;
    }

    onMount(async () => {
        paginate(sortedPersonData);
	});
</script>

<div class="mainAlign">
    <div class="radioPadding">
        Per Page:
        <label>
            <input type=radio bind:group={itemsPerPage} value={10} on:change={() => paginate(sortedPersonData)}>
            10
        </label>
        
        <label>
            <input type=radio bind:group={itemsPerPage} value={25} on:change={() => paginate(sortedPersonData)}>
            25
        </label>
        
        <label>
            <input type=radio bind:group={itemsPerPage} value={50} on:change={() => paginate(sortedPersonData)}>
            50
        </label>
        <label>
            <input type=radio bind:group={itemsPerPage} value={100} on:change={() => paginate(sortedPersonData)}>
            100
        </label>
    </div>
    <table id="myTable">
    <tr>
            {#each tableHeaders as header}
            <th class:highlighted={selectedHeader === header}
                        on:click={() => (header === "id" || header === "age" ) ? sortByNumber(header) : sortByString(header)}>
                    {header.replace("_", " ")}

                {#if header === selectedHeader}	
                    <!-- svelte-ignore a11y-click-events-have-key-events -->
                    <span class="order-icon" on:click={() => ascendingOrder = !ascendingOrder}>
                        {@html ascendingOrder ? "&#9661;" : "&#9651;"}
                    </span>		
                {/if}	
            </th>	
            {/each}
    </tr>
        
<!--   ||  sortedPersonData -->
        {#each currentPageRows as person}
            <tr>
                <td>{person.id}</td>
                <td>{person.first_name}</td>
                <td>{person.last_name}</td>
                <td>{person.age}</td>
                <td>{person.job_title}</td>
                <td>{person.country}</td>
            </tr>	
        {/each}
    </table>
    <Pagination max={maxPage} on:click={pageChange}/>
</div>

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
		/* border: 1px;
        border-style: dotted;
        border-color: #595959;        */
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
		padding: 16px;
	}

	tr:nth-child(even) {
		background-color: rgba(166, 163, 107, 0.16)
	}
</style>