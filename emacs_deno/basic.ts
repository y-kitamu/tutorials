declare var lisp: any;

const x: string = "Hello Typescript"; //
lisp.print(x);

fetch("https://api.github.com/users/denoland")
  .then((response) => response.json())
  .then((data) => {
    const buffer = lisp.get_buffer_create("typescript Buffer");
    lisp.with_current_buffer(buffer, () => lisp.insert(JSON.stringify(data)));
  })
  .catch((e) => lisp.print(JSON.stringify(e)));
