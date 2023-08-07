<script>
    import { onMount } from 'svelte';
    import { browser } from '$app/environment';

    // CANVAS
    let canvas;
    let cvsHolder = 0;
    let ctx;
    var clientWidth = 0;
    var clientHeight = 0;
    var initWidth = 0;
    var initHeight = 0;
    var stopped = false;

    // DATA
    let startDraw = false;
    let data = {
        transactions: [
            {
                startX: 50,
                startY: 50,
                endX: 85,
                endY: 25,
                token: 1
            },
        ],
        globalData: {
            canvasWidth: 0,
            canvasHeight: 0,
            globalZoom: 1,
            inX: 1,
            inY: 1,
            globalMoveX: 0,
            globalMoveY: 0
        }
        };
    onMount(() => {
        init();
	});

    function createData(){
        data.transactions = [];
        let i;
        let r1, r2, r3, r4;
        let res = [];
        let xx = 5;
        let yy = 5;
        for(i=0; i<150; i++){
            // if(xx>85){
            //     xx=5;
            //     yy+=15;
            // }
            r1 = Math.floor(Math.random() * 85)+5;
            r2 = Math.floor(Math.random() * 85)+5;
            r3 = Math.floor(Math.random() * 85)+5; //xx;//
            r4 = Math.floor(Math.random() * 85)+5; //yy;//
            res.push({          
                startX: r1,
                startY: r2,
                endX: r3,
                endY: r4,
                token: tokenSelection
            });
          //  xx+=1.92;
        }
        data.transactions = res;
    }

    function init(){
        ctx = canvas.getContext('2d');
        canvas.width = cvsHolder.clientWidth;
        canvas.height = cvsHolder.clientHeight;
        clientWidth = canvas.width;
        clientHeight = canvas.height;
        initWidth = cvsHolder.clientWidth;
        initHeight = cvsHolder.clientHeight;
        data.globalData.canvasWidth = clientWidth;
        data.globalData.canvasHeight = clientHeight;
        startDraw = false;
        stopped = false;
        c1Dir=1;
        c2Dir=1;
        c3Dir=1;
        animate();
    }
    if(browser){
        window.addEventListener("resize", function(){resize();}, true);
    }
    function resize(){
        totAddedTx = 0;
        canvas.width = cvsHolder.clientWidth;
        canvas.height = cvsHolder.clientHeight;
        clientWidth = canvas.width;
        clientHeight = canvas.height;
        initWidth = cvsHolder.clientWidth;
        initHeight = cvsHolder.clientHeight;
        data.globalData.canvasWidth = clientWidth;
        data.globalData.canvasHeight = clientHeight;
        startDraw = false;
    }
    // animate settings
    let count = 0;
    let textColourTX =0;
    let totAddedTx = 0;
    let showText = true;
    let c1 = 255;
    let c2 = 255;
    let c3 = 255;
    let c1Dir, c2Dir, c3Dir;
    let i;

    //SPEED SETTINGS
    let textColour = 1;

    // main loop
    function animate(){
        ctx.clearRect(0, 0, clientWidth, clientHeight);
        // resize
        // if(canvas.width != initWidth || canvas.height != initHeight){
            
        // } 
    
        // text colour
        if(textColourTX>textColour){
            if(c1 >=220) c1Dir = -1;
            if(c1 <=0) c1Dir = 1;
            if(c1Dir == 1){
                c1+= 15.5;
            }
            if(c1Dir == -1){
                c1-= 1;
            }

            if(c2 >=220) c2Dir = -1;
            if(c2 <=170) c2Dir = 1;
            if(c2Dir == 1){
                c2+= 0.25;
            }
            if(c2Dir == -1){
                c2-= 0.25;
            }

            if(c3 >=220) c3Dir = -1;
            if(c3 <=150) c3Dir = 1;
            if(c3Dir == 1){
                c3+= 1;
            }
            if(c3Dir == -1){
                c3-= 1;
            }
            textColourTX = 0;
        }
        count++;
        textColourTX++;
        if(showText){
            let tSize = (clientWidth/100)*5;
            ctx.font = `${tSize}px Verdana`;
            var txt = "Welcome Data Detective";
            let txW = ctx.measureText(txt).width;
            ctx.shadowOffsetX = 3;
            ctx.shadowOffsetY = 3;
            ctx.shadowColor = "rgba(0,0,0,0.5)";
            ctx.shadowBlur = 4;
            ctx.fillStyle = `rgba(${c1},${c2},${c3},1)`; //'rgba(250,255,250,1)';// 
            ctx.fillText(txt, (clientWidth/2)-(txW/2), (clientHeight*.10)+(tSize/4)); 
            //ctx.fillText(txt, (clientWidth/2)-(txW/2), (clientHeight/2)+(tSize/4)); CENTRE
            ctx.shadowColor = "transparent";
        }
        if(!stopped) requestAnimationFrame(animate);
    }

</script>
<!-- <svelte:window bind:innerWidth bind:innerHeight /> -->

<div bind:this={cvsHolder} class="canvasHolder">
    <canvas class="cvs" bind:this={canvas} ></canvas>
</div>

<style>
    .canvasHolder{
        height: 80vh;
        width: 100%;
    }
    .cvs{
        /* background-color: black; */
        width:100%;
        height: 100%;
    }

    /* canvas {
		width: 100%;
		height: 100%;
	} */

</style>