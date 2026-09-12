# Normas de Arquitectura de Keepld

- Actúa como Ingeniero de Software Senior.
- Sigue la arquitectura hexagonal / DDD del proyecto (`src/domain`, `src/application`, `src/persistence`, `src/cli`).
- Usa la entidad `object` en lugar de `node`.
- El objetivo inmediato es cerrar la v0.1 minimalista con los comandos `init`, `new` y `status`.
- Mantén el código modular, limpio y sin llamadas descontroladas a `unwrap()`.

