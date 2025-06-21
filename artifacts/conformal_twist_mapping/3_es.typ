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
- *Técnica*: Dibujo generativo trazado por plotter sobre papel fino de 290g, tinta pigmentada de archivo.
- *Dimensiones*:  50 × 50 cm


= Guíon Curatorial  

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


// (((z+conj(z))/2 - ((conj(z)-z)/2*i*(z*conj(z))^1/2)) + ((z-conj(z))*i/2 - ((z+conj(z))/2*(z*conj(z))^1/2))*i)*e^(-i*PI/7)

// $(
//   frac(z + z^*, 2)
//   - frac((z^* - z) dot i, 2) sqrt(z dot z^*)
//   + (
//     frac((z - z^*) dot i, 2)
//     - frac(z + z^*, 2) sqrt(z dot z^*)
//   ) i
// )dot e^(-i pi / 7)$

// $z^*$

// #set page(width: 210mm, height: 297mm, margin: (25mm, 25mm))
// #set text(font: "Helvetica", size: 11pt, leading: 1.35)

// // #let title-style = text(font: "Helvetica Neue Bold", size: 18pt, tracking: 2%)
// #let subtitle-style = text(font: "Helvetica Neue", size: 12pt, fill: gray)
// #let section-title = text(font: "Helvetica Neue Bold", size: 13pt, leading: 1.4)


// #text[ES]

// A primera vista la pieza parece un simple encuadre de segmentos blancos paralelos. Sin embargo, esos trazos materializan una transformación compleja rigurosa:

// $$
// f(z)=\bigl[(x - r\,y)+i\,(-y - r\,x)\bigr]\,e^{-i\pi/7},
// \qquad r=\sqrt{x^{2}+y^{2}},\ z=x+iy.
// $$

// El mapa combina tres operaciones canónicas:

// 1. \textbf{Cizalla radial} acopla cada coordenada cartesiana con el radio \(r\), curvando la retícula recta en una torsión continua.  
// 2. \textbf{Conformalidad} la función preserva los ángulos locales, característica definitoria de los mapeos conformes.  
// 3. \textbf{Rotación rígida} el factor \(e^{-i\pi/7}\) gira la imagen exactamente \(-\pi/7\) radianes.

// Al trazar un haz de líneas equidistantes y aplicar \(f\), emergen las curvas que distorsionan sutilmente el cuadrado interno. La tensión visual entre el marco inmutable y la malla deformada subraya un tema central de mi práctica: \emph{deformación continua sin ruptura}, donde reglas matemáticas mínimas reconfiguran la percepción respetando la estructura subyacente.

// La obra se inscribe en la tradición del arte algorítmico y generativo, donde el código es simultáneamente medio y metodología. Conceptualmente prolonga mis investigaciones sobre topologías conformes, transformando un mapeo abstracto en un objeto tangible dibujado por plotter. Cada línea es registro, prueba y vestigio de la lógica funcional.

// https://math.libretexts.org/Bookshelves/Analysis/Complex_Variables_with_Applications_%28Orloff%29/01%3A_Complex_Algebra_and_the_Complex_Plane/1.08%3A_Complex_Functions_as_Mappings