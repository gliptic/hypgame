//FabriceNeyret's version in 56 chars
#define mainImage(o,i)  o+=90.*fract(dot(sin(i),i))-89.-o

//old version by Fabrice Neyret :
//void mainImage(out vec4 o,vec2 i){o+=90.*fract(dot(sin(i),i))-89.-o;}

//or animated : 
//void mainImage(out vec4 o,vec2 i){o=9.*fract(dot(sin(i),i)+iDate.wwww*.1)-8.;}
   
//original :

/*
void mainImage(out vec4 fragColor,in vec2 fragCoord){	
    vec2 x=fragCoord.xy;
	vec3 a=vec3(max((fract(dot(sin(x),x))-.99)*90.,.0));
    fragColor=vec4(a,1.);
}
*/