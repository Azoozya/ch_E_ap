<!DOCTYPE html>
<html>
  <head>
    <meta charset="utf-8">
    <title>hello-wasm example</title>
  </head>
  <body>
    <script type="module">
      import init, {greet} from "./wasm.js";
      init()
        .then(() => {
         generate()
        });
    </script>
  </body>
</html>
