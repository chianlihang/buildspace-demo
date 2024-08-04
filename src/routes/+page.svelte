<script lang="ts">
  let x = 1;
  let videoSrc = `/uploads/${x}.mp4`;
  let videoElement;
  console.log("rn: " + x)

  const nextvid = () => {
    x += 2;
    videoSrc = `/uploads/${x}.mp4`;
    videoElement.load(); // Reload the video with the new source
    videoElement.play(); // Start playing the new video
    console.log("new: " + x)
  };

  function makeFullscreen() {
    if (videoElement.requestFullscreen) {
      videoElement.requestFullscreen();
    } else if (videoElement.mozRequestFullScreen) { // Firefox
      videoElement.mozRequestFullScreen();
    } else if (videoElement.webkitRequestFullscreen) { // Chrome, Safari, and Opera
      videoElement.webkitRequestFullscreen();
    } else if (videoElement.msRequestFullscreen) { // IE/Edge
      videoElement.msRequestFullscreen();
    }
  }
</script>

<div class="container">
  <video
    bind:this={videoElement}
    on:play={makeFullscreen}
    on:ended={nextvid}
    controls
    autoplay
  >
    <source
      src={videoSrc}
      type="video/mp4"
    />
  </video>
</div>

<style>
  html, body {
    margin: 0;
    padding: 0;
    width: 100%;
    height: 100%;
    overflow: hidden;
  }

  .container {
    display: flex;
    justify-content: center;
    align-items: center;
    width: 100%;
    height: 100vh;
    background-color: black; /* Optional: for better contrast */
  }

  video {
    width: 100%;
    height: 100%;
    object-fit: cover; /* Ensure video covers the entire container */
  }
</style>