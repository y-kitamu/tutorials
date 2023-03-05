declare var lisp: any;

const insertIntoTypeScriptBuffer = (str: string) => {
  const buffer = lisp.get_buffer_create("TypeScript FileWatching");
  lisp.with_current_buffer(buffer, () => lisp.insert(`${str}\n`));
};

const watch = async (dir: string) => {
  lisp.print(dir);
  const watcher = Deno.watchFs(dir);
  let i = 0;
  for await (const event of watcher) {
    i += 1;
    if (i > 5) break;
    insertIntoTypeScriptBuffer(JSON.stringify(event));
  }
};

watch("/home/kitamura/work/Learning/Tutorials/emacs_deno/");
