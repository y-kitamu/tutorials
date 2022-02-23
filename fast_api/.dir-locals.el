;;; Directory Local Variables
;;; For more information see (info "(emacs) Directory Variables")

((web-mode
  . ((lsp-docker+-client-configs
      . ((:server-id html-ls :docker-server-id html-ls-docker
                     :server-command "html-languageserver --stdio")
         (:server-id ts-ls :docker-server-id ts-ls-docker
                     :server-command "typescript-language-server --stdio")
         (:server-id css-ls :docker-server-id css-ls-docker
                     :server-command "css-languageserver --stdio")))
     (lsp-docker+-priority . 10)
     (lsp-docker+-image-id . "node_node")
     (lsp-docker+-container-name . "node_node_1")
     (lsp-docker+-server-cmd-fn . lsp-docker+-exec-in-container)
     (lsp-docker+-docker-options . "-u ${USER}")
     (lsp-docker+-path-mappings . (("${HOME}/work/" . "${HOME}/work/")))))
 (python-mode
  . (
     (lsp-docker+-server-id . pyright)
     (lsp-docker+-docker-server-id . pyr-docker)
     (lsp-docker+-server-command . "pyright-langserver --stdio")
     (lsp-docker+-server-cmd-fn . lsp-docker+-exec-in-container)
     (lsp-docker+-priority . 10)
     (lsp-docker+-image-id . "docker_node")
     (lsp-docker+-container-name . "node_node_1")
     (lsp-docker+-docker-options . "-u ${USER}")
     (lsp-docker+-path-mappings . (("${HOME}/work/" . "${HOME}/work/")))
     )))
