Una herramienta simple en Rust para filtrar y buscar líneas específicas en archivos de log según el nivel de severidad.

## ¿Qué hace?

Lee un archivo de log y busca todas las líneas que contengan un nivel específico (como ERROR, WARN, INFO, etc.). Muestra los resultados en consola y genera un reporte en JSON con las líneas encontradas y sus números.

## Uso

```bash
cargo run -- <ruta_al_archivo> [--level NIVEL]
```

### Ejemplos

Buscar errores en un archivo de log:

```bash
cargo run -- test.log
```

Buscar warnings específicamente:

```bash
cargo run -- test.log --level WARN
```

El nivel es case-insensitive, así que `--level error` y `--level ERROR` funcionan igual.

## Salida

El programa muestra:

- Las líneas encontradas con su número de línea
- El tiempo de ejecución
- Un reporte JSON completo que se guarda en `report.json`
