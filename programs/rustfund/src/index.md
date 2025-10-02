# 📘 Índice del tutorial `rustfund`

### **Parte A – Declaraciones (estructuras, cuentas y enums)**

Estas son las dependencias necesarias para que el programa compile y pueda ejecutar su lógica:

1. `FundCreate<'info>` – contexto de creación de un fondo.
2. `FundContribute<'info>` – contexto de contribución a un fondo.
3. `FundSetDeadline<'info>` – contexto para establecer fecha límite.
4. `FundRefund<'info>` – contexto para reembolso a un contribuidor.
5. `FundWithdraw<'info>` – contexto para retiro de fondos por el creador.
6. `Contribution` – estructura de cuenta que guarda contribuciones individuales.
7. `Fund` – estructura de cuenta que representa un fondo con sus datos.
8. `ErrorCode` – enumerador de errores personalizados del programa.

---

### **Parte B – Funciones del programa (`pub mod rustfund`)**

Una vez entendido lo anterior, se pasa a la lógica de negocio:

1. `fund_create` – inicializa un nuevo fondo.
2. `contribute` – permite a un usuario contribuir a un fondo.
3. `set_deadline` – fija la fecha límite de un fondo.
4. `refund` – devuelve fondos a los contribuidores si corresponde.
5. `withdraw` – transfiere los fondos recaudados al creador.

---

📌 Propuesta de flujo de explicación:

* Empezamos con la **Parte A** (estructuras, contextos, enums).
* Cada declaración la vemos **en un punto independiente**: explicamos qué es, cómo funciona en Rust, y qué papel juega en Solana/Anchor.
* Cuando terminemos todas, pasamos a la **Parte B** (funciones) con la misma metodología.

---
