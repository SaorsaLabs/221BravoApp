<script>
    import Button from "../shared/button.svelte";
    import Loading from "../shared/loading.svelte";
    import { submitFraudReport } from '../code/searchRequest_v2.js';

    let reportedAC;
    let reportedCircs;
    let reportedURL; 
    let contactDetails;
    let infoBoxMode = 0;
    let submitting = false;

    async function submitInput() {
        submitting = true;
        let res = await submitFraudReport(reportedAC, reportedCircs, reportedURL, contactDetails);
        if (res == "Fraud Report Added"){
            infoBoxMode = 1;
        } else {
            infoBoxMode = -1;
        }
        submitting = false;
    }
    function clearInput(){
        infoBoxMode = 0;
        reportedAC = undefined;
        reportedCircs = undefined;
        reportedURL = undefined;
        contactDetails = undefined;
    }

</script>
<div style="width: 100%; padding: 10px; padding-top:15px">

    <div class="form-group row">
        <label for="ReportedAccount" class="col-sm-3 col-form-label-sm">Reported Account:</label>
        <div class="col-sm-9">
          <input 
            id="ReportedAccount" 
            class="form-control form-control-sm" 
            alt="Reported Account" 
            placeholder="ICP or ICRC Account.  Note ICRC format = Principal.Subaccount"
            type="text" 
            bind:value={reportedAC}
          />
        </div>
    </div>

    <div class="form-group row" style="padding-top: 15px;">
        <label for="Circs" class="col-sm-3 col-form-label-sm">What Happened?:</label>
        <div class="col-sm-9">
            <textarea
            id="Circs"
            class="form-control form-control-sm"
            alt="Reported Account"
            placeholder="Circumstances - Explain why you think the account is involved in a fraud or scam. Share any evidence here."
            bind:value={reportedCircs}
            rows="4"
        />
        </div>
    </div>

    <div class="form-group row" style="padding-top: 15px">
        <label for="EvidenceURL" class="col-sm-3 col-form-label-sm">URL Link (if any):</label>
        <div class="col-sm-9">
          <input 
            id="EvidenceURL" 
            class="form-control form-control-sm" 
            alt="Reported Account" 
            placeholder="Any URL Link"
            type="text" 
            bind:value={reportedURL}
          />
        </div>
    </div>

    <div class="form-group row" style="padding-top: 15px;">
        <label for="EvidenceURL" class="col-sm-3 col-form-label-sm">Contact Details :</label>
        <div class="col-sm-9">
          <input 
            id="EvidenceURL" 
            class="form-control form-control-sm" 
            alt="Reported Account" 
            placeholder="Best method of contact - Email address, twitter handle etc"
            type="text" 
            bind:value={contactDetails}
          />
        </div>
    </div>

    {#if infoBoxMode == -1}
        <div class="warnText" style="width: 100%; text-align:center">Error - cannot submit report at this time</div>
    {:else if infoBoxMode == 1}
        <div class="okText" style="width: 100%; text-align:center">Fraud Report has been submitted</div>
    {/if}
    <table style="width: 100%; margin-top:20px;">
        <tr>
        <td style="width: 90%;"></td>
        {#if submitting == false}
            <td style="text-align: right; padding-right:10px"> <Button type="green" on:click={() => {submitInput()}}>Submit</Button> </td>
            <td style="text-align: right; padding-right: 10px"> <Button type="grey" on:click={() => {clearInput()}}>Clear</Button> </td>
        {:else}
            <Loading/>
        {/if}
        </tr>
    </table>
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
	.width100{
		width: 100%;
	}
	.cText{
		text-align: center;
	}
    .warnText{
        color:white;
        background-color: red; 
        text-align: center;
        margin-top: 5px;
        border-radius: 5px;
    }
    .okText{
        color:white;
        background-color: rgb(130, 234, 26); 
        text-align: center;
        margin-top: 5px;
        border-radius: 5px;
    }
</style>