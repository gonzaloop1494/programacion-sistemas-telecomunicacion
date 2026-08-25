# Programacion en Sistemas de Telecomunicacion (PST)

Repositorio de la asignatura **Programacion en Sistemas de Telecomunicacion**, tambien llamada **PST**.

El contenido se organizo desde las carpetas locales del Desktop, dejando fuera compilados, credenciales, copias vacias, duplicados claros y archivos que GitHub no acepta bien en un repositorio normal.

## Estructura

- `apuntes/`: PDFs de repaso y apuntes de Rust.
- `docs/`: documentacion del repositorio y enunciados relevantes.
- `examen/`: ejercicios finales y carpeta de noviembre.
- `parciales/parcial-1/`: ejercicios del primer parcial y examen de noviembre.
- `parciales/parcial-2/`: ejercicios del segundo parcial, sin copias repetidas de proyectos ya guardados.
- `practicas/`: ejercicios de clase, Tema 1, Parte 2 y scripts auxiliares.
- `proyectos/`: proyectos independientes (`chatgrupal`, `groupchat` y `finalmodifs`).
- `media/streamcast/`: nota sobre los videos locales no versionados.

## Criterios de limpieza

- Se eliminaron `target/`, `.idea/`, `.DS_Store`, caches y otros archivos generados.
- Se omitieron `credentials.txt`, `client_credentials.txt` y `.env`.
- Se omitieron ZIPs, videos, PCAPs grandes y `attempts-large.txt`.
- Se consolidaron duplicados evidentes:
  - `chatgrupal` es la copia canonica de `ejsparcial2/ej40` y `PST/parte2/proyectofinal/ej40`.
  - `groupchat` es la copia canonica de `ejsparcial2/groupchat`.
  - `examen/ejercicio_27` representa tambien la carpeta suelta `ejercicio_27`.
  - `examen/ejercicio_17` y `examen/ejercicio_19` cubren las copias identicas encontradas en `PST/ejsparcial1`.
  - `practicas/tema-1/entregaejercicios/ejercicio_17` cubre la copia equivalente de `PST/tiposcompuestos/Ej17`.

## Uso

La mayoria de ejercicios son proyectos Rust. Para ejecutar uno:

```bash
cd ruta/del/ejercicio
cargo run
```

Los archivos pesados omitidos estan documentados en `docs/ARCHIVOS_OMITIDOS.md`.
