<script>
import { browser } from '$app/environment';
import { onMount } from 'svelte';
import { Chart, registerables } from 'chart.js';


export let dataArray1;
export let dataArray2;
export let dataArray3;
export let dataArray4;
export let labelsArray;
export let dataset1Title = "";
export let dataset2Title = "";
export let reverse = false;

Chart.register(...registerables);
let lineChartElement;

   onMount(() => {
     if (browser) {
     
      if (reverse){
        if (dataArray1) dataArray1.reverse();
        if (dataArray2) dataArray2.reverse();
        if (dataArray3) dataArray3.reverse();
        if (dataArray4) dataArray4.reverse();
        if (labelsArray) labelsArray.reverse();
      }

      let DS = [];
        if (dataArray1) {
          DS.push({
            label: dataset1Title,
            data: dataArray1,
            fill: false,
            borderColor: 'rgb(75, 192, 192)',
            tension: 0.1
          });
        }
        if (dataArray2) {
          DS.push({
            label: dataset2Title,
            data: dataArray2,
            fill: false,
            borderColor: 'rgb(134, 52, 145)',
            tension: 0.1
          });
        }
        if (dataArray3) {
          DS.push({
            label: dataset2Title,
            data: dataArray3,
            fill: false,
            borderColor: 'rgb(95, 100, 176)',
            tension: 0.1
          });
        }
        if (dataArray4) {
          DS.push({
            label: dataset2Title,
            data: dataArray4,
            fill: false,
            borderColor: 'rgb(10, 203, 241)',
            tension: 0.1
          });
        }
        
        const labels = labelsArray;
        const data = {
          labels: labels,
          datasets: DS,
        };
       new Chart(lineChartElement, {
         type: 'line',
         data: data,
       });
     }
   });

</script>
<canvas bind:this={lineChartElement} />
<style>

</style>