<script>
    import { onMount } from 'svelte';
    import { createBlueprints } from '../code/txArtistUtilsHome.js';
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
    let drawArray = [];
    let blueprints; 
    let bpLen = 0;
    let startDraw = false;
    let tokenSelection = 2; // test data
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
        createData();// create dummy data
        ctx = canvas.getContext('2d');
        canvas.width = cvsHolder.clientWidth;
        canvas.height = cvsHolder.clientHeight;
        clientWidth = canvas.width;
        clientHeight = canvas.height;
        initWidth = cvsHolder.clientWidth;
        initHeight = cvsHolder.clientHeight;
        data.globalData.canvasWidth = clientWidth;
        data.globalData.canvasHeight = clientHeight;
        blueprints = createBlueprints(data);
        bpLen = blueprints?.length ?? 0;
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
        drawArray = [];
        totAddedTx = 0;
        canvas.width = cvsHolder.clientWidth;
        canvas.height = cvsHolder.clientHeight;
        clientWidth = canvas.width;
        clientHeight = canvas.height;
        initWidth = cvsHolder.clientWidth;
        initHeight = cvsHolder.clientHeight;
        data.globalData.canvasWidth = clientWidth;
        data.globalData.canvasHeight = clientHeight;
        blueprints = createBlueprints(data);
        bpLen = blueprints?.length ?? 0;
        startDraw = false;
        populateDrawArray(blueprints);
    }

    // draw aray settings
    let addedTo = 0;
    function populateDrawArray(blueprints){
        let bpLen = blueprints?.length ?? 0;
        if(bpLen > 0 && addedTo < bpLen){
            drawArray.push(blueprints[addedTo]);
            addedTo++;
        }
        startDraw = true;
    }

    // animate settings
    let count = 0;
    let addTX = 0;
    let mutateTX = 0;
    let textColourTX =0;
    let totAddedTx = 0;
    let drawTX = 0;
    let daLen = 0;
    let showText = true;
    let c1 = 255;
    let c2 = 255;
    let c3 = 255;
    let c1Dir, c2Dir, c3Dir;
    let i;

    //SPEED SETTINGS
    let addNewTransaction = 2;
    let textColour = 1;
    let mutate = 2;

    // Mutate Settings
    let alphaChange = 0.05; 
    let maxAlpha = 0.4;

    function mutateAndPrune(){
        let i, daLen;
        daLen = drawArray?.length ?? 0;
        let alpha;
        let stop = true;
        for(i=0; i<daLen; i++){
            // alpha
            alpha = drawArray[i].nodeArray[0].data[0].colour[3];
            if(alpha < maxAlpha) {
                stop = false;
                drawArray[i].nodeArray[0].data[0].colour[3] += alphaChange;
                drawArray[i].nodeArray[1].data[0].colour[3] += alphaChange;
                drawArray[i].nodeArray[2].data[0].colour[3] += alphaChange;
            }
        }
     //   if(stop) startDraw = false;
    }


    // main loop
    function animate(){
        ctx.clearRect(0, 0, clientWidth, clientHeight);
        // resize
        // if(canvas.width != initWidth || canvas.height != initHeight){
            
        // } 
        // add TX (Speed adjustable)
        if(addTX>=addNewTransaction ){
            populateDrawArray(blueprints);
            addTX = 0;
        }
        // draw
        if(startDraw){
            
            daLen = drawArray.length ?? 0;
            paintbrush(drawArray);
            drawTX = 0;
        }
        // mutate
        if(mutateTX>=mutate){
            mutateAndPrune();
            mutateTX = 0;
        }
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
        addTX++;
        drawTX++;
        mutateTX++;
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
            ctx.fillText(txt, (clientWidth/2)-(txW/2), (clientHeight/2)+(tSize/4)); 
            ctx.shadowColor = "transparent";
        }
        if(!stopped) requestAnimationFrame(animate);
    }


    // UTILS for animate

    function paintbrush(blueprints){
        let i,k;
        let bpLen = blueprints?.length ?? 0;
        let bpSections = 0;
        let size, sX, sY, eX, eY, R,G,B,A;
        let start = 0; 
        let end = Math.PI * 2;
        for(i=0;i<bpLen; i++){
            
            bpSections = blueprints[i].sections;
            for(k=0; k<bpSections; k++){
                // draw types
                if(blueprints[i].nodeArray[k].drawType == 'line'){
                    size = blueprints[i].nodeArray[k].data[0].size;
                    sX = blueprints[i].nodeArray[k].data[0].sX;
                    sY = blueprints[i].nodeArray[k].data[0].sY;
                    eX = blueprints[i].nodeArray[k].data[0].eX;
                    eY = blueprints[i].nodeArray[k].data[0].eY;
                    [R,G,B,A] = blueprints[i].nodeArray[k].data[0].colour ?? [255,255,255,255];
                    ctx.lineWidth = size;
                    ctx.strokeStyle = `rgba(${R},${G},${B},${A})`;
                    ctx.beginPath();
                    ctx.moveTo(sX, sY);
                    ctx.lineTo(eX, eY);
                    ctx.stroke();
                }
                else if(blueprints[i].nodeArray[k].drawType == 'circle'){
                    size = blueprints[i].nodeArray[k].data[0].size*2;
                    sX = blueprints[i].nodeArray[k].data[0].x;
                    sY = blueprints[i].nodeArray[k].data[0].y;
                    [R,G,B,A] = blueprints[i].nodeArray[k].data[0].colour ?? [255,255,255,255];
                    ctx.beginPath();
                    ctx.arc(sX, sY, size, start, end, false);
                    ctx.fillStyle = `rgba(${R},${G},${B},${A})`;
                    ctx.fill();
                }                
            }
        }

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