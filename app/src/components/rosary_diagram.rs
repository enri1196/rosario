use crate::i18n::Translation;
use leptos::prelude::*;

#[derive(Clone, Copy)]
enum BeadKind {
    HailMary,
    Pendant,
    OurFather,
}

impl BeadKind {
    const fn radius(self) -> f64 {
        match self {
            Self::HailMary => 3.8,
            Self::Pendant => 4.0,
            Self::OurFather => 7.0,
        }
    }

    const fn class(self) -> &'static str {
        match self {
            Self::HailMary => "bead hail-mary-bead",
            Self::Pendant => "bead",
            Self::OurFather => "large-bead our-father-bead",
        }
    }
}

/// Renders one rosary bead with geometry and styling derived from its kind.
#[component]
fn Bead(x: f64, y: f64, kind: BeadKind) -> impl IntoView {
    view! {
        <circle cx=x cy=y r=kind.radius() class=kind.class()/>
    }
}

#[component]
pub(super) fn RosaryDiagram(copy: Memo<Translation>) -> impl IntoView {
    const CENTER_X: f64 = 260.0;
    const CENTER_Y: f64 = 165.0;
    const RADIUS_X: f64 = 132.0;
    const RADIUS_Y: f64 = 120.0;

    let point_on_ring = |degrees: f64| {
        let radians = degrees.to_radians();
        (
            CENTER_X + RADIUS_X * radians.cos(),
            CENTER_Y + RADIUS_Y * radians.sin(),
        )
    };

    // The branch bead at the bottom crossing is the fifth decade's Our Father
    // bead. The ellipse contains the other four large beads and ten Hail Mary
    // beads on each side of the central bead.
    const DECADE_STARTS: [f64; 5] = [18.0, 90.0, 162.0, 234.0, 306.0];
    let our_father_beads = DECADE_STARTS
        .into_iter()
        .filter(|degrees| *degrees != 90.0)
        .map(point_on_ring)
        .collect::<Vec<_>>();
    let hail_mary_beads = DECADE_STARTS
        .into_iter()
        .flat_map(|start| {
            (1..=10).map(move |bead| point_on_ring(start + f64::from(bead) * (72.0 / 11.0)))
        })
        .collect::<Vec<_>>();

    view! {
        <div class="rosary-wrap">
            <svg class="rosary" viewBox="0 0 520 420" role="img" aria-labelledby="rosary-title rosary-desc">
                <title id="rosary-title">{move || copy.get().guide_title}</title>
                <desc id="rosary-desc">{move || copy.get().decade_note}</desc>
                <defs>
                    <radialGradient id="bead" cx="35%" cy="30%" r="65%">
                        <stop offset="0%" stop-color="var(--color-bead-highlight)"/><stop offset="100%" stop-color="var(--color-bead-shadow)"/>
                    </radialGradient>
                    <filter id="glow"><feGaussianBlur stdDeviation="1.2" result="blur"/><feMerge><feMergeNode in="blur"/><feMergeNode in="SourceGraphic"/></feMerge></filter>
                </defs>
                <ellipse cx=CENTER_X cy=CENTER_Y rx=RADIUS_X ry=RADIUS_Y class="chain"/>
                {hail_mary_beads.into_iter().map(|(x, y)| view! {
                    <Bead x y kind=BeadKind::HailMary/>
                }).collect_view()}
                {our_father_beads.into_iter().map(|(x, y)| view! {
                    <Bead x y kind=BeadKind::OurFather/>
                }).collect_view()}
                <line x1="260" y1="285" x2="260" y2="375" class="chain"/>
                <Bead x=260.0 y=285.0 kind=BeadKind::OurFather/>
                <Bead x=260.0 y=315.0 kind=BeadKind::Pendant/>
                <Bead x=260.0 y=330.0 kind=BeadKind::Pendant/>
                <Bead x=260.0 y=345.0 kind=BeadKind::Pendant/>
                <Bead x=260.0 y=363.0 kind=BeadKind::OurFather/>
                <path d="M254 377h12v13h10v7h-10v21h-12v-21h-10v-7h10z" class="cross"/>
                <text x="260" y="157" text-anchor="middle" class="diagram-title">{move || copy.get().hail_mary}</text>
                <text x="260" y="174" text-anchor="middle" class="diagram-copy">"10 × 5"</text>
                <text x="403" y="169" class="diagram-copy">{move || copy.get().our_father}</text>
            </svg>
        </div>
    }
}
