# An example using pagefind to index a json file in rust.

First builds the pagefind index based off the `data.json` file.

```sh
cargo run
```

This should make a folder in `/static/pagefind` with the index files.

Then you can view the index.html file with reload.

```sh
npm install -g reload
reload -p --port 3000
```

Then open `http://localhost:3000` in your browser. You will see that the search bar is using the data.json file to search for the data.



