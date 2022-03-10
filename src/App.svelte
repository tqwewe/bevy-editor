

<script lang="ts">
	import Tree from './components/Tree.svelte';
	import Viewport from './components/Viewport.svelte';
</script>

<div class="container">
	<Tree />
	<Viewport />
</div>

<style>
	:global(body) {
		background-color: rgb(30, 30, 30);
		margin: 0;
		padding: 0;
	}

	.container {
		display: flex;
		margin: 0 auto;
		height: 100vh;
	}
</style>