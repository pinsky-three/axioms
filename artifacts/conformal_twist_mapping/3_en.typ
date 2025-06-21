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

// #set align(horizon)

= * $= "Conformal_Twist_Mapping"(1)$ *

= Technical sheet

- *Year of production*: 2025
- *Technique*: Generative drawing plotted on 290g fine paper, archival pigment ink
- *Dimensions*: 50 × 50 cm

= Curatorial statement  

At first glance the piece presents little more than a square frame of parallel white segments. Yet those segments are the trace of a rigorous complex-analytic transformation:

$
f(z)=[(x - r y) + (-y - r x)i] dot e^(-pi/7 i),
r = sqrt(x^2 + y^2),
z = x + y i
$

The map combines three canonical operations:

1. *Radial shear* $(x,y) mapsto (x-r y, -y -r x)$ bends straight grid-lines into a continuous torsion.  
2. *Angle-preserving conformality* Because the algebra derives from $z$ and its conjugate $overline(z)$, the local angle between any two infinitesimal directions is preserved.  
3. *Rigid rotation* Multiplication by $e^(- pi/7 i )$ applies an exact $-pi/7$ radian rotation, echoing Euler’s interpretation of complex exponentials.

Plotting a family of equally spaced pre-image lines under $f$ produces the subtle “twisted square” you observe. The tension between the immovable frame and the internally distorted mesh foregrounds a central theme of my practice: *continuous deformation without rupture* — how minimal mathematical rules reshape perception while respecting underlying structure.

Formally, the work situates itself within the lineage of algorithmic and generative art, where code operates as both medium and method. Conceptually, it extends earlier explorations of conformal topologies, turning an abstract mapping into a tactile plotter-drawn object. Each line is simultaneously record, proof, and artefact of the function’s logic.


#bibliography("references.bib", full: true)

