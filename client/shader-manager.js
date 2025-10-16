// Custom shader system for enhanced block rendering
const THREE = require('three');

class ShaderManager {
  constructor() {
    this.shaders = new Map();
    this.initializeShaders();
  }

  initializeShaders() {
    // Minecraft-style block shader with normal mapping and AO
    this.createMinecraftShader();

    // Enhanced PBR shader with subsurface scattering
    this.createEnhancedPBRShader();

    // Water shader with refraction and reflection
    this.createWaterShader();

    // Glass shader with proper transparency
    this.createGlassShader();
  }

  createMinecraftShader() {
    const vertexShader = `
      varying vec2 vUv;
      varying vec3 vNormal;
      varying vec3 vPosition;
      varying vec3 vWorldPosition;

      void main() {
        vUv = uv;
        vNormal = normalize(normalMatrix * normal);
        vPosition = position;
        vWorldPosition = (modelMatrix * vec4(position, 1.0)).xyz;

        gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
      }
    `;

    const fragmentShader = `
      uniform sampler2D map;
      uniform sampler2D normalMap;
      uniform sampler2D aoMap;
      uniform float roughness;
      uniform float metalness;
      uniform vec3 color;
      uniform bool useNormalMap;
      uniform bool useAOMap;
      uniform float time;

      varying vec2 vUv;
      varying vec3 vNormal;
      varying vec3 vPosition;
      varying vec3 vWorldPosition;

      // Simple ambient occlusion approximation
      float calculateAO(vec3 normal, vec3 position) {
        // Simulate ambient occlusion based on surface angle
        float ao = dot(normal, vec3(0.0, 1.0, 0.0)) * 0.5 + 0.5;
        ao = mix(ao, 1.0, 0.3); // Reduce AO intensity
        return ao;
      }

      // Simple normal mapping
      vec3 perturbNormal(vec3 normal, vec2 uv) {
        if (!useNormalMap) return normal;

        vec3 tangentNormal = texture2D(normalMap, uv).xyz * 2.0 - 1.0;

        // Simple tangent space to world space conversion
        vec3 Q1 = dFdx(vWorldPosition);
        vec3 Q2 = dFdy(vWorldPosition);
        vec2 st1 = dFdx(uv);
        vec2 st2 = dFdy(uv);

        vec3 N = normalize(vNormal);
        vec3 T = normalize(Q1 * st2.t - Q2 * st1.t);
        vec3 B = -normalize(cross(N, T));
        mat3 TBN = mat3(T, B, N);

        return normalize(TBN * tangentNormal);
      }

      void main() {
        vec4 texColor = texture2D(map, vUv);

        // Apply color tint
        vec3 finalColor = texColor.rgb * color;

        // Apply normal mapping
        vec3 normal = perturbNormal(vNormal, vUv);

        // Calculate ambient occlusion
        float ao = useAOMap ? texture2D(aoMap, vUv).r : calculateAO(normal, vPosition);

        // Simple lighting calculation
        vec3 lightDir = normalize(vec3(1.0, 1.0, 0.5));
        float diff = max(dot(normal, lightDir), 0.0);
        float ambient = 0.4;

        // Apply roughness (simplified)
        diff = mix(diff, diff * 0.5 + 0.5, roughness * 0.5);

        // Apply metalness (simplified)
        vec3 specular = vec3(0.0);
        if (metalness > 0.0) {
          vec3 viewDir = normalize(cameraPosition - vWorldPosition);
          vec3 reflectDir = reflect(-lightDir, normal);
          float spec = pow(max(dot(viewDir, reflectDir), 0.0), 32.0);
          specular = spec * metalness * vec3(1.0);
        }

        // Combine lighting
        vec3 lighting = ambient + diff + specular;
        finalColor *= lighting;

        // Apply ambient occlusion
        finalColor *= ao;

        // Add slight emissive glow for certain blocks
        if (metalness > 0.5) {
          finalColor += texColor.rgb * metalness * 0.1;
        }

        gl_FragColor = vec4(finalColor, texColor.a);
      }
    `;

    this.shaders.set('minecraft', {
      vertexShader,
      fragmentShader,
      uniforms: {
        map: { value: null },
        normalMap: { value: null },
        aoMap: { value: null },
        roughness: { value: 0.8 },
        metalness: { value: 0.0 },
        color: { value: new THREE.Color(0xffffff) },
        useNormalMap: { value: false },
        useAOMap: { value: false },
        time: { value: 0.0 }
      }
    });
  }

