const DOMAIN = "https://app221b.herokuapp.com"; //"http://localhost:3000"; //

// CANVAS
const canvas = document.getElementById("canvas1");
const ctx = canvas.getContext('2d');
var clientWidth = canvas.clientWidth;
var clientHeight = canvas.clientHeight;

canvas.width = clientWidth;
canvas.height = clientHeight;

let cvsWidth = clientWidth;
let cvsHeight = clientHeight;
let jobArray = []; // Main Array of blueprints
let dataArray = []; // tx data
let pxDataArray = []; // for click search
let frameCounter = 0;
let nextDrawFrame = 0;
let blueprintsCreated = false;
let drawComplete = false;
let dataLoaded = false;
let calledOnce = false;

// UI Global Settings
let clickedToOrFrom = "X";
let clickedDataPosition = 0;
let clickedTime = 0;
let horizontalLine = false;
let verticalLine = false;
let mouseOnCanvas = false;
let numAccountsSearch = 2;
let advancedSearch =false; 
let acToThruClick;
let labelsOn = false;
let check_ICP = false;
let check_ckBTC = false;
let check_WICP = false;
let check_linked = false;

// Scale/ Position
let globalMoveX = 0;
let globalMoveY = 0;
let globalZoom = 1;
let zoomTick = 0.1;
let moveTick = 2.5;
const MAX_ZOOM = 50;
const MIN_ZOOM = 0;

// Shapes 
const DRAW_TYPE = {
    Line : "line",
    Circle : "circle",
    Text : "text",
    Diamond : "diamond"
}
// job Array contains {blueprint}
class Blueprint {
    constructor() {
        this.sections,
        this.completed,
        this.nodeArray = [] // contains Nodes
    }
}
// details array contains 
class DrawNode {
    constructor() {
        this.DrawType,
        this.data = []
    }
}

// init setup
async function init() {
    if (dataLoaded == true){
        ctx.clearRect(0, 0, canvas.width, canvas.height);
        jobArray = [];
        frameCounter = 0;
        nextDrawFrame = 0;
        drawComplete = false;
        blueprintsCreated = false;
        canvas.width = window.innerWidth;
        canvas.height = window.innerHeight;
        cvsWidth = window.innerWidth;
        cvsHeight = window.innerHeight;
        artManager(dataArray);
        //uiManager();
    }
    if(dataLoaded == false){
        // LOADING ART
        ctx.font = `Bold 25px Arial`;
        ctx.fillStyle = "white";
        var txt = "Visual Block Explorer";
        let txW = ctx.measureText(txt).width;
        ctx.fillText(txt, (clientWidth/2)-(txW/2), clientHeight/2);  
    }
}

// ART MANAGER 
function artManager(data){
    if(blueprintsCreated == false){
        // create blueprints
      //  console.log(data);
        let dataLen = data?.transactions?.length ?? 0;
        let set2;
        let ast;
        for(let i=0; i<dataLen; i++){
            ast = data.transactions[i].asset;
            let [sX, sY] = translate(data.transactions[i].startX,  data.transactions[i].startY, false);
            let [eX, eY] = translate(data.transactions[i].endX,  data.transactions[i].endY, false);
            pxDataArray[i] = {
                sX : sX, 
                sY : sY, 
                eX : eX, 
                eY : eY, 
                hash : data.transactions[i].hash
            };
            // colours for assets
            if(ast == "ICP"){
                set2 = {
                    size: 2,
                    startColour: [200,200,200,0.33],
                    endColour: [200,200,200,0.33],
                    sections : 50, // OPs 1 extra for end point
                    curveOffset : 0.001,
                    curveDirection : 1,
                    curveAplitude : 180
                };
            }
            if(ast == "ckBTC"){
                set2 = {
                    size: 2,
                    startColour: [200,0,200,0.33],
                    endColour: [200,0,200,0.33],
                    sections : 50, // OPs 1 extra for end point
                    curveOffset : 0.1,
                    curveDirection : 1,
                    curveAplitude : 180
                };
            }
            if(ast == "WICP"){
                set2 = {
                    size: 2,
                    startColour: [200,200,0,0.33],
                    endColour: [200,200,0,0.33],
                    sections : 50, // OPs 1 extra for end point
                    curveOffset : 5,
                    curveDirection : 1,
                    curveAplitude : 180
                };
            }
        
            solidLineArtist(sX,sY,eX,eY,set2);
           // lineAndDotsArtist(sX,sY,eX,eY,set1);
           //dottedLineArtist(sX,sY,eX,eY,set2);
        }
        let set1;
        for(let i=0; i<dataLen; i++){
        let [sX, sY] = translate(data.transactions[i].startX,  data.transactions[i].startY, false);
        let [eX, eY] = translate(data.transactions[i].endX,  data.transactions[i].endY, false);
        set1 = {
            size: 2,
            startColour: [255,50,50,1],
            endColour: [0,180,180,1],
            sections : 15, // OPs 1 extra for end point
            curveOffset : 0.001,
            curveDirection : 1,
            curveAplitude : 180
        };
        timelineArtist(sX,sY,eX,eY,set1);
        }

        // UI Settings 
        horizontalLine = false;
        verticalLine = false;

        blueprintsCreated = true;
    }
     if(drawComplete == false) paintbrush(0,-1,true,jobArray);
     requestAnimationFrame(artManager);
}

