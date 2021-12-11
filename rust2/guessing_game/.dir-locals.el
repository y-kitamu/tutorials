;;; Directory Local Variables
;;; For more information see (info "(emacs) Directory Variables")

((rust-mode
  . ((lsp-docker+-server-id . rust-analyzer)
     (lsp-docker+-docker-server-id . rust-analyzer-lsp-docker)
     (lsp-docker+-server-command . "rust-analyzer")
     (lsp-docker+-docker-options . "-u ${USER}")
     (lsp-docker+-image-id . "rust_rust")
     (lsp-docker+-container-name . "rust_rust_1")
     (lsp-docker+-server-cmd-fn . lsp-docker+-exec-in-container)
     (lsp-docker+-path-mappings . (("${HOME}/work" . "${HOME}/work"))))))
