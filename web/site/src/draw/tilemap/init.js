export var state = {};

export function init() {
	var gl = e2.gl;
	state.gl = gl;

	const vsSource = `
		attribute vec2 aVertexPosition;

		void main() {
			gl_Position = vec4(aVertexPosition, 0, 1);
		}
	`;

	const fsSource = `
		precision mediump float;
		uniform sampler2D uMapSampler;

		void main() {
			// TODO make resolution independent
			vec2 p = vec2(gl_FragCoord.x/1920.0, gl_FragCoord.y/1080.0);
			vec3 texel = texture2D(uMapSampler, p).xyz;
			gl_FragColor = vec4(texel, 1.0);
		}
	`;

	const shaderProgram = initShaderProgram(gl, vsSource, fsSource);
	state.programInfo = {
		program: shaderProgram,
		attribLocations: {
			vertexPosition: gl.getAttribLocation(shaderProgram, 'aVertexPosition'),
		},
		uniformLocations: {
			uMapSampler: gl.getUniformLocation(shaderProgram, 'uMapSampler'),
		},
	};

	const vsBuffer = gl.createBuffer();
	gl.bindBuffer(gl.ARRAY_BUFFER, vsBuffer);

	const r = [
		-1.0, -1.0,
		 1.0, -1.0,
		-1.0,  1.0,
		 1.0,  1.0,
	];
	state.gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(r), gl.STATIC_DRAW);
	state.vsBuffer = vsBuffer;

	state.mapTexture = gl.createTexture();

	gl.bindTexture(gl.TEXTURE_2D, state.mapTexture);
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE);
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE);
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.NEAREST);
	gl.texParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.NEAREST);
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
