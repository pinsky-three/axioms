#import "@preview/tiaoma:0.3.0"

#set page(
  paper: "a4",
  header: [
    #align(image("pinsky_logo_black.svg", width: 50pt), right)
  ],
  footer-descent: -20pt,
  footer: [
    #rect(
      stroke: 0pt,
      inset: 0pt,
      stack(
        dir: ltr,
        spacing: 1fr,
        tiaoma.qrcode("https://pinsky.studio/p/685464e5-9d0f-4004-b3c7-a8f40c0f5962"),
        align(text(datetime.today().display(), size: 14pt, weight: 600), bottom)
      )
    )
  ]
)

// #text("Conformal Twist Mapping 1", size: 18pt, weight: 800)

= * $= "Conformal_Twist_Mapping"(1)$ *


= Ficha Técnica

- *Año de producción*: 2025
- *Técnica*: Dibujo generativo trazado por plotter sobre papel de alta calidad de 290g, tinta pigmentada de archivo.
- *Dimensiones*:  50 × 50 cm


= Texto curatorial

A primera vista la pieza parece un simple encuadre de segmentos blancos paralelos. Sin embargo, esos trazos materializan una transformación compleja rigurosa:

$
f(z)=[(x - r y) + (-y - r x)i] dot e^(-pi/7 i),
r = sqrt(x^2 + y^2),
z = x + y i
// \qquad r=\sqrt{x^{2}+y^{2}},\ z=x+yi.
$

El mapa combina tres operaciones canónicas:

1. *Cizalla radial* $(x,y) mapsto (x-r y, -y -r x)$ acopla cada coordenada cartesiana con el radio $r$, curvando la retícula recta en una torsión continua.
2. *Conformalidad* la función preserva los ángulos locales, característica definitoria de los mapeos conformes.  
3. *Rotación rígida* el factor $e^(- pi/7 i )$ gira la imagen exactamente $-pi/7$ radianes.

Al trazar un haz de líneas equidistantes y aplicar $f$ , emergen las curvas que distorsionan sutilmente el cuadrado interno. La tensión visual entre el marco inmutable y la malla deformada subraya un tema central de mi práctica: *deformación continua sin ruptura* donde reglas matemáticas mínimas reconfiguran la percepción respetando la estructura subyacente.

La obra se inscribe en la tradición del arte algorítmico y generativo, donde el código es simultáneamente medio y metodología. Conceptualmente prolonga mis investigaciones sobre topologías conformes, transformando un mapeo abstracto en un objeto tangible dibujado por plotter. Cada línea es registro, prueba y vestigio de la lógica funcional.


#bibliography("references.bib", full: true, title: "Bibliografía")