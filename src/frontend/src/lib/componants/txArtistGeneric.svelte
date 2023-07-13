<script>
    import { onMount } from 'svelte';
    import { createBlueprints } from '../code/txArtistUtils.js';
    import { browser } from '$app/environment';
    import { createEventDispatcher } from 'svelte';
    
    export let data = {};
     // EXAMPLE LAYOUT OF DATA
    // let data = {
    //     settings: [{
    //         token: "ICP",
    //         lineColour: [255,255,255,0.33],
    //         dotColour: [50,230,255,0.75],
    //         size: 1.5,  // pixels 
    //     }],
    //     transactions: [
    //         {
    //             startX: 50,
    //             startY: 50,
    //             endX: 85,
    //             endY: 25,
    //             token: "ICP"
    //         }, ... ETC
    //     ],
    //     globalData: {
    //         text: "ICP: Latest Transactions" 
    //         canvasWidth: 0,
    //         canvasHeight: 0,
    //         canvasBGColour: [0,0,0,0.666],
    //         globalZoom: 1,
    //         inX: 1,
    //         inY: 1,
    //         globalMoveX: 0,
    //         globalMoveY: 0
    //     }
    //     };

    const dispatch = createEventDispatcher();

    // CANVAS
    let canvas;
    let cvsHolder = 0;
    let ctx;
    var clientWidth = 0;
    var clientHeight = 0;
    var initWidth = 0;
    var initHeight = 0;
    var stopped = false;
    var maxZoom = 4;
    var minZoom = 0.5;
    var clickHighlightActive = false;
    var clickHighlight = {
        x: -10,
        y: -10,
        arrayRef: null,
        startOrEnd: null,
        clickedPR: null,
        clickedAC: null,
    }
    var referencePositions;

    
    //Mouse Move
    var isDown = false;        // mouse button is held down
    var firstPos;
    var lastPos;
    var buttonClick = false;
    var dragActive = false;

    // DATA
    let drawArray = [];
    let returnData = [];
    let blueprints; 
    let bpLen = 0;
    let startDraw = false;

    onMount(() => {
        init();
	});

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
        blueprints = createBlueprints(data);
        bpLen = blueprints?.length ?? 0;
        startDraw = false;
        populateOnce = false;
        stopped = false;
        mutateComplete = false;
        clickHighlightActive = false;
        returnData = [];
        animate();
    }

    // event listeners 
    if(browser){
        let isWithin = false;
        let tgt; 
        let dist = 0;

        window.addEventListener("resize", function(){resize();});
        window.addEventListener("mousemove", function(e) {userMove(e)});
        window.addEventListener("touchmove", function(e) {userMove(e)})
            function userMove(e){
            // isWithin = false;
            // tgt = e.target.className;
            // if(tgt.includes("cvs")) isWithin = true;

            if(isDown){
                lastPos = getMouesPosition(e);
                 if(isWithin){
                    stopped = false;
                    // move canvas
                    data.globalData.globalMoveX += (firstPos.x-lastPos.x);
                    data.globalData.globalMoveY += (firstPos.y-lastPos.y);
                    if((Math.abs(firstPos.x-lastPos.x)+Math.abs(firstPos.x-lastPos.x))>10) dragActive = true;
                }          
            }
        }

        window.addEventListener("touchstart", function(e) {
            tgt = e.target.className;
            if(tgt.includes("cvs")){
                userDown(e)
            }
        });
        window.addEventListener('mousedown',function(e) {
            tgt = e.target.className;
            if(tgt.includes("cvs")){
                userDown(e)
            }
        });

            function userDown(e){
            isWithin = false;
            tgt = e.target.className;
            if(tgt.includes("cvs")) isWithin = true;
            isDown = true;
            if(isWithin) firstPos = getMouesPosition(e);
            // zoom +
            if(
                isWithin &&
                firstPos.x >= clientWidth-50 && firstPos.x <= clientWidth-10 &&
                firstPos.y >= (clientHeight/2)-50 && firstPos.y <= (clientHeight/2)-10
                ){  
                buttonClick = true;
                if(data.globalData.globalZoom < (maxZoom-0.2)) {
                    data.globalData.globalZoom += 0.2;
                }
            }
            // zoom -
            else if(
                isWithin &&
                firstPos.x >= clientWidth-50 && firstPos.x <= clientWidth-10 &&
                firstPos.y >= (clientHeight/2) && firstPos.y <= (clientHeight/2)+40
                ){
                buttonClick = true;
                if(data.globalData.globalZoom > (minZoom+0.2)) {
                    data.globalData.globalZoom -= 0.2;
                }
            }
            // reset 
            else if(
                isWithin &&
                firstPos.x >= clientWidth-50 && firstPos.x <= clientWidth-10 &&
                firstPos.y >= (clientHeight/2)+50 && firstPos.y <= (clientHeight/2)+90
                ){
                    buttonClick = true;
                    data.globalData.globalZoom  = 1;
                    data.globalData.globalMoveX = 0;
                    data.globalData.globalMoveY = 0;
                    clickHighlight = {x: -10, y: -10, arrayRef: null, startOrEnd: null};
                    clickHighlightActive = false;
                    returnData = [];
                    dispatch('click', {
                        txClicked: null,
                        data: returnData
                    });
            }
            // if(!isWithin){
            //     console.log("not within!");
            //     console.log("X ", clickHighlight.x);
            //     console.log("Y ", clickHighlight.y);
            //     if (clickHighlight.x != -10) {
            //             drawOverlayLinks(clickHighlight.x, clickHighlight.y);
            //     }
            // }          
        }

        window.addEventListener('mouseup',function(e) {userUP(e)});
        window.addEventListener("touchend", function(e) {userUP(e)});
        function userUP(e){
            isDown = false;
            firstPos = 0;
            lastPos = 0;
            let isWithin = false;
            tgt = e.target.className;
            if(tgt.includes("cvs")) isWithin = true;
            if(isWithin && !buttonClick && !dragActive) firstPos = getMouesPosition(e);
            // overlay lines
            if(isWithin) {
                dist = (Math.abs(firstPos.x - clickHighlight.x)+Math.abs(firstPos.y - clickHighlight.y));
                if(dist <= 6 && clickHighlightActive){
                    // clear on 2nd click
                    clickHighlight = {x: -10, y: -10, arrayRef: null, startOrEnd: null};
                    dispatch('click', {
                        txClicked: null,
                        data: returnData
                    });
                } else{
                    if(!dragActive && !buttonClick) getNearestNode(firstPos.x, firstPos.y);
                    clickHighlightActive = true;
                    if (clickHighlight.x != -10) {
                        drawOverlayLinks(clickHighlight.x, clickHighlight.y);
                    }
                }
            }
            buttonClick = false;
            dragActive = false;
        }
    }
    
    let resized = false;
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
        populateOnce = false;
        populateDrawArray(blueprints);
        resized = true;
    }

    // animate settings
    let [bgR, bgG, bgB, bgA] = data.globalData.canvasBGColour;
    let animateIn = false;
    let populateOnce = false;
    let mutateComplete = false;
    let drawComplete = false;
    let noCalc = false;
    let count = 0;
    let addTX = 0;
    let mutateTX = 0;
    let totAddedTx = 0;
    let drawTX = 0;
    let daLen = 0;
    let i;

    //SPEED SETTINGS
    let addNewTransaction = 0;
    let mutate =1;
    let max = 250;
    let cu = 0;

    // Mutate Settings
    let alphaChange = 0.1; //
    let maxAlpha = 1;
    mutateComplete = true; // ignoring mutate at the moment.

    // draw aray settings
    let addedTo = 0;
    function populateDrawArray(blueprints){
        let bpLen = blueprints?.length ?? 0;
        if(animateIn == true){
            if(bpLen > 0 && addedTo < bpLen){
                drawArray.push(blueprints[addedTo]);
                addedTo++;
            }
        } else {
            for(let i= 0; i<bpLen; i++){
                drawArray.push(blueprints[i]);
            }
            populateOnce=true;
        }
        startDraw = true;
    }
    function mutateAndPrune(){
        let i, daLen;
        daLen = drawArray?.length ?? 0;
        let alpha;
        let mutateComplete = true;
        for(i=0; i<daLen; i++){
            // alpha
            alpha = drawArray[i].nodeArray[0].data[0].colour[3];
            if(alpha < maxAlpha) {
                mutateComplete = false;
                drawArray[i].nodeArray[0].data[0].colour[3] += alphaChange;
                drawArray[i].nodeArray[1].data[0].colour[3] += alphaChange;
                drawArray[i].nodeArray[2].data[0].colour[3] += alphaChange;
            }
        }
    }
    // main loop
    function animate(){
        if(!mutateComplete && drawComplete && !isDown && !resized) noCalc = true;
        else noCalc = false;

        if(!noCalc){
        ctx.clearRect(0, 0, clientWidth, clientHeight);
        ctx.fillStyle = `rgba(${bgR},${bgG},${bgB},${bgA})`;
        ctx.fillRect(0,0,clientWidth,clientHeight);
        // add TX (Speed adjustable)
        if(!animateIn && !populateOnce) populateDrawArray(blueprints);
        if(addTX>=addNewTransaction && cu < max && animateIn == true){
            populateDrawArray(blueprints);
            addTX = 0;
            cu++;
        }
        // draw
        if(startDraw){
            daLen = drawArray.length ?? 0;
            paintbrush(drawArray);
            // TODO -  click highlight here
            zoomButtons();
        }
        // mutate
        // if(!mutateComplete){
        //     if(mutateTX>=mutate){
        //         mutateAndPrune();
        //         mutateTX = 0;
        //     }
        // }

        // text 
            ctx.font = `12px Verdana`;
            var txt = data?.globalData?.text ?? "";
            ctx.fillStyle = 'rgba(250,255,250,1)';// 
            ctx.fillText(txt, 10, 15); 

        
        count++;
        addTX++;
        drawTX++;
        mutateTX++;
        }// nocalc

        if(!stopped) requestAnimationFrame(animate);
    }

    // UTILS for animate
    function paintbrush(blueprints){
        referencePositions = [];
        drawComplete = false;
        let i,k;
        let bpLen = blueprints?.length ?? 0;
        let bpSections = 0;
        let size, sX, sY, eX, eY, R,G,B,A;
        let start = 0; 
        let end = Math.PI * 2;
        let gX, gY, gZ, zoomChangeX, zoomChangeY;
        let gZAX = 0;
        let gZAY = 0;

        for(i=0;i<bpLen; i++){
            bpSections = blueprints[i].sections;
            gX = (data.globalData.globalMoveX/25);
            gY = (data.globalData.globalMoveY/25);
            gZ = data.globalData.globalZoom;
            if(data.globalData.globalZoom != 1){
            zoomChangeX = ((initWidth*data.globalData.globalZoom)-initWidth)/2;
            zoomChangeY = ((initHeight*data.globalData.globalZoom)-initHeight)/2;
            gZAX = (zoomChangeX/2)+initWidth*.05;
            gZAY = (zoomChangeY/2)+initHeight*.05;
            }
            for(k=0; k<bpSections; k++){
                // draw types
                if(blueprints[i].nodeArray[k].drawType == 'line'){
                    size = blueprints[i].nodeArray[k].data[0].size;
                    sX = ((blueprints[i].nodeArray[k].data[0].sX - gX)*gZ) - gZAX;
                    sY = ((blueprints[i].nodeArray[k].data[0].sY - gY)*gZ) - gZAY;
                    eX = ((blueprints[i].nodeArray[k].data[0].eX - gX)*gZ) - gZAX;
                    eY = ((blueprints[i].nodeArray[k].data[0].eY - gY)*gZ) - gZAY;
                    [R,G,B,A] = blueprints[i].nodeArray[k].data[0].colour ?? [255,255,255,255];
                    ctx.lineWidth = size;
                    ctx.strokeStyle = `rgba(${R},${G},${B},${A})`;
                    ctx.beginPath();
                    ctx.moveTo(sX, sY);
                    ctx.lineTo(eX, eY);
                    ctx.stroke();
                    referencePositions.push({sX: sX,sY: sY,eX: eX, eY: eY, data: data.transactions[i]});
                }
                else if(blueprints[i].nodeArray[k].drawType == 'circle'){
                    size = blueprints[i].nodeArray[k].data[0].size;
                    sX = ((blueprints[i].nodeArray[k].data[0].x - gX)*gZ) - gZAX;
                    sY = ((blueprints[i].nodeArray[k].data[0].y - gY)*gZ) - gZAY;
                    [R,G,B,A] = blueprints[i].nodeArray[k].data[0].colour ?? [255,255,255,255];
                    ctx.beginPath();
                    ctx.arc(sX, sY, size, start, end, false);
                    ctx.fillStyle = `rgba(${R},${G},${B},${A})`;
                    ctx.fill();
                }                
            }
        }
        drawComplete = true;
        resized = false;
    }

    function zoomButtons(){
        // + 
        ctx.strokeStyle = "rgb(0, 0, 0)";
        ctx.fillStyle = "rgba(200, 200, 200, 0.4)";
        ctx.beginPath();
        ctx.roundRect(clientWidth-50,(clientHeight/2)-50,40,40, 10);
        ctx.stroke();
        ctx.fill();
        
        ctx.font = `30px Verdana`;
        var txt = "+";
        ctx.fillStyle = `rgba(255,255,255,1)`;
        ctx.fillText(txt, (clientWidth-42), (clientHeight/2)-20); 

        ctx.strokeStyle = "rgb(0, 0, 0)";
        ctx.fillStyle = "rgba(200, 200, 200, 0.4)";
        ctx.beginPath();
        ctx.roundRect(clientWidth-50,(clientHeight/2),40,40, 10);
        ctx.stroke();
        ctx.fill();

        ctx.font = `30px Verdana`;
        var txt = "_";
        ctx.fillStyle = `rgba(255,255,255,1)`;
        ctx.fillText(txt, (clientWidth-40), (clientHeight/2)+16); 


        ctx.strokeStyle = "rgb(0, 0, 0)";
        ctx.fillStyle = "rgba(50, 255, 200, 0.4)";
        ctx.beginPath();
        ctx.roundRect(clientWidth-50,(clientHeight/2)+50,40,40, 10);
        ctx.stroke();
        ctx.fill();

        ctx.font = `24px Verdana`;
        var txt = "R";
        ctx.fillStyle = `rgba(255,255,255,1)`;
        ctx.fillText(txt, (clientWidth-38), (clientHeight/2)+79); 

    }

    function getMouesPosition(e) {
        var mouseX;
        var mouseY;

        if(canvas?.width && canvas?.height && canvas?.clientWidth && canvas?.clientHeight) {
            mouseX = e.offsetX * canvas.width / canvas.clientWidth | 0;
            mouseY = e.offsetY * canvas.height / canvas.clientHeight | 0;
        }
        return {x: mouseX, y: mouseY};
    }

    function getNearestNode(mouseX, mouseY){
        let dataLen = referencePositions?.length ?? 0;
        let i;
        let maxD;
        if(data.globalData.canvasWidth >= data.globalData.canvasHeight) maxD = data.globalData.canvasWidth;
        else maxD = data.globalData.canvasHeight;
        let minObj = {
            distance: maxD*maxD,
            arrayPos: null,
            startOrEnd: null
        };
        let distX, distY;
        for(i=0; i<dataLen; i++){

            distX = Math.abs(referencePositions[i].sX - mouseX);
            distY = Math.abs(referencePositions[i].sY - mouseY);
            if((distX+distY) < minObj.distance) {
                minObj.distance = (distX+distY);
                minObj.arrayPos = i;
                minObj.startOrEnd = 'start';
            }
            distX = Math.abs(referencePositions[i].eX - mouseX);
            distY = Math.abs(referencePositions[i].eY - mouseY);
            if((distX+distY) < minObj.distance) {
                minObj.distance = (distX+distY);
                minObj.arrayPos = i;
                minObj.startOrEnd = 'end';
            }
        }
        if(minObj.startOrEnd == 'start'){
            clickHighlight.x = referencePositions[minObj.arrayPos].sX;
            clickHighlight.y = referencePositions[minObj.arrayPos].sY;
            clickHighlight.arrayRef = minObj.arrayPos;
            clickHighlight.startOrEnd = minObj.startOrEnd;
            
            // Only using account ID at this time. 
            // if( referencePositions[minObj.arrayPos].data.fromPrincipal && 
            //     referencePositions[minObj.arrayPos].data.fromPrincipal != "" &&
            //     referencePositions[minObj.arrayPos].data.fromPrincipal != "N/A"
            // ){
            //     clickHighlight.clickedPR = referencePositions[minObj.arrayPos].data.fromPrincipal;
            // } else {
                clickHighlight.clickedPR = referencePositions[minObj.arrayPos].data.fromPrincipal;
                clickHighlight.clickedAC = referencePositions[minObj.arrayPos].data.fromAccount;
           // }
        }
        if(minObj.startOrEnd == 'end'){
            clickHighlight.x = referencePositions[minObj.arrayPos].eX;
            clickHighlight.y = referencePositions[minObj.arrayPos].eY;
            clickHighlight.arrayRef = minObj.arrayPos;
            clickHighlight.startOrEnd = minObj.startOrEnd;
            // if( referencePositions[minObj.arrayPos].data.toPrincipal && 
            //     referencePositions[minObj.arrayPos].data.toPrincipal != "" &&
            //     referencePositions[minObj.arrayPos].data.toPrincipal != "N/A"
            // ){
            //     clickHighlight.clickedPR = referencePositions[minObj.arrayPos].data.toPrincipal;
            // } else {
                clickHighlight.clickedPR = referencePositions[minObj.arrayPos].data.toPrincipal;
                clickHighlight.clickedAC = referencePositions[minObj.arrayPos].data.toAccount;
            //}
        }
        return minObj.arrayPos;
    }

    function drawOverlayLinks(nodeX, nodeY,){
        if(dragActive || buttonClick) {
            if(clickHighlight.startOrEnd == 'start'){
                nodeX = referencePositions[clickHighlight.arrayRef]?.sX; 
                nodeY = referencePositions[clickHighlight.arrayRef]?.sY;
            } else {
                nodeX = referencePositions[clickHighlight.arrayRef]?.eX; 
                nodeY = referencePositions[clickHighlight.arrayRef]?.eY;
            }
        }

        let i;
        let arLen = referencePositions?.length ?? 0;
        returnData = [];
        for(i=0; i<arLen; i++){
            //outgoing
            if(referencePositions[i].sX == nodeX && referencePositions[i].sY == nodeY){
                    ctx.lineWidth = data.settings[0].size;
                    ctx.strokeStyle = 'rgba(255,0,0,0.5)';
                    ctx.beginPath();
                    ctx.moveTo(referencePositions[i].sX, referencePositions[i].sY);
                    ctx.lineTo(referencePositions[i].eX, referencePositions[i].eY);
                    ctx.stroke();
                    returnData.push(referencePositions[i].data);
            }

            //incoming
            if(referencePositions[i].eX == nodeX && referencePositions[i].eY == nodeY){
                    ctx.lineWidth = data.settings[0].size;
                    ctx.strokeStyle = 'rgba(0,255,0,0.5)';
                    ctx.beginPath();
                    ctx.moveTo(referencePositions[i].sX, referencePositions[i].sY);
                    ctx.lineTo(referencePositions[i].eX, referencePositions[i].eY);
                    ctx.stroke();
                    returnData.push(referencePositions[i].data);
            }
        }

        // clicked overlay
        ctx.beginPath();
        if(clickHighlight.startOrEnd == 'start'){
            ctx.arc(referencePositions[clickHighlight.arrayRef]?.sX, referencePositions[clickHighlight.arrayRef]?.sY , 4, 0, Math.PI * 2, false);
        }else{
            ctx.arc(referencePositions[clickHighlight.arrayRef]?.eX, referencePositions[clickHighlight.arrayRef]?.eY , 4, 0, Math.PI * 2, false);
        }
        ctx.fillStyle = `rgba(230,230,0,1)`;
        ctx.fill();
        drawTX = 0;

        dispatch('click', {
                        txClicked: {principal: clickHighlight.clickedPR, account: clickHighlight.clickedAC},
                        data: returnData
            });
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