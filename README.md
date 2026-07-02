# RustDesk Server (fork)

> ## Важно — прочитайте перед использованием
>
> Этот репозиторий **публичный** и выложен в ознакомительных целях.
>
> **Код и документация в этом форке создавались и дорабатывались с помощью нейросетей (ИИ-ассистентов)** в рамках личного проекта и **только для личных, тестовых и учебных сред**. Цель — изучение возможностей нейросетей в разработке и администрировании, а не выпуск готового коммерческого продукта.
>
> **Никаких гарантий.** ПО предоставляется «как есть» (AS IS). Автор не гарантирует работоспособность, безопасность, пригодность для какой-либо цели, отсутствие ошибок и уязвимостей. **Используйте на свой страх и риск.**
>
> **Это не замена [RustDesk Server Pro](https://rustdesk.com/pricing.html).** Для production, SLA, расширенных функций (ACL, LDAP, веб-консоль, аудит и т.д.) **покупайте официальную коммерческую версию** у [RustDesk](https://rustdesk.com/). Этот форк **не поддерживается** командой RustDesk и **не претендует** на замену Pro.
>
> **Лицензия AGPL-3.0.** Исходный код основан на [rustdesk/rustdesk-server](https://github.com/rustdesk/rustdesk-server) и распространяется под **GNU Affero General Public License v3.0** (см. файл [LICENSE](./LICENSE)). Если вы модифицируете код и предоставляете доступ к серверу по сети, вы обязаны соблюдать условия AGPL, включая раскрытие исходников производных работ. **Это не юридическая консультация** — при сомнениях обратитесь к юристу.
>
> **Отказ от ответственности.** Автор не несёт ответственности за прямой или косвенный ущерб, потерю данных, простой, утечки, нарушение законодательства или иные последствия использования этого репозитория. Вы самостоятельно оцениваете риски и соблюдаете лицензии upstream-проектов.

---

Fork ветки `slmr-fork` на базе upstream [rustdesk/rustdesk-server](https://github.com/rustdesk/rustdesk-server). Экспериментальные доработки для self-hosted сценариев; **не использовать в production без собственного аудита.**

| Ссылка | Описание |
|--------|----------|
| [Upstream](https://github.com/rustdesk/rustdesk-server) | Оригинальный OSS-сервер RustDesk |
| [Документация OSS](https://rustdesk.com/docs/en/self-host/rustdesk-server-oss/) | Официальная установка |
| [RustDesk Server Pro](https://rustdesk.com/pricing.html) | Коммерческая версия с поддержкой |
| [FAQ](https://github.com/rustdesk/rustdesk/wiki/FAQ) | Частые вопросы |

## Сборка

```bash
git submodule update --init --recursive
cargo build --release
```

В `target/release`:

- **hbbs** — ID / rendezvous server
- **hbbr** — relay server
- **rustdesk-utils** — CLI utilities

Официальные бинарники upstream: [Releases](https://github.com/rustdesk/rustdesk-server/releases).

## Установка

См. [официальный документ](https://rustdesk.com/docs/en/self-host/rustdesk-server-oss/) upstream. Для этого форка — только после прочтения дисклеймера выше.

---

> ## Important — read before use
>
> This repository is **public** and published for **informational and educational purposes only**.
>
> **Code and documentation in this fork were created and modified with the help of neural networks (AI assistants)** as part of a personal project, **for personal, test, and learning environments only**. The goal is to explore what AI tools can do in software development and operations — **not** to ship a production-ready commercial product.
>
> **No warranty.** The software is provided **“AS IS”**, without warranty of any kind. The author does not guarantee fitness for any purpose, correctness, security, or freedom from defects. **Use at your own risk.**
>
> **This is not a replacement for [RustDesk Server Pro](https://rustdesk.com/pricing.html).** For production deployments, SLAs, and advanced features (ACL, LDAP, web console, audit, etc.), **please purchase the official commercial product** from [RustDesk](https://rustdesk.com/). This fork is **not supported** by the RustDesk team and **does not** replace Pro.
>
> **AGPL-3.0 license.** This project is based on [rustdesk/rustdesk-server](https://github.com/rustdesk/rustdesk-server) and is licensed under the **GNU Affero General Public License v3.0** (see [LICENSE](./LICENSE)). If you modify the code and offer network access to the server, you must comply with AGPL terms, including source disclosure for derivative works. **This is not legal advice** — consult a lawyer if unsure.
>
> **Disclaimer of liability.** The author shall not be liable for any direct or indirect damages, data loss, downtime, security incidents, regulatory violations, or other consequences arising from use of this repository. You are solely responsible for risk assessment and compliance with upstream licenses.
