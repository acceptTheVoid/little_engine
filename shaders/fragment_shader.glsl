#version 330 core
out vec4 FragColor;
  
in vec3 ourColor; // we set this variable in the OpenGL code.
in vec2 TexCoord;

uniform sampler2D texture1;
uniform sampler2D texture2;

void main()
{
    // FragColor = texture(texture2, TexCoord);
    FragColor = mix(texture(texture1, TexCoord), texture(texture2, TexCoord), 0.2) * vec4(ourColor, 1.0);
}