<script lang="ts">
  import "../../styles/main.scss";
  import Presetbox from "../../components/presetbox.svelte";
  import Colorbox from "../../components/colorbox.svelte";
  import Parameterbox from "../../components/parameterbox.svelte";
  import { Button } from "$lib/components/ui/button";
  import animations from "../../../../animations.json";
  import { dev } from "$app/environment";

  import * as Tabs from "$lib/components/ui/tabs";

  let selected_mode = "presets";
  let selection = {
    name: "axis",
    speed: 20,
    colors: ["255;0;0", "0;255;0", "0;0;255"],
    default_colors: ["255;0;0", "0;255;0", "0;0;255"],
    color_names: ["X ass", "Y ass", "Z ass"],
    params: [{ name: "Platums", default: 10, type: "number" }],
  };

  function update_on_server() {//palaist animāciju
    console.log("server", selection);
    let server_url = "";
    if (dev){
      server_url = "http://localhost:3000"
    }
    else{
      server_url = "https://lampinas-server.cvgmerch.lv";
    }
    fetch(server_url + "/requestanim", {
      method: "POST",
      body: JSON.stringify(selection),
      headers: new Headers({ "content-type": "application/json" }),
    });
  }


  function select_preset(index: number) {
    console.log("called")
    console.log(animations[index]);
    selection = {
      name: animations[index].name,
      colors: animations[index].colors,
      default_colors: animations[index].colors,
      speed: animations[index].speed,
      color_names: animations[index].color_names,
      params: [],
    };
    update_on_server();
  }

  function color_change(value: string, index: number) {
    console.log(value);
    selection.colors[index] = value;
    console.log(selection);
    update_on_server();
  }
</script>

<h1>Lampiņu kontrole</h1>
<div class="vis-frame">
  <iframe src={dev ? "http://localhost:5174" : "https://3-d-xmaslights-v3.vercel.app"} frameborder="0" />
</div>

<Tabs.Root value="presets" class="w-full">
  <Tabs.List class="w-full">
    <Tabs.Trigger class="flex-1 w-full" value="presets">Animācijas</Tabs.Trigger>
    <Tabs.Trigger class="flex-1 w-full" value="colors">Krāsas</Tabs.Trigger>
    <Tabs.Trigger class="flex-1 w-full" value="params">Parametri</Tabs.Trigger>
  </Tabs.List>
  <Tabs.Content value="presets">
    <div class="presetgrid">
      {#each animations as animation, i}

        <Presetbox
                {animation}
                selected_name={selection.name}
                onclick={select_preset}
                index={i}
        />
      {/each}
    </div>
  </Tabs.Content>
  <Tabs.Content value="colors">
    {#each selection.color_names as name, i}
      <Colorbox
              rgb={{r: selection.colors[i].split(";")[0], g: selection.colors[i].split(";")[1], b: selection.colors[i].split(";")[2]}}
              value={selection.colors[i]}
              default_value={selection.colors[i]}
              onchange={color_change}
              index={i}
              name={name}
      />
    {/each}
  </Tabs.Content>
  <Tabs.Content value="params">
    <h2 class="text-center">Ātrums</h2>
    <div class="flex flex-col w-48 m-auto">
      <input type="number" bind:value={selection.speed}>
      <Button on:click={update_on_server}>Atjaunināt</Button>
    </div>


  </Tabs.Content>
</Tabs.Root>
