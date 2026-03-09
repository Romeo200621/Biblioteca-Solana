Bar de Bebidas en Solana 

CRUD básico de un Solana Program desarrollado con Rust y Anchor desde Solana Playground.

Este proyecto permite gestionar un bar de bebidas en la blockchain, donde se pueden registrar bebidas, ver el menú, modificar su disponibilidad o eliminarlas.

Puedes comenzar dándole Fork a este repositorio (abajo te explicamos cómo ).
Hemos preparado un entorno de codespaces listo para que no tengas que instalar nada; solo sigue los ejercicios y ejemplos diseñados especialmente para aprender.

Asegúrate de clonar este repositorio a tu cuenta usando el botón Fork.

Importando el proyecto

Una vez que el repositorio esté en tu cuenta, lo siguiente que debes hacer es copiar el enlace de tu repositorio, lo cual puedes hacer directamente desde el navegador.

Posteriormente, lo uniremos con el siguiente enlace en tu navegador:

https://beta.solpg.io/

Esto abrirá el entorno de Solana Playground con tu proyecto listo para trabajar.

Al presionar Enter, serás enviado al entorno de Solana Playground con tu proyecto abierto.

Para guardarlo, solo debes hacer clic en el botón Import y asignar un nombre a tu proyecto.

Preparación del entorno

Primero conectaremos el entorno con la Devnet de Solana, lo que también creará automáticamente una wallet para interactuar con el programa.

Para hacerlo, haz clic en donde dice:

Not Connected

Se abrirá una ventana donde debes presionar el botón Continue.

Después de esto, el entorno mostrará la siguiente información:

Estado de conexión

 En verde:
El estado de la conexión y la red a la que estás conectado (Devnet).

 En amarillo:
La dirección de la wallet conectada.

 En azul:
La cantidad de tokens SOL disponibles en la wallet, los cuales podrás usar para probar las transacciones del bar.

Funcionalidad del proyecto

Este programa permite administrar un bar de bebidas en la blockchain mediante las siguientes operaciones:

Crear un Bar

Agregar bebidas al menú

Ver las bebidas disponibles

Eliminar bebidas

Cambiar la disponibilidad de una bebida

Todo esto queda registrado on-chain en Solana.
