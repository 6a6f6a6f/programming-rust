# Capitulo 2 - *A Tour of Rust*

Nesse capitulo é introduzido literalmente o básico do básico, caso você tenha pressa para aprender efetivamente a linguagem, sem perder tempo sobre instaladores ou coisas do tipo, recomendo pular a introdução, e ir direto ao conteúdo de exemplo, em especial, sobre as funções e como elas funcionam no Rust.

# `rustup` e Cargo

Para quem já desenvolve em Rust, acredito que conheça bem o `rustup`, porém, raramente o utilizo para instalar no Windows o *toolchain* para desenvolver software em Rust, opto pelo gerenciador de pacotes do Windows para isso:

```shell
$ winget install Rustlang.rust-msvc
```

> **Nota**: Para Windows, você pode utilizar o Rust compilado com o MSVC, ou a versão GNU, que é similar a disponível para Linux (e macOS). Tenho preferência para o *flavor* mais Windows, pois irei utilizar algumas coisas relacionadas.

# O *toolchain*

- `cargo`: É um negócio complexo, faz muitas coisas dentro do ecossistema da linguagem, mas no geral é responsável por gerenciar o compilador (`rustc`), ser nosso *package manager* (pessoalmente, acho muito mais simples de utilizar, e mais poderoso do que o próprio NuGet) e também uma ferramenta de propósito geral (bem como o `dotnet`);
- `rustc`: O nosso compilador, podemos chamar manualmente, mas 99% do tempo deixaremos a cargo do `cargo` realizar estas ações;
- `rustdoc`: Seria algo similar a documentação via XML do C#, essa ferramenta muito poderosa gera a documentação completa daquilo que escreveu, caso siga exatamente o jeito correto de se documentar software em Rust.

# Observações

Caso você for utilizar o `code` para escrever código, prepare-se para instalar diversas ferramentas que não são abordadas no livro, desde o `rls` até o `rust-analysis`, recomendo estudar o que são, após ter certa familiaridade com o idioma.