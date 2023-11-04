#version 430 core

layout (location = 0) in vec3 aPos;

layout (std140, binding = 0) uniform Camera
{
    mat4 projection;
    mat4 view;
};

layout (location = 1) uniform mat4 model;

void main(){
    gl_Position = projection * view * model * vec4(aPos, 1.0);
}