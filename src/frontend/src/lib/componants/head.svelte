<script>
    import logo from '$lib/images/221B_sml_beta.png';
    import HeadBtns from './headBTNS.svelte';

</script>

<header class="head">
    <div class="container-lg">
        <div class="row">
            <div class="col logoHolder">
               <a href="/"><img class="logo" src={logo} alt="221Bravo Logo" /></a>
            </div>
            <div class="col-1"></div>
            <div class="col buttonAlign">
                <HeadBtns/>
            </div>
        </div>
        
    </div>
    
</header>

<style>
    .head{
        flex: auto;
        background-color: rgb(0, 0, 0);
         height: 65px;  /*keep row the same*/
         min-width: 700px;
         border-top: 0;
         border-left: 0;
         border-right: 0;
         border-bottom: 1px;
         border-color: rgba(255, 255, 255, 0.8);
         border-style: solid;
         
    }
    .row{
        height: 61px;
    }
    .logo{
        height:53px;
        margin-top: 3px;
    }
    .box{
        border-color: aliceblue;
        border-style: dotted;
        border-width: 2px;
    }
    .logoHolder{
        min-width: inherit;
        max-width: fit-content;
    }
</style>