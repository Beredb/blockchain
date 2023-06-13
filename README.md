# blockchain
Mi primer repositorio para incluir la tarea de blockchain de la Maestría
# Contrato de Seguro de Vida en Rust para Solana

Este repositorio contiene una implementación básica de un contrato de seguro de vida utilizando el lenguaje de programación Rust y la red Solana. El contrato de seguro de vida se modela a través de la estructura `InsurancePolicy`.

## Estructura `InsurancePolicy`

La estructura `InsurancePolicy` representa una póliza de seguro de vida y tiene los siguientes campos:

- `coverage_period`: Período de cobertura en años.
- `premium`: Prima anual de la póliza.
- `coverage_amount`: Suma asegurada.
- `beneficiaries`: Lista de beneficiarios de la póliza.
- `payment_status`: Estado del pago de la prima.

## Funcionalidades

El contrato de seguro de vida proporciona las siguientes funcionalidades:

- `new`: Crea una nueva póliza de seguro de vida con los parámetros especificados.
- `make_payment`: Procesa el pago de la prima de la póliza.
- `file_claim`: Presenta un reclamo de la póliza, verificando la autenticidad de la muerte del titular y devuelve los beneficiarios elegibles para recibir el pago.

## Ejemplo de Uso

El archivo `main.rs` muestra un ejemplo de uso del contrato de seguro de vida. En este ejemplo, se crea una póliza de seguro de vida con un período de cobertura de 20 años, una prima de $100.0, una suma asegurada de $100,000.0 y dos beneficiarios. Luego, se realiza el pago de la prima y se verifica la autenticidad de la muerte del titular. Si el reclamo es elegible, se muestra la lista de beneficiarios.

## Requisitos

- Rust: Asegúrate de tener Rust instalado en tu sistema. Puedes obtenerlo en [https://www.rust-lang.org/](https://www.rust-lang.org/).

## Configuración y Ejecución

1. Clona este repositorio en tu máquina local.
2. Navega hasta el directorio del proyecto.
3. Ejecuta `cargo run` para compilar y ejecutar el programa.

¡Listo! Ahora puedes explorar y experimentar con la implementación básica del contrato de seguro de vida en Rust para Solana.

## Contribución

¡Siéntete libre de contribuir a este proyecto! Puedes enviar pull requests con mejoras, correcciones de errores o agregar nuevas funcionalidades al contrato de seguro de vida.

## Agradecimientos

Este proyecto se inspira en la implementación de contratos inteligentes en Rust y Solana. Agradecemos a la comunidad de Rust y Solana por su contribución y recursos disponibles.


