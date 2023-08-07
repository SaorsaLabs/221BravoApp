let cvsWidth, cvsHeight;
let globalZoom = 1;
let inX, inY;
let globalMoveX, globalMoveY;
let numSections = 60; // <= change in main componant as well

const DRAW_TYPE = {
	Line: 'line',
	Circle: 'circle',
	Text: 'text',
	Diamond: 'diamond'
};

class Blueprint {
	constructor() {
		this.sections, this.completed, (this.nodeArray = []); // contains Nodes
	}
}

class DrawNode {
	constructor() {
		this.DrawType, (this.data = []);
	}
}

function createBlueprints(dataArray) {
	// update Global data;
	cvsWidth = dataArray.globalData.canvasWidth;
	cvsHeight = dataArray.globalData.canvasHeight;
	globalZoom = dataArray.globalData.globalZoom;
	inX = dataArray.globalData.inX;
	inY = dataArray.globalData.inY;
	globalMoveX = dataArray.globalData.globalMoveX;
	globalMoveY = dataArray.globalData.globalMoveY;

	let dataLen = dataArray?.transactions?.length ?? 0;
	let token;
	let blueprints = [];
	let settings = {
		size: 1,
		sections: 1,
		startColour: [255, 255, 255, 255],
		endColour: [255, 255, 255, 255],
		curveOffset: 5,
		curveAplitude: 180,
		curveDirection: 1
	};
	let rndm1, rndm2, rndm3;

	for (let i = 0; i < dataLen; i++) {
		token = dataArray.transactions[i].token;
		let [sX, sY] = translate(
			dataArray.transactions[i].startX,
			dataArray.transactions[i].startY,
			false
		);
		let [eX, eY] = translate(dataArray.transactions[i].endX, dataArray.transactions[i].endY, false);
		if (token == 1) {
			settings.size = (cvsWidth / 100) * 0.035;
			settings.sections = numSections;
			settings.startColour = [0, 0, 0, 0.5];
			settings.endColour = [255, 255, 255, 0.5];
			settings.curveOffset = 5;
			settings.curveAplitude = 180;
		}
		rndm1 = Math.floor(Math.random() * 155) + 150;
		rndm2 = Math.floor(Math.random() * 155) + 150;
		rndm3 = Math.floor(Math.random() * 155) + 150;

		if (token == 2) {
			settings.size = (cvsWidth / 100) * 0.035;
			settings.sections = numSections;
			settings.startColour = [0, 0, 0, 0.1];
			settings.endColour = [rndm1, rndm2, rndm3, 1];
			settings.curveOffset = 5;
			settings.curveAplitude = 180;
		}
		// settings.curveDirection = (settings.curveDirection == 1)  ? -1 : 1;
		blueprints[i] = dotsArtist(sX, sY, eX, eY, settings);
	}
	return blueprints;
}

function dotsArtist(sX, sY, eX, eY, settings) {
	// load settings
	let size = settings?.size ?? 1;
	let sections = settings?.sections ?? 1;
	let [sR, sG, sB, sA] = settings?.startColour ?? [255, 255, 255, 1];
	let [eR, eG, eB, eA] = settings?.endColour ?? [255, 255, 255, 1];
	let curveDirection = settings?.curveDirection ?? 1;
	let curveOffset = settings?.curveOffset ?? 5;
	let curveAplitude = settings?.curveAplitude ?? 180;
	// calculate sections
	let i;
	let CRV = [];
	let OP = new Blueprint();
	OP.sections = sections; // include 2 circles
	OP.completed = 0;
	let DN;
	CRV = plotCurve(sX, sY, eX, eY, curveDirection, curveAplitude, curveOffset, sections); // OP 1 extra for end point
	let currentColour = [0, 0, 0, 0];
	let [csR, csG, csB, csA] = colourSteps([sR, sG, sB, sA], [eR, eG, eB, eA], sections);

	for (i = 0; i < sections; i++) {
		// -1 for end xy on line draw
		// colour tween
		if (i == 0) currentColour = [sR, sG, sB, sA];
		else {
			currentColour[0] += csR;
			currentColour[1] += csG;
			currentColour[2] += csB;
			currentColour[3] += csA;
		}
		// push nodes into node array
		DN = new DrawNode();
		DN.DrawType = DRAW_TYPE.Circle;
		DN.data = [
			{
				x: CRV[i].x,
				y: CRV[i].y,
				start: 0,
				end: Math.PI * 2,
				radius: size * 10,
				colour: [currentColour[0], currentColour[1], currentColour[2], currentColour[3]],
				reverse: false
			}
		];
		OP.nodeArray.push(DN);
	}
	return OP;
}

// UTILS FUNCTIONS
function translate(x, y, useViewport) {
	let opX, opY;
	let inX, inY;
	inX = parseFloat(x);
	inY = parseFloat(y);

	// MOVE
	// inX += globalMoveX;
	// inY += globalMoveY;
	if (useViewport == false) {
		opX = (cvsWidth / 100) * inX;
		opY = (cvsHeight / 100) * inY;
	}

	return [opX, opY];
}
function plotCurve(sX, sY, eX, eY, direction, amplitude, shapeSize, sections) {
	let OP = [];
	let i;
	// start points for calcs
	var x = sX;
	var y = sY;

	// input line (radians)
	// theta is *-1 to match canvas direction
	var polar = {
		theta: Math.atan((eY - sY) / (eX - sX)) * -1,
		r: Math.sqrt(Math.pow(eX - sX, 2) + Math.pow(eY - sY, 2))
	};

	if (eX < sX) polar.r = polar.r * -1; // rotate line direction if needed
	var cx = sX;
	var cy = sY;
	var d = amplitude / sections; // degrees per iteration
	var r = d * (Math.PI / 180); // radians per iteration
	var t = 0; // current radian
	var shapeOffset = (polar.r / 100) * shapeSize; // offset as % of length

	for (i = 0; i <= sections; i++) {
		x = sX + (polar.r / sections) * i;
		if (direction >= 0) y = sY + shapeOffset * Math.sin(t);
		if (direction < 0) y = sY - shapeOffset * Math.sin(t);

		//change angle
		var radians = polar.theta; // (Math.PI / 180) * 0
		var cos = Math.cos(radians);
		var sin = Math.sin(radians);
		var nx = cos * (x - cx) + sin * (y - cy) + cx;
		var ny = cos * (y - cy) - sin * (x - cx) + cy;

		OP[i] = { x: nx, y: ny };
		t += r; // r*(sections/100);
	} //i
	return OP;
}
function colourSteps(startColour, endColour, steps) {
	let [sR, sG, sB, sA] = startColour ?? [0, 0, 0, 0];
	let [eR, eG, eB, eA] = endColour ?? [255, 255, 255, 255];
	let R, G, B, A;
	if (steps > 0) {
		R = (eR - sR) / steps;
		G = (eG - sG) / steps;
		B = (eB - sB) / steps;
		A = (eA - sA) / steps;
	} else {
		let er = 'steps cannot be less than 1';
		return er;
	}
	let Res = [R, G, B, A];
	return Res;
}

export { createBlueprints };