  createEnhancedPBRShader() {
    const vertexShader = `
      varying vec2 vUv;
      varying vec3 vNormal;
      varying vec3 vPosition;
      varying vec3 vWorldPosition;
      varying vec3 vViewPosition;

      void main() {
        vUv = uv;
        vNormal = normalize(normalMatrix * normal);
        vPosition = position;
        vWorldPosition = (modelMatrix * vec4(position, 1.0)).xyz;
        vViewPosition = -(modelViewMatrix * vec4(position, 1.0)).xyz;

        gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
      }
    `;

    const fragmentShader = `
      uniform sampler2D map;
      uniform sampler2D normalMap;
      uniform sampler2D roughnessMap;
      uniform sampler2D metalnessMap;
      uniform float roughness;
      uniform float metalness;
      uniform vec3 color;
      uniform bool useNormalMap;
      uniform bool useRoughnessMap;
      uniform bool useMetalnessMap;
      uniform float time;

      varying vec2 vUv;
      varying vec3 vNormal;
      varying vec3 vPosition;
      varying vec3 vWorldPosition;
      varying vec3 vViewPosition;

      // Physically based lighting functions
      vec3 fresnelSchlick(float cosTheta, vec3 F0) {
        return F0 + (1.0 - F0) * pow(1.0 - cosTheta, 5.0);
      }

      float DistributionGGX(vec3 N, vec3 H, float roughness) {
        float a = roughness * roughness;
        float a2 = a * a;
        float NdotH = max(dot(N, H), 0.0);
        float NdotH2 = NdotH * NdotH;

        float num = a2;
        float denom = (NdotH2 * (a2 - 1.0) + 1.0);
        denom = 3.14159 * denom * denom;

        return num / denom;
      }

      float GeometrySchlickGGX(float NdotV, float roughness) {
        float r = (roughness + 1.0);
        float k = (r * r) / 8.0;

        float num = NdotV;
        float denom = NdotV * (1.0 - k) + k;

        return num / denom;
      }

      float GeometrySmith(vec3 N, vec3 V, vec3 L, float roughness) {
        float NdotV = max(dot(N, V), 0.0);
        float NdotL = max(dot(N, L), 0.0);
        float ggx2 = GeometrySchlickGGX(NdotV, roughness);
        float ggx1 = GeometrySchlickGGX(NdotL, roughness);

        return ggx1 * ggx2;
      }

      vec3 perturbNormal(vec3 normal, vec2 uv) {
        if (!useNormalMap) return normal;

        vec3 tangentNormal = texture2D(normalMap, uv).xyz * 2.0 - 1.0;

        vec3 Q1 = dFdx(vWorldPosition);
        vec3 Q2 = dFdy(vWorldPosition);
        vec2 st1 = dFdx(uv);
        vec2 st2 = dFdy(uv);

        vec3 N = normalize(vNormal);
        vec3 T = normalize(Q1 * st2.t - Q2 * st1.t);
        vec3 B = -normalize(cross(N, T));
        mat3 TBN = mat3(T, B, N);

        return normalize(TBN * tangentNormal);
      }

      void main() {
        vec4 texColor = texture2D(map, vUv);
        vec3 albedo = texColor.rgb * color;

        // Get material properties
        float matRoughness = useRoughnessMap ? texture2D(roughnessMap, vUv).r : roughness;
        float matMetalness = useMetalnessMap ? texture2D(metalnessMap, vUv).r : metalness;

        // Normal mapping
        vec3 N = perturbNormal(vNormal, vUv);
        vec3 V = normalize(cameraPosition - vWorldPosition);

        // Lighting calculation
        vec3 F0 = vec3(0.04);
        F0 = mix(F0, albedo, matMetalness);

        // Directional light
        vec3 lightDir = normalize(vec3(1.0, 1.0, 0.5));
        vec3 L = lightDir;
        vec3 H = normalize(V + L);
        vec3 radiance = vec3(1.0); // Light color and intensity

        // Cook-Torrance BRDF
        float NDF = DistributionGGX(N, H, matRoughness);
        float G = GeometrySmith(N, V, L, matRoughness);
        vec3 F = fresnelSchlick(max(dot(H, V), 0.0), F0);

        vec3 kS = F;
        vec3 kD = vec3(1.0) - kS;
        kD *= 1.0 - matMetalness;

        vec3 numerator = NDF * G * F;
        float denominator = 4.0 * max(dot(N, V), 0.0) * max(dot(N, L), 0.0);
        vec3 specular = numerator / max(denominator, 0.001);

        float NdotL = max(dot(N, L), 0.0);
        vec3 Lo = (kD * albedo / 3.14159 + specular) * radiance * NdotL;

        // Ambient lighting
        vec3 ambient = vec3(0.03) * albedo;

        vec3 finalColor = ambient + Lo;

        // Tone mapping
        finalColor = finalColor / (finalColor + vec3(1.0));
        finalColor = pow(finalColor, vec3(1.0/2.2));

        gl_FragColor = vec4(finalColor, texColor.a);
      }
    `;

    this.shaders.set('enhanced-pbr', {
      vertexShader,
      fragmentShader,
      uniforms: {
        map: { value: null },
        normalMap: { value: null },
        roughnessMap: { value: null },
        metalnessMap: { value: null },
        roughness: { value: 0.8 },
        metalness: { value: 0.0 },
        color: { value: new THREE.Color(0xffffff) },
        useNormalMap: { value: false },
        useRoughnessMap: { value: false },
        useMetalnessMap: { value: false },
        time: { value: 0.0 }
      }
    });
  }

