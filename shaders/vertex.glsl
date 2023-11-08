#version 430 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in vec3 aNorm;
layout (location = 2) in vec2 aTex;


layout (std140, binding = 0) uniform Camera
{
    mat4 projection;
    mat4 view;
};

layout (location = 0) uniform mat4 model;

out vec3 oVex;
out vec2 oTex;

void main(){
    oVex = aPos;
    oTex = aTex;
    gl_Position = projection * view * model * vec4(aPos, 1.0);
}