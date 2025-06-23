#import "@preview/tiaoma:0.3.0"

#set page(
  paper: "a4",
  header: [
    #align(image("pinsky_logo_black.svg", width: 50pt), right)
  ],
  footer-descent: -20pt,
  footer: [
    #rect(
      stroke: none,
      inset: 0pt,
      stack(
        dir: ltr,
        spacing: 1fr,
        tiaoma.qrcode("https://pinsky.studio/p/fe5bb12f-29de-426a-ab42-6f6ec7ca8174"),
        align(text(datetime.today().display(), size: 14pt, weight: 600), bottom)
      )
    )
  ]
)

= * $= "Conformal_Twist_Mapping"(3)$ *

= Ficha técnica

- *Año de producción*: 2025
- *Técnica*: Dibujo generativo trazado en papel de alta calidad de 290g, tinta pigmentada de archivo
- *Dimensiones*: 50 × 50 cm

= Texto curatorial

Esta obra explora la aparición de simetrías de orden superior a partir de un conjunto mínimo de reglas matemáticas. La imagen, un patrón clásico de tetralóbulo (cuatro hojas), se genera mediante un mapeo conforme no iterativo que transforma todo el plano complejo.

La función se basa en un único polo, que luego se eleva a una potencia para crear los lóbulos simétricos. La transformación se define como:

$
f(z) = 1/( z + 0.1) dot e^(pi i)
$

La obra se define por dos principios fundamentales:

1. *El Polo Fundacional*
  La transformación está definida fundamentalmente por un único polo en $z = -0.1$. Esta singularidad actúa como un centro gravitacional que deforma el tejido del plano complejo. Toda la estructura visual —la curvatura de líneas rectas en arcos elegantes— se origina a partir de la influencia de este único punto fundacional.

2. *Rotación Rígida*
  La multiplicación por $e^(pi i)$ (que equivale a $-1$) aplica una rotación rígida de 180 grados a todo el campo invertido. Este paso final reorienta la estructura en el plano pero preserva su geometría interna, completando la transformación de una cuadrícula simple a la forma final y elegante.

  
La pieza es un estudio sobre la emergencia estructural. Demuestra cómo un elemento simple —un único polo— puede ser modificado por una operación sencilla —la exponenciación— para producir una forma de belleza intrincada e inesperada, tendiendo un puente entre la fórmula abstracta y el artefacto físico.

#bibliography("references.bib", full: true, title: "Bibliografía")