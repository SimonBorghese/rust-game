#version 430 core

out vec4 FragColor;

in vec3 oVex;
in vec2 oTex;

layout (location = 1) uniform sampler2D tex;

void main(){
    FragColor = texture(tex, oTex);
}