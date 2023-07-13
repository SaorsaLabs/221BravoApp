<script>
    export let type = 'standard'; // standard, grey, white, orange
    export let flat = false;
    export let noBG = false;
    export let slim = false;

  </script>
  <button class:slim={slim} class:flat={flat} class:noBG={noBG} class={type} on:click>
    <slot></slot>
  </button>
  
  <style>
        /* OLD  #31848f, #06beb6 */
        /* NEW  #89318f, #06beb6 */
    button{
      border: 0;
      cursor: pointer;
      border-radius: 4px;
      padding: 4px 6px;
      font-weight: bold;
      box-shadow: 1px 2px 3px rgba(0,0,0,0.2);
    }
    .flat{
      box-shadow: none;
    }
    .standard{
      background: #b2b37f;
      color: white;
    }
    button.standard:hover {
      color: rgb(255, 255, 255);
      background: #89318f;  /* fallback for old browsers */
		  background: -webkit-linear-gradient(to right, #89318f, #09cbf1);  /* Chrome 10-25, Safari 5.1-6 */
		  background: linear-gradient(to right, #89318f, #09cbf1); /* W3C, IE 10+/ Edge, Firefox 16+, Chrome 26+, Opera 12+, Safari 7+ */
    }
    button.standard:active {
      color: rgb(223, 216, 126);
      border-style:solid;
      background: #89318f;  /* fallback for old browsers */
		  background: -webkit-linear-gradient(to right, #89318f, #09cbf1);  /* Chrome 10-25, Safari 5.1-6 */
		  background: linear-gradient(to right, #89318f, #09cbf1); /* W3C, IE 10+/ Edge, Firefox 16+, Chrome 26+, Opera 12+, Safari 7+ */
    }
    .white{
      background: #ffffff;
      color: rgb(0, 0, 0);
    }
    button.white:hover {
      color: rgb(255, 255, 255);
      background: #89318f;  /* fallback for old browsers */
		  background: -webkit-linear-gradient(to right, #89318f, #09cbf1);  /* Chrome 10-25, Safari 5.1-6 */
		  background: linear-gradient(to right, #89318f, #09cbf1); /* W3C, IE 10+/ Edge, Firefox 16+, Chrome 26+, Opera 12+, Safari 7+ */
    }
    button.white:active {
      color: rgb(223, 216, 126);
      border-style:solid;
      background: #89318f;  /* fallback for old browsers */
		  background: -webkit-linear-gradient(to right, #89318f, #09cbf1);  /* Chrome 10-25, Safari 5.1-6 */
		  background: linear-gradient(to right, #89318f, #09cbf1); /* W3C, IE 10+/ Edge, Firefox 16+, Chrome 26+, Opera 12+, Safari 7+ */
    }
    .grey{
      background: #565656;
      border: #8f8f8f;
      border-width: 1px;
      border-style: solid;
      color: white;
    }
    button.grey:hover {
      background: #89318f;  /* fallback for old browsers */
		  background: -webkit-linear-gradient(to right, #89318f, #09cbf1);  /* Chrome 10-25, Safari 5.1-6 */
		  background: linear-gradient(to right, #89318f, #09cbf1); /* W3C, IE 10+/ Edge, Firefox 16+, Chrome 26+, Opera 12+, Safari 7+ */
    }
    button.grey:active {
      /* background-color: yellow; */
      background: #06beb6;  /* fallback for old browsers */
		  background: -webkit-linear-gradient(to right, #89318f, #06beb6);  /* Chrome 10-25, Safari 5.1-6 */
		  background: linear-gradient(to right, #89318f, #06beb6); /* W3C, IE 10+/ Edge, Firefox 16+, Chrome 26+, Opera 12+, Safari 7+ */
    }
    .orange{
      background: #dba604;
      border: #8f8f8f;
      border-width: 1px;
      border-style: solid;
      color: white;
    }
    button.orange:hover {
      background: #89318f;  /* fallback for old browsers */
		  background: -webkit-linear-gradient(to right, #f1e909, #b78d00);  /* Chrome 10-25, Safari 5.1-6 */
		  background: linear-gradient(to right, #f1e909, #b78d00); /* W3C, IE 10+/ Edge, Firefox 16+, Chrome 26+, Opera 12+, Safari 7+ */
    }
    button.orange:active {
      /* background-color: yellow; */
      background: #89318f;  /* fallback for old browsers */
		  background: -webkit-linear-gradient(to right, #f1e909, #b78d00);  /* Chrome 10-25, Safari 5.1-6 */
		  background: linear-gradient(to right, #f1e909, #b78d00); /* W3C, IE 10+/ Edge, Firefox 16+, Chrome 26+, Opera 12+, Safari 7+ */
      color: rgb(74, 97, 141);
    }
    .head{
      color: white;
      background: none;
    }
    button.head:hover {
      color: #09cbf1;  
      background: -webkit-linear-gradient(#89318f, #09cbf1);
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
    }
    button.head:active {
     color:#bdb76b;
      background: -webkit-linear-gradient(#788f31, #bdb76b);
      -webkit-background-clip: text;
      -webkit-text-fill-color: transparent;
    }

    .noBG{
      background: none;
    }
    .regular{
      font-size: 1em;
    }
    .slim{
      padding: 0px;
      padding-left: 3px;
      padding-right: 3px;
    }

  </style>