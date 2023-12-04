<script>
  import {onMount} from "svelte";
  import { Label } from "$lib/components/ui/label";
  import { Switch } from "$lib/components/ui/switch";
  let is_simple = true;
  export let name = "";
  export let default_value = "";

  export let value = "";
  /**
   * @type {(value: string, index: number) => any}
   */
  export let onchange;
  /**
   * @type {number}
   */
  export let index;
  import ColorPicker from "svelte-awesome-color-picker";
  import { Button } from "$lib/components/ui/button";
  /**
   * @type {any}
   */
  export let rgb = {
    //just for simple colors
    r: value.split(";")[0],
    g: value.split(";")[1],
    b: value.split(";")[2],
  }; // or hsv or hex
  let rgb_fn = {
    r: "100",
    g: "0",
    b: "0",
  };
  let hsv_fn = {
    h: "0",
    s: "100",
    v: "100",
  };
  let is_hsv = false;
  //style={"background-color: rgb(" + rgb.r + "," + rgb.g + "," + rgb.b + ")"}

  function changeColor() {
    rgb = {
      //just for simple colors
      r: default_value.split(";")[0],
      g: default_value.split(";")[1],
      b: default_value.split(";")[2],
    };
    if (is_simple) {
      onchange(rgb.r + ";" + rgb.g + ";" + rgb.b, index);
    } else {
      if (is_hsv) {
        onchange("@HSV@" + hsv_fn.h + ";" + hsv_fn.s + ";" + hsv_fn.v, index);
      } else {
        onchange("@RGB@" + rgb_fn.r + ";" + rgb_fn.g + ";" + rgb_fn.b, index);
      }
    }
  }
  onMount(() => {rgb = {
    //just for simple colors
    r: default_value.split(";")[0],
    g: default_value.split(";")[1],
    b: default_value.split(";")[2],
  };})
</script>

<div class="colorbox">
  <div class="color-type-selector-btn">
    <h2>{name}</h2>
      {#if is_simple}
      <button
        on:click={() => {
          is_simple = false;
          changeColor();
        }}>Pārslēgties uz krāsas funkciju</button
      >
    {:else}
      <button
        on:click={() => {
          is_simple = true;
          changeColor();
        }}>Pārslēgties uz vienkāršu krāsu</button
      >
    {/if}
  </div>
  {#if is_simple}
    <div class="color">
      <ColorPicker
        bind:rgb
        isOpen={true}
        disableCloseClickOutside={true}
        label={""}
        isAlpha={false}
        isInput={false}
        isTextInput={false}
      />
      <div
        class="color-preview"
        style={"background-color: rgb(" +
          rgb.r +
          "," +
          rgb.g +
          "," +
          rgb.b +
          ")"}
      />
    </div>
    <span>
      <Button on:click={() => {
          onchange(rgb.r + ";" + rgb.g + ";" + rgb.b, index);
        }}>
        Atsvaidzināt
      </Button>

    </span>
  {:else}
    <div class="discription">
      <p>
        Krāsas funkcija dod iespēju animācijā redzamās krāsas izteikt ar
        matemātisku izteiksmi
      </p>
      <p>
        Izteiksmēm zemāk esošajos 3 lodziņos ir jāizvada skaitlis intervālā
        0..100
      </p>
      <p>
        Krāsas tiek miksētas RGB vai HSV sistēmā. RGB sistēmā katra izteiksme
        nosaka sarkanās, zaļās un zilās krāsas daudzumu krāsā, bet HSV sistēmā
        katra izteiksme nosaka krāsas toni, piesātinājumu un spilgtumu. Šīs
        izteiksmes tiek izpildītas katrai lampiņai, katrā animācijas kadrā.
      </p>
      Pieejami šādi argumenti(mainīgie):
      <ul>
        <li>
          <strong>x</strong> - lampiņas x koordināta [0;100]
        </li>
        <li><strong>y</strong> - lampiņas y koordināta [0;100]</li>
        <li><strong>z</strong> - lampiņas z koordināta [0;100]</li>
      </ul>
      Pieejamas šādas funkcijas:
      <ul>
        <li>
          <strong>sin(x)</strong> - sīnusa funkcija, kas izvada vērtību [0;100].
          Lai effekts būtu redzams, ieteicams funkcijas argumentu dalīt ar lielu
          skaitli
        </li>
        <li><strong>ks(x)</strong> - kvadrātsakne no x</li>
        <li>
          <strong>rnd(a1, a2)</strong> - nejauši ģenerēts skaitlis robežās [a1;a2]
        </li>
      </ul>
      Matemātiskas darbības:
      <ul>
        <li><strong>+</strong> - saskaitīšana</li>
        <li><strong>-</strong> - atņemšana</li>
        <li><strong>*</strong> - reizināšana</li>
        <li><strong>/</strong> - dalīšana</li>
        <li><strong>^</strong> - kāpināšana</li>
      </ul>
    </div>
    {#if is_hsv}
      <button on:click={() => (is_hsv = false)}>Pārslēgties RGB sistēmu</button>
      <label>
        H
        <input type="text" bind:value={hsv_fn.h} />
      </label>
      <label>
        S
        <input type="text" bind:value={hsv_fn.s} />
      </label>
      <label>
        V
        <input type="text" bind:value={hsv_fn.v} />
      </label>
    {:else}
      <button on:click={() => (is_hsv = true)}
        >Pārslēgties uz HSV sistēmu</button
      >
      <label>
        R
        <input type="text" bind:value={rgb_fn.r} />
      </label>
      <label>
        G
        <input type="text" bind:value={rgb_fn.g} />
      </label>
      <label>
        B
        <input type="text" bind:value={rgb_fn.b} />
      </label>
    {/if}
    <span>
      <Button on:click={changeColor}>Atsvaidzināt</Button>
    </span>{/if}
</div>
