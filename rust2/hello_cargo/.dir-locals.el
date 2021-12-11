((rust-mode
  . (
     (lsp-docker+-server-id . rust-analyzer)
     (lsp-docker+-docker-server-id . rust-analyzer-lsp-docker)
     (lsp-docker+-server-command . "rust-analyzer")
     (lsp-docker+-image-id . "rust_rust")
     (lsp-docker+-container-name . "rust-lsp-docker")
     (lsp-docker+-path-mappings . (("/home/kitamura/" . "/home/kitamura/")))
     )
  )
 )