// DRAW STYLES
function solidLineArtist(sX, sY, eX, eY, settings){
    // load settings
    let size = settings?.size ?? 1;
    let sections = settings?.sections ?? 1;
    let [sR,sG,sB,sA] = settings?.startColour ?? [255,255,255,255];
    let [eR,eG,eB,eA] = settings?.endColour ?? [255,255,255,255];
    let curveDirection = settings?.curveDirection ?? 1;
    let curveOffset = settings?.curveOffset ?? 5;
    let curveAplitude = settings?.curveAplitude ?? 180;
    // calculate sections
    let i;
    let CRV = [];
    let OP = new Blueprint;
    OP.sections = sections; 
    OP.completed = 0;
    let DN;
    CRV = plotCurve(sX, sY, eX, eY, curveDirection, curveAplitude, curveOffset, sections); // OP 1 extra for end point
    let currentColour = [0,0,0,0];
    let [csR, csG, csB, csA] = colourSteps([sR,sG,sB,sA], [eR,eG,eB,eA], sections);
    for(i=0; i<sections; i++){ // -1 for end xy on line draw
        // colour tween
        if(i == 0) currentColour = [sR,sG,sB,sA];
        else{
            currentColour[0] += csR;
            currentColour[1] += csG;
            currentColour[2] += csB;
            currentColour[3] += csA;
        }
        // push nodes into node array
        DN = new DrawNode;
        DN.DrawType = DRAW_TYPE.Line;
        DN.data = [{
            "size" : size,
            "sX" : CRV[i].x,
            "sY" : CRV[i].y,
            "eX" : CRV[i+1].x,
            "eY" : CRV[i+1].y,
            "colour" : [currentColour[0],currentColour[1],currentColour[2],currentColour[3]]
        }];  
        OP.nodeArray.push(DN)  
    }
    jobArray.push(OP);
}
function dottedLineArtist(sX, sY, eX, eY, settings){
    // load settings
    let size = settings?.size ?? 1;
    let sections = settings?.sections ?? 1;
    let [sR,sG,sB,sA] = settings?.startColour ?? [255,255,255,255];
    let [eR,eG,eB,eA] = settings?.endColour ?? [255,255,255,255];
    let curveDirection = settings?.curveDirection ?? 1;
    let curveOffset = settings?.curveOffset ?? 5;
    let curveAplitude = settings?.curveAplitude ?? 180;
    // calculate sections
    let i;
    let CRV = [];
    let OP = new Blueprint;
    OP.completed = 0;
    let DN;
    CRV = plotCurve(sX, sY, eX, eY, curveDirection, curveAplitude, curveOffset, sections); // OP 1 extra for end point
    let currentColour = [0,0,0,0];
    let [csR, csG, csB, csA] = colourSteps([sR,sG,sB,sA], [eR,eG,eB,eA], sections);
    let altCount = 0;
    let added = 0;
    for(i=0; i<sections; i++){ // -1 for end xy on line draw
        // colour tween
        if(i == 0) currentColour = [sR,sG,sB,sA];
        else{
            currentColour[0] += csR;
            currentColour[1] += csG;
            currentColour[2] += csB;
            currentColour[3] += csA;
        }
        // push nodes into node array
        DN = new DrawNode;
        DN.DrawType = DRAW_TYPE.Line;
        DN.data = [{
            "size" : size,
            "sX" : CRV[i].x,
            "sY" : CRV[i].y,
            "eX" : CRV[i+1].x,
            "eY" : CRV[i+1].y,
            "colour" : [currentColour[0],currentColour[1],currentColour[2],currentColour[3]]
        }];  
        if(isEven(altCount) == true){
            OP.nodeArray.push(DN);
            added++;
        }
        altCount++;
    }
    OP.sections = added; 
    jobArray.push(OP);
}
function lineAndDotsArtist(sX, sY, eX, eY, settings){
    // load settings
    let size = settings?.size ?? 1;
    let sections = settings?.sections ?? 1;
    let [sR,sG,sB,sA] = settings?.startColour ?? [255,255,255,255];
    
    let [eR,eG,eB,eA] = settings?.endColour ?? [255,255,255,255];
    let curveDirection = settings?.curveDirection ?? 1;
    let curveOffset = settings?.curveOffset ?? 5;
    let curveAplitude = settings?.curveAplitude ?? 180;
    // calculate sections
    let i;
    let CRV = [];
    let OP = new Blueprint;
    OP.sections = sections+2; // include 2 circles 
    OP.completed = 0;
    let DN;
    CRV = plotCurve(sX, sY, eX, eY, curveDirection, curveAplitude, curveOffset, sections); // OP 1 extra for end point
    let currentColour = [0,0,0,0];
    let [csR, csG, csB, csA] = colourSteps([sR,sG,sB,sA], [eR,eG,eB,eA], sections);
    
    // first Circle
    DN = new DrawNode;
    DN.DrawType = DRAW_TYPE.Circle;
    DN.data = [{
        "x" : sX,
        "y" : sY,
        "start" : 0,
        "end" : Math.PI * 2,
        "radius" : size*10,
        "colour" : [sR, sG, sB,sA],
        "reverse" : false
    }];  
    OP.nodeArray.push(DN);  

    for(i=0; i<sections; i++){ // -1 for end xy on line draw
        // colour tween
        if(i == 0) currentColour = [sR,sG,sB,sA];
        else{
            currentColour[0] += csR;
            currentColour[1] += csG;
            currentColour[2] += csB;
            currentColour[3] += csA;
        }
        // push nodes into node array
        DN = new DrawNode;
        DN.DrawType = DRAW_TYPE.Line;
        DN.data = [{
            "size" : size,
            "sX" : CRV[i].x,
            "sY" : CRV[i].y,
            "eX" : CRV[i+1].x,
            "eY" : CRV[i+1].y,
            "colour" : [currentColour[0],currentColour[1],currentColour[2],currentColour[3]]
        }];  
        OP.nodeArray.push(DN)  
    }

    // last Circle
    DN = new DrawNode;
    DN.DrawType = DRAW_TYPE.Circle;
    DN.data = [{
        "x" : eX,
        "y" : eY,
        "start" : 0,
        "end" : Math.PI * 2,
        "radius" : size*10,
        "colour" : [eR, eG, eB, eA],
        "reverse" : false
    }];  
    OP.nodeArray.push(DN);  

    jobArray.push(OP);
}
function timelineArtist(sX, sY, eX, eY, settings){
    // load settings
    let size = settings?.size ?? 1;
    let sections = settings?.sections ?? 1;
    let [sR,sG,sB,sA] = settings?.startColour ?? [255,255,255,255];
    
    let [eR,eG,eB,eA] = settings?.endColour ?? [255,255,255,255];
    let curveDirection = settings?.curveDirection ?? 1;
    let curveOffset = settings?.curveOffset ?? 5;
    let curveAplitude = settings?.curveAplitude ?? 180;
    // calculate sections
    let i;
    let CRV = [];
    let OP = new Blueprint;
    OP.sections = 2; // include 2 circles 
    OP.completed = 0;
    let DN;
    //CRV = plotCurve(sX, sY, eX, eY, curveDirection, curveAplitude, curveOffset, sections); // OP 1 extra for end point
    let currentColour = [0,0,0,0];
    let [csR, csG, csB, csA] = colourSteps([sR,sG,sB,sA], [eR,eG,eB,eA], sections);
    
    // first Circle
    DN = new DrawNode;
    DN.DrawType = DRAW_TYPE.Circle;
    DN.data = [{
        "x" : sX,
        "y" : sY,
        "start" : 0,
        "end" : Math.PI * 2,
        "radius" : size*10,
        "colour" : [sR, sG, sB,sA],
        "reverse" : false
    }];  
    OP.nodeArray.push(DN);  

    // last Circle
    DN = new DrawNode;
    DN.DrawType = DRAW_TYPE.Circle;
    DN.data = [{
        "x" : eX,
        "y" : eY,
        "start" : 0,
        "end" : Math.PI * 2,
        "radius" : size*10,
        "colour" : [eR, eG, eB, eA],
        "reverse" : false
    }];  
    OP.nodeArray.push(DN);  

    jobArray.push(OP);
}
function textArtist(sX, sY, settings){
    let fontSize = settings?.fontSize ?? 12;
    let fontType = settings?.fontType ?? "Arial";
    let [sR,sG,sB,sA] = settings?.colour ?? [255,255,255,255];
    let content = settings?.content ?? "Hello World";
    let background = settings?.background ?? false;

    let OP = new Blueprint;
    if (background == false){
        OP.sections = 1; 
    }
    if (background ==  true){
        OP.sections = 2;
    }
    OP.completed = 0;

    DN = new DrawNode;
    DN.DrawType = DRAW_TYPE.Text;
    DN.data = [{
        "fontSize" : fontSize,
        "fontType" : fontType,
        "colour" : [sR,sG,sB,sA],
        "content" : content,
    }];  
    OP.nodeArray.push(DN)  
    // ctx.font = "30px Arial";
    // var txt = "Hello World"
    // ctx.fillText("width:" + ctx.measureText(txt).width, 10, 50)
    // ctx.fillText(txt, 10, 100);
    // draw background...
}

