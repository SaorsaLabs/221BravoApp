<script>
    
</script>
<header class="head">
    <div class="container-lg">
        <div class="row">
            <div class="col alignCentre">
                <slot></slot>
            </div>
        </div>
    </div>
</header>

<style>
    .head{
        flex: auto;
        background-color: rgb(81, 81, 81);
         min-height: 32px;  /*keep row the same*/
         min-width: 700px;
         border: 0;
         border-bottom: 1px;
         border-color: rgba(255, 255, 255, 0.8);
         border-style: solid;
    }
    .row{
        min-height: 30px;
    }
    .alignCentre{
        text-align: center;
    }
</style>