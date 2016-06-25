#version 140
in uint vertexdata;
uniform mat4 mvp;

out vec2 v_tex_coords;

void main() {
	uint bitmask = uint(63); //AND with this to get lowest six bits of data.
	//Extract X:
	float x = float(vertexdata & bitmask);
	//Extract Y:
	float y = float((vertexdata >> 6) & bitmask);
	//Extract Z:
	float z = float((vertexdata >> 12) & bitmask);
	
	bitmask = uint (1); //Lowest one bit time.
	float u = float((vertexdata >> 30) & bitmask);
	float v = float((vertexdata >> 31) & bitmask);
	
	v_tex_coords.x = u;
	v_tex_coords.y = v;
	
    gl_Position = mvp * vec4(x, y, z, 1.0);
}