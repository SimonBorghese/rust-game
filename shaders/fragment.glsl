#version 430 core

out vec4 FragColor;

in vec3 oVex;

void main(){
    FragColor = vec4(oVex, 1.0f);
}