<script>
    import { onMount } from 'svelte';
    import { createBlueprints } from '../../code/txArtistAnimated.js';

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
    let workbasket = [];
    let drawArray = [];
    let blueprints; 
    let bpComplete = false;
    let bpLen = 0;
    let startDraw = false;
    let sections = 60; // change in txArtistAnimated as well. 
    let tokenSelection = 2;
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
        for(i=0; i<250; i++){
            r1 = Math.floor(Math.random() * 85)+5;
            r2 = Math.floor(Math.random() * 85)+5;
            r3 = Math.floor(Math.random() * 85)+5;
            r4 = Math.floor(Math.random() * 85)+5;
            res.push({          
                startX: 50,
                startY: 50,
                endX: r3,
                endY: r4,
                token: tokenSelection
            });
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

    function resize(){
        workbasket = [];
        drawArray = [];
        expandTX = 0;
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
    }

    function populateDrawArray(){
        let i,k;
        let wbLen = workbasket?.length ?? 0;
        let drawnTo; 
        if(wbLen > 0){
            for(i=0; i<wbLen; i++){
                drawnTo = workbasket[i].completed;
                if(drawnTo<sections){
                    drawArray.push(workbasket[i].nodeArray[drawnTo]);
                    workbasket[i].completed++;
                }
            }
        startDraw = true;
        }
    }

    // mutate settings
    let pruneCounter = 0;
    let alphaReduce = 0.045;
    let sizeReduce = 5;
    let pruneTrigger = 50;

    function mutateAndPrune(){
        let i, daLen;
        
        daLen = drawArray?.length ?? 0;
        let alpha, size;
        let smAlpha = 999999999;
        // mutate
        for(i=0; i<daLen; i++){
            // alpha
            alpha = drawArray[i].data[0].colour[3];
           
            //if(alpha < smAlpha) smAlpha = alpha;
           // console.log(smAlpha);
            if(alpha>= alphaReduce && alpha>0) drawArray[i].data[0].colour[3] -= alphaReduce;
            
            //size
            size = drawArray[i].data[0].radius;
            if(size>((size/100)*sizeReduce)) drawArray[i].data[0].radius -= ((size/100)*sizeReduce);

            // "x" : CRV[i].x,
            // "y" : CRV[i].y,
            // "start" : 0,
            // "end" : Math.PI * 2,
            // "radius" : size*10,
            // "colour" : [currentColour[0],currentColour[1],currentColour[2],currentColour[3]],
            // "reverse" : false
        }

        // prune 
        if(pruneCounter >= pruneTrigger){
            let maxZero = 0;
            for(i=0; i<daLen; i++){
                if(drawArray[i].data[0].colour[3] <= alphaReduce/2){
                    if(i>maxZero) maxZero = i;
                }
            }
            drawArray.splice(0, maxZero-250);
            pruneCounter = 0;
        }

        pruneCounter++;
    }

    // animate settings
    let count = 0;
    let addTX = 0;
    let expandTX = 0;
    let mutateTX = 0;
    let textColourTX =0;
    let totAddedTx = 0;
    let drawTX = 0;
    let daLen = 0;
    let showText = false;
    let c1 = 255;
    let c2 = 255;
    let c3 = 255;
    let c1Dir, c2Dir, c3Dir;
    let i;

    //SPEED SETTINGS
    let addNewTransaction = 5;
    let expandSection = 2;
    let mutate = 5;
    let textColour = 2;

    function animate(){
        ctx.clearRect(0, 0, clientWidth, clientHeight);

        // resize
        if(cvsHolder.clientWidth != initWidth || cvsHolder.clientHeight != initHeight){
            resize();
        } 

        // add TX
        if(addTX>=addNewTransaction){
            if(totAddedTx<bpLen){
                workbasket.push(blueprints[totAddedTx]);
                totAddedTx++;
                if(totAddedTx >= bpLen*0.025) showText = true;
                if(totAddedTx >= bpLen-2) {
                    // bpComplete = true;
                    console.log("Fetching More Data!");
                    tokenSelection = 2;
                    createData();
                    workbasket = [];
                    blueprints = createBlueprints(data);
                    bpLen = blueprints?.length ?? 0;
                    populateDrawArray();
                    expandTX = 0;
                    totAddedTx = 0;
                }
            }
            addTX = 0;
        }

        // expand
        if(expandTX>=expandSection){
            populateDrawArray();
            expandTX = 0;
        }
        
        // draw
        if(startDraw){
            daLen = drawArray.length ?? 0;
            for(i=0; i<daLen; i++){
                drawCircle(drawArray[i].data[0]);
            }
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
                c1+= 2.5;
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
        expandTX++;
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
    function drawCircle(settings){

        // load settings
        let size = settings?.radius ?? 3;
        let x = settings?.x ?? -1;
        let y = settings?.y ?? -1;
        let start = settings?.start ?? 0;
        let end = settings?.end ?? Math.PI * 2;
        let [R,G,B,A] = settings?.colour ?? [255,255,255,1];
        ctx.beginPath();
        ctx.arc(x, y, size, start, end, false);
        ctx.fillStyle = `rgba(${R},${G},${B},${A})`;
        ctx.fill();
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