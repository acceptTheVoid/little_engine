#version 330 core
out vec4 FragColor;
  
in vec3 ourColor; // we set this variable in the OpenGL code.
in vec2 TexCoord;

uniform sampler2D texture1;

void main()
{
    // FragColor = texture(texture2, TexCoord);
    FragColor = texture(texture1, TexCoord) * vec4(ourColor, 1.0);
}