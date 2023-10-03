<script>
import LineChart from "../charts/lineChart.svelte";
export let chartLines = 1; // 1= , 2= 
export let dataArray1 = [];
export let dataArray2 = [];
export let dataArray3 = [];
export let dataArray4 = [];
export let labelsArray = [];
export let colours = [];
export let title = "";
export let reverse = true;

if (reverse == true ){
        if (dataArray1) dataArray1.reverse();
        if (dataArray2) dataArray2.reverse();
        if (dataArray3) dataArray3.reverse();
        if (dataArray4) dataArray4.reverse();
        if (labelsArray) labelsArray.reverse();
}

</script>

{#if chartLines == 1}
    <LineChart 
        dataArray1={dataArray1} 
        labelsArray={labelsArray}
        dataset1Title={title}
        colours={colours}
    />
{:else if chartLines ==2}
    <LineChart 
        dataArray1={dataArray1}
        dataArray2={dataArray2} 
        labelsArray={labelsArray}
        dataset1Title={title}
        colours={colours}
    />
{:else if chartLines ==3}
    <LineChart 
        dataArray1={dataArray1}
        dataArray2={dataArray2} 
        dataArray3={dataArray3} 
        labelsArray={labelsArray}
        dataset1Title={title}
        colours={colours}
    />
{:else if chartLines ==4}
    <LineChart 
        dataArray1={dataArray1}
        dataArray2={dataArray2}
        dataArray3={dataArray3} 
        dataArray4={dataArray4} 
        labelsArray={labelsArray}
        dataset1Title={title}
        colours={colours}
    />
{/if}

<style>

</style>