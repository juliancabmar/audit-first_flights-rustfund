Perfecto ✅, entonces el nuevo orden del tutorial será:

---

## Índice del tutorial `minimal_counter` (nuevo orden)

### A) Dependencias y configuraciones iniciales

1. `use anchor_lang::prelude::*;` → qué importa y por qué
2. `declare_id!` → qué hace, cómo se usa

### D) Structs persistentes (`#[account]`)

1. `Counter`

   * Campo `value`
   * Cómo Anchor serializa en la cuenta
   * Espacio en bytes (`8 + 8`)

### C) Contextos de cuentas (`#[derive(Accounts)]`)

1. `Initialize<'info>`

   * Campos: `counter`, `user`, `system_program`
   * Macro de atributo `#[account(...)]` y sus parámetros (`init`, `payer`, `space`, `mut`)
2. `SetValue<'info>`

   * Campo `counter` y `mut`

### B) Programa principal

1. `#[program] pub mod minimal_counter { ... }`

   * Qué significa `#[program]`
   * Estructura del módulo y uso de `use super::*;`
2. Funciones del programa

   * `initialize`
   * `set_value`

### E) Flujo de transacción y almacenamiento

1. Cómo se crea la cuenta PDA en `initialize`
2. Cómo se modifica en `set_value`
3. Quién puede leer/escribir la cuenta

### F) Comparaciones con Solidity/EVM

* Owner, almacenamiento, permisos
* Similitudes y diferencias con `msg.sender`, `storage` y `contract`
