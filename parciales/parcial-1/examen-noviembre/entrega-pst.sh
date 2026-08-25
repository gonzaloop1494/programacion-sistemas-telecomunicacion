#!/bin/bash

###########################################
ejercicio=`basename "$(pwd)"` 
GITLAB_PATH=git@gitlab.eif.urjc.es:pst-24-25/$USER/${ejercicio}.git
GITLAB_URL=https://gitlab.eif.urjc.es/pst-24-25/$USER/${ejercicio}.git
###########################################


# Función para mostrar la barra de progreso
show_progress() {
    local total=$1
    local current=0

    while [ $current -le $total ]; do
        # Calcular el porcentaje y la barra
        percent=$((current * 100 / total))
        bar=$(printf "%-${total}s" "#" | tr " " "#")

        # Mostrar la barra de progreso
        printf "\r[%-${total}s] %d%%" "$bar" "$percent"
        
        # Incrementar el contador (simulando trabajo)
        sleep 0.1  # Simulando una tarea
        ((current++))
    done

    printf "\n"
}


clear
echo -e "\nRealizando entrega del ${ejercicio} de PST\n"
echo "Si te pide una cuenta y una password introduce las de tu cuenta en los laboratorios GNU/Linux"
show_progress 20
echo -e "\nFinalizando,..."


# Renombramos master a main
git branch -m master main > /dev/null 2>&1

# Configuramos remote
git remote add origin $GITLAB_URL > /dev/null 2>&1

# Añadmos .gitignore para que no suba binarios de Rust en target
GITIGNORE_FILE=".gitignore"
cat <<EOF > $GITIGNORE_FILE
target/
EOF

# Añadimos pipeline para pasar tests
CI_FILE=".gitlab-ci.yml"
# Crear el archivo .gitlab-ci.yml
cat <<EOF > $CI_FILE
stages:
  - test

rust-test:
  stage: test
  image: rust:latest
  script:
    - cargo test
EOF



# Añadimos, comprometemos y subimos cambios
git add .  > /dev/null 2>&1 
git commit -m "Initial commit" > /dev/null 2>&1 

# Push final
git push --set-upstream origin main  > /dev/null 2>&1 



if [ $? -ne 0 ]; then
      echo -e "\n========================================"
      echo -e "PROBLEMAS en la entrega del ${ejercicio}"
      echo -e "========================================"
else  
      echo -e "\n================================================"
      echo -e "Entrega del ${ejercicio} realizada correctamente"
      echo    "================================================"
fi



echo -e "\nComprueba si has subido el código de tu ejercicio en este url:"
echo "  ${GITLAB_URL}"
echo -e "\n  1. Comprueba que los ficheros que hay allí son los que has entregado."
echo -e "\n  2. Si los tests te pasaban saldrá un círculo verde en el que "
echo    "     puedes pinchar para ver cómo en el servidor también pasan."
echo -e "\n     Si no te pasaban verás un círculo rojo en su lugar, y pinchando"
echo    "     en él podrás comprobar qué tests pasan y cuáles no."
echo -e "\nPuedes volver a entregar tu práctica si haces cambios ejecutando"
echo    "de nuevo el comando entrega.sh desde el directorio del ejercicio."

echo -e "\nSi no consigues ver tu código subido, comprueba si tienes conexión a la red,"
echo    "asegúrate de que has introducido bien tus credenciales,"
echo    "y asegúrate de que el directorio de tu código tiene el nombre adecuado:"
echo -e "    ejercicio_XY,   siendo XY el número de ejercicio\n"