//[][]---------------------------[][]
//            UTILS
//[][]---------------------------[][]
function translate(x, y, useViewport){
    let opX, opY;
    let inX, inY;
    inX = parseFloat(x);
    inY = parseFloat(y);
    // MOVE
    inX += globalMoveX;
    inY += globalMoveY;
    if(useViewport == false){
        opX = ((cvsWidth*globalZoom)/100)*inX;
        opY = ((cvsHeight*globalZoom)/100)*inY;
    }
    return [opX, opY];
}
function colourSteps(startColour, endColour, steps){
    let [sR,sG,sB,sA] = startColour ?? [0,0,0,0];
    let [eR,eG,eB,eA] = endColour ?? [255,255,255,255];
    let R,G,B,A;
    if(steps > 0 ){
    R = (eR-sR)/steps;
    G = (eG-sG)/steps;
    B = (eB-sB)/steps;
    A = (eA-sA)/steps;
    }
    else{
        let er = "steps cannot be less than 1"
        return er;
    }
    let Res = [R,G,B,A];
    return Res;
}
function plotCurve(sX, sY, eX, eY, direction, amplitude, shapeSize, sections){
    let OP = [];
    let i;
    // start points for calcs
    var x = sX;
    var y = sY;
    
    // input line (radians) 
    // theta is *-1 to match canvas direction
    var polar = {
      theta : Math.atan((eY - sY) / (eX - sX))*-1,
      r : (Math.sqrt(Math.pow((eX - sX),2)+Math.pow((eY - sY),2))) 
    };

    if (eX<sX) polar.r = polar.r*-1; // rotate line direction if needed
    var cx = sX;
    var cy = sY;
    var d = (amplitude/sections); // degrees per iteration 
    var r = d * (Math.PI/180); // radians per iteration 
    var t = 0; // current radian 
    var shapeOffset = (polar.r/100)*shapeSize; // offset as % of length
    
    for(i = 0; i<=sections; i++) {
        x = sX + (((polar.r)/sections)*i); 
        if (direction >=0) y = sY + (shapeOffset * Math.sin(t)); 
        if (direction < 0) y = sY - (shapeOffset * Math.sin(t)); 

        //change angle
        var radians = polar.theta; // (Math.PI / 180) * 0
        var cos = Math.cos(radians);
        var sin = Math.sin(radians);
        var nx = (cos * (x - cx)) + (sin * (y - cy)) + cx;
        var ny = (cos * (y - cy)) - (sin * (x - cx)) + cy;

        OP[i] = {"x" : nx, "y" : ny};
        t += r;// r*(sections/100);
    }//i
    return OP;
}
function drawLine(settings){
    // load settings
    let size = settings[0]?.size ?? 3;
    let sX = settings[0]?.sX ?? -1;
    let sY = settings[0]?.sY ?? -1;
    let eX = settings[0]?.eX ?? -1;
    let eY = settings[0]?.eY ?? -1;
    let [R,G,B,A] = settings[0]?.colour ?? [255,255,255,255];
    ctx.lineWidth = size;
    ctx.strokeStyle = `rgba(${R},${G},${B},${A})`;
    ctx.beginPath();
    ctx.moveTo(sX, sY);
    ctx.lineTo(eX, eY);
    ctx.stroke();
}
function drawCircle(settings){
    // load settings
    let size = settings[0]?.size ?? 3;
    let x = settings[0]?.x ?? -1;
    let y = settings[0]?.y ?? -1;
    let start = settings[0]?.start ?? 0;
    let end = settings[0]?.end ?? Math.PI * 2;
    let [R,G,B,A] = settings[0]?.colour ?? [255,255,255,255];

    ctx.beginPath();
    ctx.arc(x, y, size, start, end, false);
    ctx.fillStyle = `rgba(${R},${G},${B},${A})`;
    ctx.fill();
}
function isEven(value){
    if (value%2 == 0)
        return true;
    else
        return false;
}

