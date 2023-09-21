<script>
            // USAGE 
            // <DoubleContentBox>
			// 	    <span slot="leftContent"> Content Here </span>
			// 	    <span slot="rightContent"> Content Here </span>
			// </DoubleContentBox>

    export let type = "standard-shaddow-dark";
    export let hidden = false;

    let t1, t2; 
    if (type == "standard"){
        t1 = "standard-left";
        t2 = "standard-right";
    }
    if (type == "standard-dotborder"){
        t1 = "standard-dotborder-left";
        t2 = "standard-dotborder-right";
    }
    if (type == "standard-shaddow-dark"){
        t1 = "standard-shaddow-dark-left";
        t2 = "standard-shaddow-dark-right";
    }
    if (type == "light-shaddow-dark"){
        t1 = "light-shaddow-dark-left";
        t2 = "light-shaddow-dark-right";
    }
    if (type == "standard-shaddow-black"){
        t1 = "standard-shaddow-black-left";
        t2 = "standard-shaddow-black-right";
    }
    if (type == "standard-shaddow-black"){
        t1 = "standard-shaddow-black-left";
        t2 = "standard-shaddow-black-right";
    }
    if (type == "standard-shaddow-dark-padding"){
        t1 = "standard-shaddow-dark-padding-left";
        t2 = "standard-shaddow-dark-padding-right";
    }
    if (type == "standard-shaddow-light"){
        t1 = "standard-shaddow-light-left";
        t2 = "standard-shaddow-light-right";
    }
</script>
<div class="div-container">
    <div class={t1} class:hidden={hidden}>
        <slot name="leftContent"></slot>
    </div>
    <div class={t2} class:hidden={hidden}>
        <slot name="rightContent"></slot>
    </div>
</div>
<style>
    .div-container {
        display: flex;
        flex-wrap: wrap;
        /* justify-content: space-around; */
        width: 100%;
    }
    .standard-left{
        margin-top: 10px;
        padding-left: 5px;
        padding-right: 5px;
        background-color: rgba(49, 49, 49, 0.5);
        border-radius: 5px;
        color: azure;
        width:49.5%;
        margin-right: 0.5%;
    }
    .standard-right{
        margin-top: 10px;
        padding-left: 5px;
        padding-right: 5px;
        background-color: rgba(49, 49, 49, 0.5);
        border-radius: 5px;
        color: azure;
        width:49.5%;
        margin-left: 0.5%;
    }
    .standard-dotborder-left{
        color: azure;
        margin-top: 10px;
        padding-left: 5px;
        padding-right: 5px;
        background-color: rgba(49, 49, 49, 0.5);
        border-radius: 5px;
        border: 1px;
        border-style: dotted;
        border-color: #b2b37f;
        width:49.5%;
        margin-right: 0.5%;
    }
    .standard-dotborder-right{
        color: azure;
        margin-top: 10px;
        padding-left: 5px;
        padding-right: 5px;
        background-color: rgba(49, 49, 49, 0.5);
        border-radius: 5px;
        border: 1px;
        border-style: dotted;
        border-color: #b2b37f;
        width:49.5%;
        margin-left: 0.5%;
    }
    .standard-shaddow-dark-left{
        color: azure;
        margin-top: 10px;
        padding-left: 5px;
        padding-right: 5px;
        border-radius: 5px;
        background-color: rgba(49, 49, 49, 0.5);
        box-shadow: 0px 0px 3px rgba(0,0,0,0.3);
        width:49.5%;
        margin-right: 0.5%;
    }
    .standard-shaddow-dark-right{
        color: azure;
        margin-top: 10px;
        padding-left: 5px;
        padding-right: 5px;
        border-radius: 5px;
        background-color: rgba(49, 49, 49, 0.5);
        box-shadow: 0px 0px 3px rgba(0,0,0,0.3);
        width:49.5%;
        margin-left: 0.5%;
    }
    .light-shaddow-dark-left{
        color: azure;
        margin-top: 10px;
        padding-left: 5px;
        padding-right: 5px;
        border-radius: 5px;
        background-color: rgba(49, 49, 49, 0.05);
        box-shadow: 0px 0px 3px rgba(0,0,0,0.5);
        width:49.5%;
        margin-right: 0.5%;
    }
    .light-shaddow-dark-right{
        color: azure;
        margin-top: 10px;
        padding-left: 5px;
        padding-right: 5px;
        border-radius: 5px;
        background-color: rgba(49, 49, 49, 0.05);
        box-shadow: 0px 0px 3px rgba(0,0,0,0.5);
        width:49.5%;
        margin-left: 0.5%;
    }
    .standard-shaddow-black-left{
        color: azure;
        margin-top: 10px;
        padding-left: 5px;
        padding-right: 5px;
        border-radius: 5px;
        background-color: rgba(0, 0, 0, 0.66);
        box-shadow: 0px 0px 3px rgba(0,0,0,0.3);
        width:49.5%;
        margin-right: 0.5%;
    }
    .standard-shaddow-black-right{
        color: azure;
        margin-top: 10px;
        padding-left: 5px;
        padding-right: 5px;
        border-radius: 5px;
        background-color: rgba(0, 0, 0, 0.66);
        box-shadow: 0px 0px 3px rgba(0,0,0,0.3);
        width:49.5%;
        margin-left: 0.5%;
    }
    .standard-shaddow-dark-padding-left{
        color: azure;
        margin-top: 10px;
        padding-left: 5px;
        padding-right: 5px;
        border-radius: 5px;
        background-color: rgba(49, 49, 49, 0.5);
        box-shadow: 0px 0px 3px rgba(0,0,0,0.3);
        padding: 5px;
        width:49.5%;
        margin-right: 0.5%;
    }
    .standard-shaddow-dark-padding-right{
        color: azure;
        margin-top: 10px;
        padding-left: 5px;
        padding-right: 5px;
        border-radius: 5px;
        background-color: rgba(49, 49, 49, 0.5);
        box-shadow: 0px 0px 3px rgba(0,0,0,0.3);
        padding: 5px;
        width:49.5%;
        margin-left: 0.5%;
    }
    .standard-shaddow-light-left{
        color: azure;
        margin-top: 10px;
        padding-left: 5px;
        padding-right: 5px;
        border-radius: 5px;
        background-color: rgba(49, 49, 49, 0.5);
        box-shadow: 0px 0px 3px rgba(236, 236, 236, 1);
        width:49.5%;
        margin-right: 0.5%;
    }
    .standard-shaddow-light-right{
        color: azure;
        margin-top: 10px;
        padding-left: 5px;
        padding-right: 5px;
        border-radius: 5px;
        background-color: rgba(49, 49, 49, 0.5);
        box-shadow: 0px 0px 3px rgba(236, 236, 236, 1);
        width:49.5%;
        margin-left: 0.5%;
    }
    .hidden{
        visibility: hidden;
    }
</style>