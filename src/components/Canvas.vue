<template>
  <div class="canvas">
    <canvas id="canvas" ref="canvas"></canvas>
  </div>
</template>

<script>
export default {
  name: 'HelloWorld',
  props: {
    height: {
      type: Number,
      default: 350
    },
    width: {
      type: Number,
      default: 350
    }
  },
  data() {
    return {
    }
  },
  mounted() {
    this.$refs.canvas.height = this.height;
    this.$refs.canvas.width  = this.width;

    import('../../pkg').then(rust => {

      const loop = () => {
        rust.refresh();
        requestAnimationFrame(loop);
      }

      loop();
    });
  }
}
</script>

<!-- Add "scoped" attribute to limit CSS to this component only -->
<style scoped>
canvas {
  border: 1px solid black;
}
</style>