// MAIN DRAW FUNCTION
function paintbrush(speed, batchSize, drawAll, jobArray){
    let i,k;
    
    let jaLen = batchSize;
    let allComplete;
    let startPoint = 0;
    if(frameCounter >= nextDrawFrame){
        let len = jobArray.length;
        let done = 0;
        let DT, param1;
        let sections;
        allComplete = true;
        if (batchSize === -1) jaLen = len;
        for(i=0; i<jaLen; i++){
            sections = jobArray[i].sections;
            done = jobArray[i].completed;
            if (drawAll && done<sections){
                allComplete = false;
                for(k=0; k<sections; k++){
                    DT = jobArray[i].nodeArray[k].DrawType;
                    param1 = jobArray[i].nodeArray[k].data;
                    if (DT === "circle") drawCircle(param1); 
                    if (DT === "line") drawLine(param1); 
                }
                jobArray[i].completed = sections;
            }
            else if(done<sections){
                allComplete = false;
                DT = jobArray[i].nodeArray[done].DrawType;
                param1 = jobArray[i].nodeArray[done].data;
                if (DT === "line") drawLine(param1);
                if (DT === "circle") drawCircle(param1); 
                jobArray[i].completed = done+1;
                if(done+1 == sections && jobArray.length > batchSize) {
                    jobArray.splice(i, 1);
                }
            }
        }
       nextDrawFrame = frameCounter+speed;
    }

    // if not complete
    frameCounter++;
    if(allComplete) drawComplete = true;
}

