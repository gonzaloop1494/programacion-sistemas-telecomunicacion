# Archivos omitidos

Estos archivos se dejaron fuera para evitar duplicados, secretos, compilados o blobs demasiado grandes para un repositorio GitHub normal.

## Generados o locales

- `target/`
- `.idea/`
- `.DS_Store`
- `__pycache__/`
- `.venv/`, `venv/`

## Credenciales

- `credentials.txt`
- `client_credentials.txt`
- `.env`

## Duplicados consolidados

- `Desktop/chatgrupal copia/`: carpeta vacia.
- `Desktop/ejsparcial2/ej40/`: mismo codigo que `Desktop/chatgrupal/`.
- `Desktop/PST/parte2/proyectofinal/ej40/`: mismo codigo que `Desktop/chatgrupal/`.
- `Desktop/ejsparcial2/groupchat/`: mismo codigo que `Desktop/groupchat/`, salvo README.
- `Desktop/ejercicio_27/`: mismo codigo que `Desktop/examen/ejercicio_27/`.
- `Desktop/PST/ejsparcial1/ejercicio_17/`: cubierto por `Desktop/examen/ejercicio_17/`.
- `Desktop/PST/ejsparcial1/ejercicio_19/`: cubierto por `Desktop/examen/ejercicio_19/`.
- `Desktop/PST/tiposcompuestos/Ej17/`: cubierto por `Desktop/PST/Tema1/entregaejercicios/ejercicio_17/`.

## Datos y medios pesados

GitHub bloquea archivos de mas de 100 MB en repositorios normales y avisa con archivos de mas de 50 MB. Como `git-lfs` no esta instalado en este equipo, estos archivos quedaron referenciados pero no versionados.

- `Desktop/streamcast/cliente.MOV` (628 MB)
- `Desktop/streamcast/ejecucion.MOV` (1.1 GB)
- `Desktop/streamcast/ejecucionycliente.zip` (1.5 GB)
- `Desktop/streamcast/servidor.MOV` (980 MB)
- `Desktop/streamcast/servidor.MOV.zip` (841 MB)
- `Desktop/PST/.../example.pcap` y `Desktop/ejsparcial2/.../example.pcap` (224 MB cada uno)
- `Desktop/PST/.../example2.pcap` y `Desktop/ejsparcial2/.../example2.pcap` (224 MB cada uno)
- `attempts-large.txt` repetido en varias carpetas (86 MB cada copia)

Si estos archivos son necesarios en GitHub, lo adecuado es subirlos con Git LFS o adjuntarlos como assets de una release.
