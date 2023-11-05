<script>
    import { onMount } from 'svelte';
    import { initialise } from '../../../../../node_modules/@open-ic/openchat-xframe/lib/index.js';
  
    let iframe;
    
    onMount(async () => {
      const client = await initialise(iframe, {
        targetOrigin: 'https://oc.app',
        initialPath: '/group/zzipy-eaaaa-aaaar-adnqa-cai/?ref=zx4ws-zqaaa-aaaar-adsuq-cai&code=717f4c493c799554', // /community/rfeib-riaaa-aaaar-ar3oq-cai/channel/334961401678552956581044255076222828441
        theme: {
          name: 'my-app-theme',
          base: 'dark',
          overrides: {
            primary: "blue",
            bd: 'rgb(150, 150, 150)',
            bg: 'transparent',
            txt: "white",
            placeholder: "aqua",
            'txt-light': '#75c8af',
            timeline: {
              txt: "orange"
            },
            // ... more theme overrides
          }
        }
      });
    });
  </script>
  
  <div class="chat">
    <div class="header">
    </div>
    
    <iframe bind:this={iframe} title="OpenChat" frameborder="0" class="chat-iframe"/>
  </div>

  <style>
      .chat-iframe {
      width: 100%;
      height: 80vh;
    }
  </style>