//[][]---------------------------[][]
//             UI STUFF
//[][]---------------------------[][]

// FIXED MOUSE POSITION
let mouse = {
    "x" : 0,
    "y" : 0
};
function getMouesPosition(canvasNumber, e) {
    var mouseX;
    var mouseY;
    if(canvasNumber == 1){
        mouseX = e.offsetX * canvas.width / canvas.clientWidth | 0;
        mouseY = e.offsetY * canvas.height / canvas.clientHeight | 0;
    }
    if(canvasNumber == 2){
        mouseX = e.offsetX * canvas2.width / canvas2.clientWidth | 0;
        mouseY = e.offsetY * canvas2.height / canvas2.clientHeight | 0;
    }
    return {x: mouseX, y: mouseY};
}
window.addEventListener("resize", function(event){
    event.preventDefault();
    clientWidth = canvas.clientWidth;
    clientHeight = canvas.clientHeight;
    canvas.width = clientWidth;
    canvas.height = clientHeight;
    cvsWidth = clientWidth;
    cvsHeight = clientHeight;
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    dataLoaded = true;
    jobArray = [];
    frameCounter = 0;
    nextDrawFrame = 0;
    drawComplete = false;
    blueprintsCreated = false;
    artManager(dataArray);
});


if(calledOnce == false){
    init();// setup
    calledOnce = true;
}


