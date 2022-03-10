<script lang="ts" context="module">
	import init from '../../public/build/viewport';
	init();
</script>

<div class="container">

  <canvas id="viewport"></canvas>
</div>

<style>
  .container {
    position: relative;
    flex: 1;
  }

	#viewport {
    position: absolute;
    top: 0;
    left: 0;
    width: 100% !important;
    height: 100% !important;
		background-color: white;
	}

  #viewport:focus {
    outline: none;
  }
</style>
