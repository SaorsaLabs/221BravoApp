
let cvsWidth, cvsHeight;
let globalZoom = 1;
let inX, inY;
let globalMoveX, globalMoveY;


const DRAW_TYPE = {
    Line : "line",
    Circle : "circle",
    Text : "text",
    Diamond : "diamond"
}

class Blueprint {
    constructor() {
        this.sections,
        this.completed,
        this.nodeArray = [] // contains Nodes
    }
}

class DrawNode {
    constructor() {
        this.drawType,
        this.data = []
    }
}

function createBlueprints(dataArray){
    // update Global data;
    cvsWidth = dataArray.globalData.canvasWidth;
    cvsHeight = dataArray.globalData.canvasHeight;
    globalZoom = dataArray.globalData.globalZoom;
    inX = dataArray.globalData.inX;
    inY = dataArray.globalData.inY;
    globalMoveX = dataArray.globalData.globalMoveX;
    globalMoveY = dataArray.globalData.globalMoveY;

    let dataLen = dataArray?.transactions?.length ?? 0;
    let token, i , k;
    let blueprints = [];
    let settings = {};
    let ISL = dataArray?.settings?.length ?? 0;
    if(ISL == 0) console.log("No blueprint settings!"); 

    for(i=0; i<dataLen; i++){
        // Fallback Settings
        settings = {
            token: "XXX",
            lineColour: [255,255,255,0.33],
            dotColour: [50,230,255,0.75],
            size: 1.5, // canvas width/100 * size
        }
        token = dataArray.transactions[i].token;
        let [sX, sY] = translate(dataArray.transactions[i].startX,  dataArray.transactions[i].startY, false);
        let [eX, eY] = translate(dataArray.transactions[i].endX,  dataArray.transactions[i].endY, false);
        for(k=0; k<ISL; k++){
            if(token == dataArray.settings[k].token){
                settings.size = dataArray.settings[k].size;
                settings.lineColour = dataArray.settings[k].lineColour;
                settings.dotColour = dataArray.settings[k].dotColour;
            }
        } 
        blueprints[i] = standardArtist(sX, sY, eX, eY, settings);
    }
    return blueprints;
}

function standardArtist(sX, sY, eX, eY, settings){
        // load settings
        let size = settings?.size ?? 1;
        let [sR,sG,sB,sA] = settings?.lineColour ?? [255,255,255,1];
        let [sR2,sG2,sB2,sA2] = settings?.dotColour ?? [255,255,255,1];

        let OP = new Blueprint;
        OP.sections = 3; // include 2 circles 
        OP.completed = 0;
        let DN;
        
        // Line 
        DN = new DrawNode;
        DN.drawType = DRAW_TYPE.Line;
        DN.data = [{
            "size" : size,
            "sX" : sX,
            "sY" : sY,
            "eX" : eX,
            "eY" : eY,
            "colour" : [sR, sG, sB,sA]
        }];  
        OP.nodeArray.push(DN) 

        // Start Circle
        DN = new DrawNode;
        DN.drawType = DRAW_TYPE.Circle;
        DN.data = [{
            "size" : size*1.35,
            "x" : sX,
            "y" : sY,
            "start" : 0,
            "end" : Math.PI * 2,
            "radius" : size*10,
            "colour" : [sR2, sG2, sB2,sA2],
            "reverse" : false
        }];  
        OP.nodeArray.push(DN); 

        // End Circle
        DN = new DrawNode;
        DN.drawType = DRAW_TYPE.Circle;
        DN.data = [{
            "size" : size*1.35,
            "x" : eX,
            "y" : eY,
            "start" : 0,
            "end" : Math.PI * 2,
            "radius" : size*10,
            "colour" : [sR2, sG2, sB2,sA2],
            "reverse" : false
        }];  
        OP.nodeArray.push(DN); 
        return OP;
}

// UTILS FUNCTIONS 
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

export {
    createBlueprints,
};