# Demandas M1 — Fundação

Espelho operacional das issues abertas do milestone **M1 — Fundação** (T01–T07).
Implementação das issues fica para etapa posterior; aqui só plano e slot de evidências.

Fonte: [milestone M1](https://github.com/RafaelCarvalho89/oak-note/milestone/1) · PREP-01/02 em `.docs/mvp/`.

## Ordem de dependências

```
T01 (#3) ─┬─► T02 (#4) ─┐
          ├─► T05 (#6)  │
          └─► T06 (#8) ─┼─► T07 (#7)
T03 (#5) ──────────────┴─► T04 (#10)
T08 (#9) → M5 (fora deste espelho)
```

| Pasta | Issue | Código | Depende de | Sprint |
|---|---|---|---|---|
| [`003-t01-gnome51-flatpak/`](003-t01-gnome51-flatpak/) | [#3](https://github.com/RafaelCarvalho89/oak-note/issues/3) | T01 | Nenhuma. | Sprint M1-A (sem dependências) |
| [`004-t02-flatpak-portal/`](004-t02-flatpak-portal/) | [#4](https://github.com/RafaelCarvalho89/oak-note/issues/4) | T02 | T01 (#3) | Sprint M1-B |
| [`005-t03-nomes-identidade/`](005-t03-nomes-identidade/) | [#5](https://github.com/RafaelCarvalho89/oak-note/issues/5) | T03 | Nenhuma | Sprint M1-A (sem dependências) |
| [`006-t05-workers-serializacao/`](006-t05-workers-serializacao/) | [#6](https://github.com/RafaelCarvalho89/oak-note/issues/6) | T05 | T01 (#3) | Sprint M1-B |
| [`007-t07-gettext-gsettings/`](007-t07-gettext-gsettings/) | [#7](https://github.com/RafaelCarvalho89/oak-note/issues/7) | T07 | T06 (#8) | Sprint M1-C |
| [`008-t06-rust-gtk-ci/`](008-t06-rust-gtk-ci/) | [#8](https://github.com/RafaelCarvalho89/oak-note/issues/8) | T06 | T01 (#3) | Sprint M1-B |
| [`010-t04-migracao-recuperacao/`](010-t04-migracao-recuperacao/) | [#10](https://github.com/RafaelCarvalho89/oak-note/issues/10) | T04 | T02 (#4), T03 (#5) | Sprint M1-C |

## Fora deste espelho

- T08 (#9) pertence a **M5 — Release**.
- H01–H27 ficam para milestones posteriores.
