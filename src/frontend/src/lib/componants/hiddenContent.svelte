<script>
    import Button from "../shared/button.svelte";
</script>

<div class="box">
<p class="text">Premium Content:</p>
    <slot></slot>
<p class="text"><a></a></p>
<div class="buttonMargin">
    <a  href="/members">
        <Button type="orange" 
        flat={true} 
        noBG={true}
        slim={true}
        >
        Get Membership
        </Button> 
        </a>
</div>

</div>
<style>
    .box{
        border: 2px;
        border-radius: 5px;
        border-style: dotted;
        border-color: #dba604;
        width: 100%;
        height: 100%;
        text-align: center;
    }
    .text{ 
        color: #dba604;
        padding:0;
        margin:0;
    }
    .buttonMargin{
        margin: 5px;
    }
</style>