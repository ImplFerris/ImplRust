# implrust.com

This repository contains the source code for the https://implrust.com website.

This website is written with Dioxus, pre-generated with `dioxus_ssr`, and then
rehydrated with interactivity provided by `dioxus_web`.

## Development

The docsite uses the newest Tailwind (v4) so you need to install it first and then run it using:

```sh
cd packages/implrust
npx @tailwindcss/cli -i ../../tailwind.css -o ./assets/tailwind.css --watch
```


```sh
cargo install dioxus-cli
```

With [`dx`][dx] installed, you can use it to build and serve the website
on your local system:

```sh
cd packages/implrust
dx serve
```

This will start a local server that will be available on
[localhost:8080](localhost:8080) and will automatically build and re-build the
documentation when it changes.

## Credits

The source code is based on the official Dioxus [Docsite](https://github.com/DioxusLabs/docsite) repository, which serves as a great example of using Markdown to generate pages while leveraging Dioxus for the frontend.
