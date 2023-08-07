<script>
    import { millisToDate, millisToTime } from '../code/utils.js';

    let sentValue = 0;
    let receivedValue = 0;
    let balance = 0;
    let sentTX = 0;
    let receivedTX = 0;
    let totalTX = 0;
    let activeFrom = 0;
    let lastActive = 0;
    let afDate = "Date";
    let laDate = "Date";
    let afTime = "Time";
    let laTime = "Time";
    const fmtOptionsValue = { style: 'decimal', maximumFractionDigits: 8, minimumFractionDigits: 0 };

    export let token = 'Token';
    export let data = {};

    // -- DATA Should have the following
    // { 
    //     sentValue,
    //     receivedValue,
    //     balance: 
    //     sentTX,
    //     receivedTX,
    //     activeFrom,
    //     lastActive
    // };

    let sV, rV, bA;
    sV = data.sentValue.toLocaleString('en-US', fmtOptionsValue);
    rV = data.receivedValue.toLocaleString('en-US', fmtOptionsValue);
    bA = data.balance.toLocaleString('en-US', fmtOptionsValue);
    
    $: laDate = millisToDate(data.lastActive);
    $: laTime = millisToTime(data.lastActive);
    $: afDate = millisToDate(data.activeFrom);
    $: afTime = millisToTime(data.activeFrom);
    $: totalTX = data.sentTX + data.receivedTX;
</script>

<div class="row mainAlign">
    <div class="col-sm-12 col-md-7">
      <div class="row">
        <div class="col cText">
          <p>{token} Sent</p>
          <h3 class="textC3">{sV}</h3>
          <p class="textC3">{data.sentTX} TXs</p>
        </div>
        <div class="col cText">
          <p>{token} Received</p>
          <h3 class="textC2">{rV}</h3>
          <p class="textC2">{data.receivedTX} TXs</p>
        </div>
        <div class="col cText">
          <p>{token} Balance</p>
          <h3 class="textC1">{bA}</h3>
          <p class="textC1">Total: {totalTX} TXs</p>
        </div>
      </div>
    </div>
    <div class="col-sm-12 col-md-5 OverviewBox">
      <div class="row cText">
        <div class="col">
          <p>Active From</p>
          <h4>{afDate}</h4>
          <p>{afTime}</p>
        </div>
        <div class="col cText">
          <p>Last Active</p>
          <h4>{laDate}</h4>
          <p>{laTime}</p>
        </div>
      </div>
    </div>
</div>

<style>
	.mainAlign{
		padding: 10px;
	}
	.smlPadTB{
		padding-top: 3px;
		padding-bottom: 3px;
	}
	.box{
		border: 2px;
		border-style: dashed;
		border-color: aliceblue;
	}
    .cText{
		text-align: center;
	}
    .textC1{
        color:aqua;
    }
    .textC2{
        color:rgb(0, 195, 0);
    }
    .textC3{
        color:red;
    }
</style>