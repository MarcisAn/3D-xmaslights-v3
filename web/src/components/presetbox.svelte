<script>
  /**
   * @type {{ f_name: string; name: string;}}
   */
  export let animation;
  /**
   * @type {string}
   */
  export let selected_name;
  /**
   * @type {(index: number) => any}
   */
  export let onclick;
  /**
   * @type {number}
   */
  export let index;
</script>

<!-- svelte-ignore a11y-click-events-have-key-events -->
<!-- svelte-ignore a11y-no-static-element-interactions -->
<div
  class={animation.name == selected_name ? "selected-presetbox" : "presetbox"}
  on:click={onclick(index)}
>
  <div class="gif">
    <!-- svelte-ignore a11y-media-has-caption -->
    <img src={"/animation-previews/"+animation.name+".gif"} alt="">
  </div>
  {animation.f_name}
</div>