// window.addEventListener('click',
//     function(event) {
//         if(mouseOnCanvas == true){
//             ctx2.clearRect(0, 0, canvas2.width, canvas2.height);
//             labelsOn = false; // update other show all label state 

//             mouse.x = event.offsetX;
//             mouse.y = event.offsetY;

//             let newX = getMouesPosition(2,event).x;
//             let newY = getMouesPosition(2,event).y;

//             let closestResult = lookupClosestData(newX, newY, "all", dataArray, pxDataArray);
//             let [ts_cX, ts_cY] = translate(closestResult.x, closestResult.y, false);

//             // Mouse click circle canvas 3 (clears)
//             // ctx3.beginPath();
//             // ctx3.arc(newX, newY, 3, 0, Math.PI*2, false);
//             // ctx3.fillStyle = 'rgba(255,50,100,255)';
//             // ctx3.fill();
            
//             // datafromTX
//             ctx2.beginPath();
//             ctx2.arc(ts_cX, ts_cY, 5, 0, Math.PI*2, false);
//             ctx2.fillStyle = 'rgba(255, 255, 0, 1)';
//             ctx2.fill();
            
//             // SHOW Account Name
//             if(closestResult.whatClick == "from"){
//                 showShortName(ts_cX,ts_cY,closestResult.fromName, 'rgba(255, 255, 0, 1)');
//                 updateInfoBox(closestResult.data[1].from,closestResult.fromName);
//                 acToThruClick = closestResult.data[1].from;
//             }
//             if(closestResult.whatClick == "to"){
//                 showShortName(ts_cX,ts_cY,closestResult.toName, 'rgba(255, 255, 0, 1)');
//                 updateInfoBox(closestResult.data[1].to,closestResult.toName);
//                 acToThruClick = closestResult.data[1].to
//             }

//             // Click Through/ MODAL BUTTONS
//             $('#thruSearch').click(function() {
//                 localStorage.setItem("referSearch", true);
//                 localStorage.setItem("referAccount", acToThruClick);
//                 $('#ACmodal').modal('toggle');
//                 document.getElementById("ACmodalBody").innerHTML = `
//                 <iframe src="AccountSearch.html" width="100%" height="100%" frameBorder="0">Browser not compatible.</iframe>`;
//             });  

//         }// on canvas
//     });
// window.addEventListener('mousemove',
//     function(event) {
//         let rawX = event.x;
//         let rawY = event.y;

//         mouse.x = event.offsetX;
//         mouse.y = event.offsetY;
//         let newX = getMouesPosition(2,event).x;
//         let newY = getMouesPosition(2,event).y;
//         let bound = canvas.getBoundingClientRect();


//         // is mouse on the canvas
//         if(rawX < (bound.x+10) || rawX >= bound.right) mouseOnCanvas = false;
//         else mouseOnCanvas = true;
//         if(rawY <= bound.y+2 || rawY >= bound.bottom) mouseOnCanvas = false;
//         else mouseOnCanvas = true;
//         ctx3.clearRect(0, 0, canvas2.width, canvas2.height);

