<script>
    export let type = "standard-shaddow-dark";
    export let hidden = false;
</script>
<div class={type} class:hidden={hidden}>
    <slot></slot>
</div>
<style>
    .standard{
        margin-top: 10px;
        padding-left: 5px;
        padding-right: 5px;
        background-color: rgba(49, 49, 49, 0.5);
        border-radius: 5px;
        color: azure;
    }
    .standard-dotborder{
        color: azure;
        margin-top: 10px;
        padding-left: 5px;
        padding-right: 5px;
        background-color: rgba(49, 49, 49, 0.5);
        border-radius: 5px;
        border: 1px;
        border-style: dotted;
        border-color: #b2b37f;
    }
    .standard-shaddow-dark{
        color: azure;
        margin-top: 10px;
        padding-left: 5px;
        padding-right: 5px;
        border-radius: 5px;
        background-color: rgba(49, 49, 49, 0.5);
        box-shadow: 0px 0px 3px rgba(0,0,0,0.3);
    }
    .standard-shaddow-black{
        color: azure;
        margin-top: 10px;
        padding-left: 5px;
        padding-right: 5px;
        border-radius: 5px;
        background-color: rgba(0, 0, 0, 0.66);
        box-shadow: 0px 0px 3px rgba(0,0,0,0.3);
    }
    .standard-shaddow-dark-padding{
        color: azure;
        margin-top: 10px;
        padding-left: 5px;
        padding-right: 5px;
        border-radius: 5px;
        background-color: rgba(49, 49, 49, 0.5);
        box-shadow: 0px 0px 3px rgba(0,0,0,0.3);
        padding: 5px;
    }
    .standard-shaddow-light{
        color: azure;
        margin-top: 10px;
        padding-left: 5px;
        padding-right: 5px;
        border-radius: 5px;
        background-color: rgba(49, 49, 49, 0.5);
        box-shadow: 0px 0px 3px rgba(236, 236, 236, 1);
    }
    .hidden{
        visibility: hidden;
    }
</style>