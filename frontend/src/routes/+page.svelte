<script>
    import { onMount } from 'svelte';

    let shaderCode;

    async function fetchShader() {
        try {
            const response = await fetch('http://127.0.0.1:3000/shader/basic');
            const blob = await response.arrayBuffer();
            shaderCode = new Uint32Array(blob);
            console.log("Shader code loaded:", shaderCode);
        } catch (error) {
            console.error("Failed to load shader:", error);
        }
    }

    onMount(fetchShader);
</script>

<main>
    <h1>3D Contribution Grid</h1>
    <canvas id="webgpuCanvas"></canvas>
</main>

<style>
    canvas { width: 100%; height: 100%; }
</style>