//         if(mouseOnCanvas == true) {
//             // Horiz Line
//             if (horizontalLine == true){
//                 ctx3.lineWidth = 2;
//                 ctx3.strokeStyle = 'rgba(200,200,200,1)';
//                 ctx3.beginPath();
//                 ctx3.moveTo(0, newY);
//                 ctx3.lineTo(canvas2.width, newY);
//                 ctx3.stroke();
//             }
//             // Vertical Line
//             if (verticalLine == true){
//                 ctx3.lineWidth = 2;
//                 ctx3.strokeStyle = 'rgba(200,200,200,1)';
//                 ctx3.beginPath();
//                 ctx3.moveTo(newX, 0);
//                 ctx3.lineTo(newX, canvas2.height);
//                 ctx3.stroke();
//             }
//         }// on canvas
//     }
// );


//     let url = DOMAIN+"/blockVisualMulti";
//     let sData = {
//         start : start,
//         end    : end,
//         user    : user,
//         assets : assets,
//         linked : check_linked
//     };
//     await fetch(url, {
//         method: "POST",
//         body: JSON.stringify(sData),
//         headers: {"Content-type": "application/json; charset=UTF-8"}
//         })
//     .then((res) => res.json())
//     .then((data) => {
//         console.log("Data Rec'd ", data.transactions.length);
//         // Check for errors
//         let returnError = false;
//         if (data == "Too Many Results"){
//             error = 'ERROR :Too Many Results - Max Blocks is 10,000';
//             document.getElementById("searchErrors").innerHTML = error;
//             returnError = true;
//         }
//         if (data == "No Blocks Found"){
//             error = 'ERROR : No Blocks found matching the search parameters';
//             document.getElementById("searchErrors").innerHTML = error;
//             returnError = true;
//         }
//         if (data == "Warning - No assets included in the BlockVisualMulti request!"){
//             error = 'ERROR : No Assets Selected';
//             document.getElementById("searchErrors").innerHTML = error;
//             returnError = true;
//         }
//         if (returnError == false){
//             document.getElementById("searchErrors").innerHTML = "";

//             if (data.length == 20000){
//                 error = 'Note: Result has been limited to 10,000 blocks';
//                 document.getElementById("searchErrors").innerHTML = error;
//                 //returnError = false; <= still show the result! 
//             }

//             let iBox = document.getElementById("infoBox");
//                 iBox.removeAttribute("hidden"); 

//                 clientWidth = canvas.clientWidth;
//                 clientHeight = canvas.clientHeight;
//                 canvas.width = clientWidth;
//                 canvas.height = clientHeight;
//                 canvas2.width = clientWidth;
//                 canvas2.height = clientHeight;
//                 canvas3.width = clientWidth;
//                 canvas3.height = clientHeight;
//                 cvsWidth = clientWidth;
//                 cvsHeight = clientHeight;
//                 ctx.clearRect(0, 0, canvas.width, canvas.height);
//                 ctx2.clearRect(0, 0, canvas.width, canvas.height);
//                 dataLoaded = true;
//                 jobArray = [];
//                 frameCounter = 0;
//                 nextDrawFrame = 0;
//                 drawComplete = false;
//                 blueprintsCreated = false;
//                 dataArray = data;
//                 resetArray = data;
//                 artManager(data);
//         }
//     });
// }



// async function blockSearch(start, end, user){
//     ctx.clearRect(0, 0, canvas.width, canvas.height);
//     ctx2.clearRect(0, 0, canvas.width, canvas.height);
//     ctx2.font = `Bold 25px Arial`;
//     ctx2.fillStyle = "white";
//     var txt = "Loading...";
//     let txW = ctx2.measureText(txt).width;
//     ctx2.fillText(txt, (clientWidth/2)-(txW/2), clientHeight/2);  

//     document.getElementById("searchErrors").innerHTML = "";

//     let assets = [];
//     if(check_ICP == true) assets.push("ICP");
//     if(check_ckBTC == true) assets.push("ckBTC");
//     if(check_WICP == true) assets.push("WICP");

