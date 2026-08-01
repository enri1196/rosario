use crate::i18n::Translation;
use leptos::prelude::*;

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

    // Each of the five 72° sections begins with an Our Father bead and
    // contains exactly ten Hail Mary beads before the next section.
    let our_father_beads = (0..5)
        .map(|decade| point_on_ring(-90.0 + f64::from(decade) * 72.0))
        .collect::<Vec<_>>();
    let hail_mary_beads = (0..5)
        .flat_map(|decade| {
            (1..=10).map(move |bead| {
                point_on_ring(-90.0 + f64::from(decade) * 72.0 + f64::from(bead) * (72.0 / 11.0))
            })
        })
        .collect::<Vec<_>>();

    view! {
        <div class="rosary-wrap">
            <svg class="rosary" viewBox="0 0 520 420" role="img" aria-labelledby="rosary-title rosary-desc">
                <title id="rosary-title">{move || copy.get().guide_title}</title>
                <desc id="rosary-desc">{move || copy.get().decade_note}</desc>
                <defs>
                    <radialGradient id="bead" cx="35%" cy="30%" r="65%">
                        <stop offset="0%" stop-color="#eadb92"/><stop offset="100%" stop-color="#675022"/>
                    </radialGradient>
                    <filter id="glow"><feGaussianBlur stdDeviation="1.2" result="blur"/><feMerge><feMergeNode in="blur"/><feMergeNode in="SourceGraphic"/></feMerge></filter>
                </defs>
                <ellipse cx=CENTER_X cy=CENTER_Y rx=RADIUS_X ry=RADIUS_Y class="chain"/>
                {hail_mary_beads.into_iter().map(|(x, y)| view! {
                    <circle cx=x cy=y r="3.8" class="bead hail-mary-bead"/>
                }).collect_view()}
                {our_father_beads.into_iter().map(|(x, y)| view! {
                    <circle cx=x cy=y r="7" class="large-bead our-father-bead"/>
                }).collect_view()}
                <line x1="258" y1="285" x2="258" y2="375" class="chain"/>
                <circle cx="258" cy="315" r="7" class="large-bead"/>
                <circle cx="258" cy="337" r="4" class="bead"/><circle cx="258" cy="350" r="4" class="bead"/><circle cx="258" cy="363" r="4" class="bead"/>
                <path d="M252 377h12v13h10v7h-10v21h-12v-21h-10v-7h10z" class="cross"/>
                <text x="258" y="157" text-anchor="middle" class="diagram-title">{move || copy.get().hail_mary}</text>
                <text x="258" y="174" text-anchor="middle" class="diagram-copy">"10 × 5"</text>
                <text x="403" y="169" class="diagram-copy">{move || copy.get().our_father}</text>
            </svg>
        </div>
    }
}