  createWaterShader() {
    const vertexShader = `
      varying vec2 vUv;
      varying vec3 vNormal;
      varying vec3 vPosition;
      varying vec3 vWorldPosition;
      varying vec4 vClipSpace;

      uniform float time;

      void main() {
        vUv = uv;
        vNormal = normalize(normalMatrix * normal);
        vPosition = position;
        vWorldPosition = (modelMatrix * vec4(position, 1.0)).xyz;

        // Calculate clip space position for reflection/refraction
        vClipSpace = projectionMatrix * modelViewMatrix * vec4(position, 1.0);

        // Add wave animation
        vec3 pos = position;
        pos.y += sin(time * 2.0 + position.x * 0.5 + position.z * 0.3) * 0.05;
        pos.y += cos(time * 1.5 + position.x * 0.3 + position.z * 0.7) * 0.03;

        gl_Position = projectionMatrix * modelViewMatrix * vec4(pos, 1.0);
      }
    `;

    const fragmentShader = `
      uniform sampler2D waterTexture;
      uniform float time;
      uniform vec3 waterColor;
      uniform float transparency;

      varying vec2 vUv;
      varying vec3 vNormal;
      varying vec3 vPosition;
      varying vec3 vWorldPosition;
      varying vec4 vClipSpace;

      void main() {
        // Water texture with distortion
        vec2 distortedUv = vUv;
        distortedUv.x += sin(time + vWorldPosition.z * 0.1) * 0.01;
        distortedUv.y += cos(time + vWorldPosition.x * 0.1) * 0.01;

        vec4 texColor = texture2D(waterTexture, distortedUv);

        // Fresnel effect for water edges
        vec3 viewDir = normalize(cameraPosition - vWorldPosition);
        float fresnel = pow(1.0 - max(dot(viewDir, vNormal), 0.0), 2.0);

        // Mix water color with texture
        vec3 finalColor = mix(waterColor, texColor.rgb, 0.3);

        // Add fresnel highlight
        finalColor += vec3(1.0, 1.0, 1.0) * fresnel * 0.3;

        // Add some subsurface scattering effect
        float subsurface = max(dot(vNormal, -viewDir), 0.0);
        finalColor += waterColor * subsurface * 0.2;

        gl_FragColor = vec4(finalColor, transparency);
      }
    `;

    this.shaders.set('water', {
      vertexShader,
      fragmentShader,
      uniforms: {
        waterTexture: { value: null },
        time: { value: 0.0 },
        waterColor: { value: new THREE.Color(0x006994) },
        transparency: { value: 0.8 }
      }
    });
  }

