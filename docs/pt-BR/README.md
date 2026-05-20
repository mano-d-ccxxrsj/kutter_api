# Kutter API

Este documento explica a estrutura atual do projeto, como rodar a API e onde mexer ao adicionar novas features.

Repositório: [mano-d-ccxxrsj/kutter_api](https://github.com/mano-d-ccxxrsj/kutter_api)

Idioma: [English](https://github.com/mano-d-ccxxrsj/kutter_api/blob/main/docs/en/README.md)

## Visão Geral

O projeto é um workspace Rust dividido por responsabilidade. A ideia atual é manter a regra de negócio explícita no domínio, deixar banco e HTTP como adaptadores, e centralizar a montagem concreta da aplicação no crate `app`.

## Estrutura

| Caminho       | Responsabilidade                                                                                                      |
|---------------|-----------------------------------------------------------------------------------------------------------------------|
| `app`         | Ponto de entrada, criação do runtime Tokio, schema inicial e composição dos serviços concretos.                       |
| `domain`      | Entidades, comandos, erros, portas e serviços de negócio. Deve conhecer contratos, não detalhes de Postgres ou Actix. |
| `persistence` | Implementação Postgres das portas de repositório, modelos SQLx, mappers, schema e pool.                               |
| `web`         | Servidor Actix, rotas, handlers e DTOs de request/response.                                                           |
| `infra`       | Leitura de ambiente e valores padrão de configuração.                                                                 |
| `security`    | Bcrypt, JWT e geração de chave pública.                                                                               |
| `shared`      | Tipos e portas compartilhadas por mais de uma camada.                                                                 |

## Fluxo de Dependências

`domain` define o que a aplicação precisa. `persistence`, `security` e `web` implementam detalhes externos. `app` junta tudo em `ApplicationConfig`.

O fluxo esperado é:

```text
web -> domain <- persistence
app -> web/domain/persistence/security/infra/shared
```

O crate `domain` não deve depender de `web`, `persistence`, `infra` ou `security`.

## Configuração de Ambiente

A aplicação lê `.env` automaticamente por `infra::EnvConfig`. `JWT_KEY` é obrigatório.

| Variável                     | Uso                                                                                                          |
|------------------------------|--------------------------------------------------------------------------------------------------------------|
| `APP_HOST`                   | Host usado pela API fora do Docker. Padrão: `127.0.0.1`.                                                     |
| `APP_CLIENT_PORT`            | Porta esperada para o cliente/frontend. Padrão: `3001`.                                                      |
| `APP_SERVER_PORT`            | Porta HTTP da API. Padrão: `8080`.                                                                           |
| `USE_HTTPS`                  | Liga/desliga uso lógico de HTTPS na configuração. Padrão: `false`.                                           |
| `JWT_KEY`                    | Chave usada para assinar tokens de sessão. Obrigatória.                                                      |
| `CONTENT_MODERATION_ENABLED` | Liga/desliga o filtro de conteúdo. Padrão: `false`.                                                          |
| `DB_HOST`                    | Host do Postgres. Use `localhost` no host; no Docker a aplicação troca para `db_postgres` quando necessário. |
| `DB_PORT`                    | Porta do Postgres. Padrão: `5432`.                                                                           |
| `DB_USER`                    | Usuário do Postgres.                                                                                         |
| `DB_PASSWORD`                | Senha do Postgres.                                                                                           |
| `DB_NAME`                    | Nome do banco.                                                                                               |
| `DATABASE_URL`               | Opcional. Se definida, substitui a URL montada a partir das variáveis `DB_*`.                                |
| `DB_MAX_CONNECTIONS`         | Máximo de conexões do pool.                                                                                  |
| `DB_MIN_CONNECTIONS`         | Mínimo de conexões do pool.                                                                                  |
| `DB_ACQUIRE_TIMEOUT`         | Timeout para adquirir conexão, em segundos.                                                                  |
| `DB_MAX_LIFETIME`            | Tempo máximo de vida da conexão, em segundos.                                                                |
| `DB_IDLE_TIMEOUT`            | Tempo máximo ocioso da conexão, em segundos.                                                                 |

## Rodando com Docker

Para subir API e Postgres:

```bash
docker compose up --build
```

Para subir apenas o banco e rodar a API no host:

```bash
docker compose up -d db_postgres
```

Seguido de:

```bash
cargo run -p app
```

O schema atual é criado na inicialização da aplicação por funções em `persistence/src/database/schema.rs`.

## Rodando no Host

Requisitos:

- Rust instalado.
- Postgres rodando localmente.
- Um arquivo `.env` com `JWT_KEY` e as variáveis de banco.

Com o banco disponível:

```bash
cargo run -p app
```

Para validar compilação e testes:

```bash
cargo check
```

Seguido de:

```bash
cargo test
```

## Moderação de Conteúdo

A moderação é configurável por `CONTENT_MODERATION_ENABLED`.

Quando desligada, `ToggleContentModerationService` libera o conteúdo sem consultar a base de termos. Quando ligada, o serviço consulta `banned_words`, normaliza o conteúdo e registra violações em `user_flags`.

Até existir CRUD administrativo, os termos podem ser inseridos diretamente:

```sql
INSERT INTO banned_words (word, active) VALUES ('example', TRUE);
```

## Como Adicionar uma Feature

1. Comece pelo `domain`.
2. Crie ou ajuste entidades em `domain/src/entities`.
3. Crie tipos de entrada/saída em `domain/src/types` quando a operação precisar de comandos próprios.
4. Defina portas em `domain/src/ports`.
5. Implemente a regra em `domain/src/services`.
6. Se houver banco, implemente modelo, mapper e repositório em `persistence`.
7. Atualize `persistence/src/database/schema.rs` se a feature precisa de tabela ou coluna.
8. Se houver HTTP, crie DTOs manuais em `web/src/dto` e handlers em `web/src/handlers`.
9. Registre as rotas em `web/src/http/server.rs`.
10. Ligue a implementação concreta em `app/src/config/application_config.rs`.
11. Rode `cargo check` e `cargo test`.

## Convenções Atuais

- Prefira tipos explícitos em variáveis relevantes.
- Prefira DTOs manuais a derives de serialização quando fizer sentido para manter controle.
- Mantenha aliases, structs e serviços nos arquivos que representam sua responsabilidade.
- Evite espalhar strings de configuração ou composição fora de `infra` e `app`.
- Use macros apenas quando a integração com a crate justificar, como `async_trait` e `sqlx::FromRow`.

## Endpoints Atuais

| Método | Caminho                       | Uso                                        |
|--------|-------------------------------|--------------------------------------------|
| `POST` | `/api/user/register`          | Registra usuário.                          |
| `POST` | `/api/user/login`             | Autentica usuário e cria cookie de sessão. |
| `POST` | `/api/community/create`       | Cria comunidade autenticada.               |
| `POST` | `/api/channel/create`         | Cria canal autenticado.                    |
| `POST` | `/api/channel/message/send`   | Envia mensagem.                            |
| `POST` | `/api/channel/message/list`   | Lista mensagens de um canal.               |
| `POST` | `/api/channel/message/edit`   | Edita mensagem do autor.                   |
| `POST` | `/api/channel/message/delete` | Remove mensagem do autor.                  |