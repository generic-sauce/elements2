window.canvas = document.getElementById("main-canvas");

canvas.width = window.innerWidth;
canvas.height = window.innerHeight;

window.init_drawing = function() {
	window.gl = canvas.getContext("webgl");

	const vsSource = `
		attribute vec4 aVertexPosition;

		void main() {
			gl_Position = aVertexPosition;
		}
	`;

	const fsSource = `
		void main() {
			gl_FragColor = vec4(1.0, 1.0, 1.0, 1.0);
		}
	`;

	const shaderProgram = initShaderProgram(gl, vsSource, fsSource);
	window.programInfo = {
		program: shaderProgram,
		attribLocations: {
			vertexPosition: gl.getAttribLocation(shaderProgram, 'aVertexPosition'),
		},
		uniformLocations: {
		},
	};

	window.buffers = initBuffers(gl);
}

window.draw_world = function(world) {
	window.world = world;

	drawScene(window.gl, window.programInfo, window.buffers);
}

window.drawScene = function(gl, programInfo, buffers) {
	gl.clearColor(0.0, 0.0, 0.0, 1.0);
	gl.clear(gl.COLOR_BUFFER_BIT);

	{
		const numComponents = 2;	// pull out 2 values per iteration
		const type = gl.FLOAT;		// the data in the buffer is 32bit floats
		const normalize = false;	// don't normalize
		const stride = 0;				 // how many bytes to get from one set of values to the next
															// 0 = use type and numComponents above
		const offset = 0;				 // how many bytes inside the buffer to start from
		gl.bindBuffer(gl.ARRAY_BUFFER, buffers.position);
		gl.vertexAttribPointer(
				programInfo.attribLocations.vertexPosition,
				numComponents,
				type,
				normalize,
				stride,
				offset);
		gl.enableVertexAttribArray(
				programInfo.attribLocations.vertexPosition);
	}

	gl.useProgram(programInfo.program);

	{
		const offset = 0;
		const vertexCount = 4;
		gl.drawArrays(gl.TRIANGLE_STRIP, offset, vertexCount);
	}
}


function initShaderProgram(gl, vsSource, fsSource) {
	const vertexShader = loadShader(gl, gl.VERTEX_SHADER, vsSource);
	const fragmentShader = loadShader(gl, gl.FRAGMENT_SHADER, fsSource);

	// Create the shader program

	const shaderProgram = gl.createProgram();
	gl.attachShader(shaderProgram, vertexShader);
	gl.attachShader(shaderProgram, fragmentShader);
	gl.linkProgram(shaderProgram);

	// If creating the shader program failed, alert

	if (!gl.getProgramParameter(shaderProgram, gl.LINK_STATUS)) {
		alert('Unable to initialize the shader program: ' + gl.getProgramInfoLog(shaderProgram));
		return null;
	}

	return shaderProgram;
}

//
// creates a shader of the given type, uploads the source and
// compiles it.
//
function loadShader(gl, type, source) {
	const shader = gl.createShader(type);

	// Send the source to the shader object

	gl.shaderSource(shader, source);

	// Compile the shader program

	gl.compileShader(shader);

	// See if it compiled successfully

	if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
		alert('An error occurred compiling the shaders: ' + gl.getShaderInfoLog(shader));
		gl.deleteShader(shader);
		return null;
	}

	return shader;
}


function initBuffers(gl) {

	// Create a buffer for the square's positions.

	const positionBuffer = gl.createBuffer();

	// Select the positionBuffer as the one to apply buffer
	// operations to from here out.

	gl.bindBuffer(gl.ARRAY_BUFFER, positionBuffer);

	// Now create an array of positions for the square.

	const positions = [
		-0.5,	0.5,
		 0.5,	0.5,
		-0.5, -0.5,
		 0.5, -0.5,
	];

	// Now pass the list of positions into WebGL to build the
	// shape. We do this by creating a Float32Array from the
	// JavaScript array, then use it to fill the current buffer.

	gl.bufferData(gl.ARRAY_BUFFER,
								new Float32Array(positions),
								gl.STATIC_DRAW);

	return {
		position: positionBuffer,
	};
}
