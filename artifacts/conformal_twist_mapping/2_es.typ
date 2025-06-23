#import "@preview/tiaoma:0.3.0"

#set page(
  paper: "a4",
  // El archivo del logo no fue proporcionado, por lo que el encabezado está comentado.
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
        // Código QR apuntando a una URL de marcador de posición para esta pieza.
        tiaoma.qrcode("https://pinsky.studio/p/121608e8-3418-4a95-8b02-cba2a6d7602c"),
        align(text(datetime.today().display(), size: 14pt, weight: 600), bottom)
      )
    )
  ]
)

= * $= "Conformal_Twist_Mapping"(2)$ *

= Ficha técnica

- *Año de producción*: 2025
- *Técnica*: Dibujo generativo trazado en papel de alta calidad de 290g, tinta pigmentada de archivo
- *Dimensiones*: 50 × 50 cm

= Texto curatorial

Esta pieza traduce un concepto fundamental del análisis complejo en un artefacto tangible y trazado. La imagen, que recuerda a un campo dipolar magnético, se genera al mapear una cuadrícula simple de líneas a través de una función compleja, revelando una estructura elegante y emergente.

La transformación está gobernada por la función $f(z)$, que describe un campo con un único polo, o singularidad:

$
f(z) = 1 / (z + c), quad c = 0.01 + 0.01i
$

La obra se define por dos principios fundamentales:

1. *El Polo y la Singularidad*
   La función posee un único polo en $z = -c$. Este punto, donde la función tiende a infinito, actúa como el centro desde el cual emana toda la estructura del campo, dictando el flujo y la curvatura de cada línea.

2. *Mapeo Conforme*
   Como función analítica, $f(z)$ es conforme en todo su dominio excepto en su polo. Esta propiedad asegura que los ángulos entre las líneas que se intersectan se preserven desde la cuadrícula original hasta el dibujo final, lo que resulta en las elegantes curvas ortogonales que definen la textura de la obra.

La obra es una exploración de cómo reglas simples y deterministas pueden generar una profunda complejidad estructural. Captura la influencia invisible de un único punto en el plano complejo, representándolo como un sereno patrón de tinta sobre papel, donde cada línea sirve como testamento de la lógica matemática subyacente.

#bibliography("references.bib", full: true, title: "Bibliografía")