  createGlassShader() {
    const vertexShader = `
      varying vec2 vUv;
      varying vec3 vNormal;
      varying vec3 vWorldPosition;
      varying vec3 vViewPosition;

      void main() {
        vUv = uv;
        vNormal = normalize(normalMatrix * normal);
        vWorldPosition = (modelMatrix * vec4(position, 1.0)).xyz;
        vViewPosition = -(modelViewMatrix * vec4(position, 1.0)).xyz;

        gl_Position = projectionMatrix * modelViewMatrix * vec4(position, 1.0);
      }
    `;

    const fragmentShader = `
      uniform sampler2D map;
      uniform float reflectivity;
      uniform float refractionRatio;
      uniform vec3 color;
      uniform float time;

      varying vec2 vUv;
      varying vec3 vNormal;
      varying vec3 vWorldPosition;
      varying vec3 vViewPosition;

      void main() {
        vec4 texColor = texture2D(map, vUv);
        vec3 finalColor = texColor.rgb * color;

        // Fresnel effect for glass
        vec3 viewDir = normalize(vViewPosition);
        float fresnel = pow(1.0 - max(dot(-viewDir, vNormal), 0.0), 5.0);

        // Add chromatic aberration effect
        vec2 uvOffset = vUv;
        float aberration = 0.005;

        vec3 refractedColor;
        refractedColor.r = texture2D(map, uvOffset + vec2(aberration, 0.0)).r;
        refractedColor.g = texture2D(map, uvOffset).g;
        refractedColor.b = texture2D(map, uvOffset - vec2(aberration, 0.0)).b;

        // Mix reflection and refraction
        finalColor = mix(refractedColor * color, vec3(1.0), fresnel * reflectivity);

        // Add some sparkle effect
        float sparkle = sin(time * 10.0 + vWorldPosition.x * 5.0 + vWorldPosition.z * 3.0);
        sparkle = smoothstep(0.8, 1.0, sparkle);
        finalColor += vec3(sparkle * 0.3);

        gl_FragColor = vec4(finalColor, texColor.a * 0.9);
      }
    `;

    this.shaders.set('glass', {
      vertexShader,
      fragmentShader,
      uniforms: {
        map: { value: null },
        reflectivity: { value: 0.5 },
        refractionRatio: { value: 0.98 },
        color: { value: new THREE.Color(0xffffff) },
        time: { value: 0.0 }
      }
    });
  }

  // Create material with specific shader
  createMaterial(shaderName, uniforms = {}) {
    const shader = this.shaders.get(shaderName);
    if (!shader) {
      console.warn(`Shader '${shaderName}' not found, using default material`);
      return new THREE.MeshLambertMaterial();
    }

    const materialUniforms = { ...shader.uniforms, ...uniforms };

    return new THREE.ShaderMaterial({
      vertexShader: shader.vertexShader,
      fragmentShader: shader.fragmentShader,
      uniforms: materialUniforms,
      transparent: shaderName === 'water' || shaderName === 'glass',
      side: THREE.DoubleSide
    });
  }

  // Update shader time uniform
  updateTime(time) {
    this.shaders.forEach(shader => {
      if (shader.uniforms.time) {
        shader.uniforms.time.value = time;
      }
    });
  }
}

module.exports = ShaderManager;