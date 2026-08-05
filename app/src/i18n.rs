/// The storage key used for an explicitly selected language.
pub(crate) const LANGUAGE_STORAGE_KEY: &str = "rosary-language";

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Language {
    Italian,
    English,
}

impl Language {
    pub const ALL: [Self; 2] = [Self::Italian, Self::English];

    pub const fn code(self) -> &'static str {
        match self {
            Self::Italian => "it",
            Self::English => "en",
        }
    }

    pub const fn label(self) -> &'static str {
        match self {
            Self::Italian => "Italiano",
            Self::English => "English",
        }
    }

    /// Resolves a saved language preference into a supported language.
    ///
    /// Missing and invalid values preserve the application's Italian default.
    pub(crate) fn resolve(saved_language: Option<&str>) -> Self {
        match saved_language {
            Some("en") => Self::English,
            Some("it") | None | Some(_) => Self::Italian,
        }
    }

    /// Resolves the initial language from browser storage when it is available.
    ///
    /// Storage failures are deliberately non-fatal so language initialization
    /// can never prevent the application from rendering.
    #[cfg(target_arch = "wasm32")]
    pub(crate) fn from_browser() -> Self {
        let saved_language = web_sys::window()
            .and_then(|window| window.local_storage().ok().flatten())
            .and_then(|storage| storage.get_item(LANGUAGE_STORAGE_KEY).ok().flatten());

        Self::resolve(saved_language.as_deref())
    }

    /// Returns the Italian fallback outside a browser build.
    #[cfg(not(target_arch = "wasm32"))]
    pub(crate) fn from_browser() -> Self {
        Self::resolve(None)
    }

    pub fn from_code(code: &str) -> Self {
        Self::resolve(Some(code))
    }
}

/// Persists an explicit user choice when browser storage is available.
///
/// Browsers may block storage, especially in private browsing contexts, so
/// persistence errors are intentionally ignored.
#[cfg(target_arch = "wasm32")]
pub(crate) fn persist_language(language: Language) {
    if let Some(storage) =
        web_sys::window().and_then(|window| window.local_storage().ok().flatten())
    {
        let _ = storage.set_item(LANGUAGE_STORAGE_KEY, language.code());
    }
}

/// Does nothing when persistence is requested outside a browser build.
#[cfg(not(target_arch = "wasm32"))]
pub(crate) fn persist_language(_language: Language) {
    let _ = LANGUAGE_STORAGE_KEY;
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Prayer {
    pub title: &'static str,
    pub text: &'static str,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Mystery {
    pub icon: &'static str,
    pub title: &'static str,
    pub meditation: &'static str,
    pub fruit: &'static str,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct MysteryGroup {
    pub set: MysterySet,
    pub mysteries: &'static [Mystery; 5],
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct RecommendationBasisDescriptions {
    pub weekday: &'static str,
    pub advent: &'static str,
    pub christmas_period: &'static str,
    pub lent: &'static str,
    pub easter_season: &'static str,
    pub feast_override: &'static str,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub enum MysterySet {
    Joyful,
    Luminous,
    Sorrowful,
    Glorious,
}

impl MysterySet {
    pub const fn label(self, language: Language) -> &'static str {
        match (language, self) {
            (Language::Italian, Self::Joyful) => "Misteri Gaudiosi",
            (Language::Italian, Self::Luminous) => "Misteri Luminosi",
            (Language::Italian, Self::Sorrowful) => "Misteri Dolorosi",
            (Language::Italian, Self::Glorious) => "Misteri Gloriosi",
            (Language::English, Self::Joyful) => "Joyful Mysteries",
            (Language::English, Self::Luminous) => "Luminous Mysteries",
            (Language::English, Self::Sorrowful) => "Sorrowful Mysteries",
            (Language::English, Self::Glorious) => "Glorious Mysteries",
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub struct Translation {
    pub page_title: &'static str,
    pub heading: &'static str,
    pub language_label: &'static str,
    pub theme_control_label: &'static str,
    pub dark_theme_label: &'static str,
    pub light_theme_label: &'static str,
    pub skip_link: &'static str,
    pub prayers_heading: &'static str,
    pub prayers: &'static [Prayer; 5],
    pub sidebar_prayers: &'static [Prayer; 7],
    pub mystery_recommendation_title: &'static str,
    pub mystery_recommendation_pray_label: &'static str,
    pub mystery_recommendation_date_label: &'static str,
    pub mystery_recommendation_date_help: &'static str,
    pub mystery_recommendation_today_label: &'static str,
    pub mystery_recommendation_selected_date_label: &'static str,
    pub mystery_recommendation_reason_label: &'static str,
    pub mystery_recommendation_invalid_date: &'static str,
    pub mystery_recommendation_basis: RecommendationBasisDescriptions,
    pub creed_title: &'static str,
    pub creed: &'static str,
    pub guide_title: &'static str,
    pub our_father: &'static str,
    pub hail_mary: &'static str,
    pub steps: &'static [&'static str; 9],
    pub ending_title: &'static str,
    pub ending: &'static str,
    pub decade_note: &'static str,
    pub guided_start_label: &'static str,
    pub guided_title: &'static str,
    pub guided_close_label: &'static str,
    pub guided_step_label: &'static str,
    pub guided_of_label: &'static str,
    pub guided_mystery_set_label: &'static str,
    pub guided_selected_mystery_label: &'static str,
    pub guided_decade_label: &'static str,
    pub guided_ten_hail_marys_label: &'static str,
    pub guided_previous_label: &'static str,
    pub guided_next_label: &'static str,
    pub guided_reset_label: &'static str,
    pub guided_finish_label: &'static str,
    pub guided_restart_label: &'static str,
    pub guided_completion_title: &'static str,
    pub guided_completion_message: &'static str,
    pub post_rosary_prayers_title: &'static str,
    pub post_rosary_prayers: &'static [Prayer; 4],
    pub guided_pray_mystery_label: &'static str,
    pub guided_intention_label: &'static str,
    pub intention_title: &'static str,
    pub intention_label: &'static str,
    pub intention_help: &'static str,
    pub intention_add_label: &'static str,
    pub intention_list_label: &'static str,
    pub intention_item_label: &'static str,
    pub intention_reorder_label: &'static str,
    pub intention_delete_label: &'static str,
    pub intention_count_label: &'static str,
    pub intention_character_count_label: &'static str,
    pub intention_empty_state: &'static str,
    pub intention_too_long_error: &'static str,
    pub intention_too_many_error: &'static str,
    pub intention_duplicate_error: &'static str,
    pub intention_added_status: &'static str,
    pub intention_removed_status: &'static str,
    pub intention_reordered_status: &'static str,
    pub intention_storage_error: &'static str,
    pub mysteries_heading: &'static str,
    pub groups: &'static [MysteryGroup; 4],
    pub fruit_label: &'static str,
    pub version: &'static str,
}

impl Translation {
    pub const fn for_language(language: Language) -> Self {
        match language {
            Language::Italian => IT,
            Language::English => EN,
        }
    }
}

const IT_PRAYERS: [Prayer; 5] = [
    Prayer { title: "Padre Nostro", text: "Padre nostro, che sei nei cieli,\nsia santificato il tuo nome;\nvenga il tuo regno;\nsia fatta la tua volontà,\ncome in cielo così in terra.\nDacci oggi il nostro pane quotidiano,\ne rimetti a noi i nostri debiti\ncome noi li rimettiamo ai nostri debitori,\ne non ci indurre in tentazione,\nma liberaci dal male. Amen." },
    Prayer { title: "Ave Maria", text: "Ave Maria, piena di grazia,\nil Signore è con te.\nTu sei benedetta fra le donne\ne benedetto è il frutto del tuo seno, Gesù.\nSanta Maria, Madre di Dio,\nprega per noi peccatori,\nadesso e nell'ora della nostra morte. Amen." },
    Prayer { title: "Gloria al Padre", text: "Gloria al Padre, al Figlio\ne allo Spirito Santo.\nCome era nel principio,\nora e sempre,\nnei secoli dei secoli. Amen." },
    Prayer { title: "O Mio Gesù", text: "O Gesù mio, perdona i nostri peccati,\npreservaci dal fuoco dell'inferno,\nporta in cielo tutte le anime,\nspecialmente le più bisognose\ndella tua misericordia. Amen." },
    Prayer { title: "Salve Regina", text: "Salve, Regina, madre di misericordia,\nvita, dolcezza e speranza nostra, salve.\nA te ricorriamo, esuli figli di Eva;\na te sospiriamo gementi e piangenti\nin questa valle di lacrime.\nOrsù dunque, avvocata nostra,\nrivolgi a noi gli occhi tuoi misericordiosi.\nE mostraci, dopo questo esilio,\nGesù, il frutto benedetto del tuo seno.\nO clemente, o pia, o dolce Vergine Maria.\nPrega per noi, santa Madre di Dio,\nperché siamo degni delle promesse di Cristo." },
];

const EN_PRAYERS: [Prayer; 5] = [
    Prayer { title: "Our Father", text: "Our Father, who art in heaven,\nhallowed be thy name;\nthy kingdom come; thy will be done\non earth as it is in heaven.\nGive us this day our daily bread,\nand forgive us our trespasses,\nas we forgive those who trespass against us;\nand lead us not into temptation,\nbut deliver us from evil. Amen." },
    Prayer { title: "Hail Mary", text: "Hail Mary, full of grace,\nthe Lord is with thee.\nBlessed art thou among women,\nand blessed is the fruit of thy womb, Jesus.\nHoly Mary, Mother of God,\npray for us sinners,\nnow and at the hour of our death. Amen." },
    Prayer { title: "Glory Be", text: "Glory be to the Father, and to the Son,\nand to the Holy Spirit.\nAs it was in the beginning,\nis now, and ever shall be,\nworld without end. Amen." },
    Prayer { title: "O My Jesus", text: "O my Jesus, forgive us our sins,\nsave us from the fires of hell,\nlead all souls to Heaven,\nespecially those most in need\nof thy mercy. Amen." },
    Prayer { title: "Hail, Holy Queen", text: "Hail, holy Queen, Mother of mercy,\nour life, our sweetness and our hope.\nTo thee do we cry, poor banished children of Eve;\nto thee do we send up our sighs,\nmourning and weeping in this valley of tears.\nTurn then, most gracious advocate,\nthine eyes of mercy toward us;\nand after this our exile show unto us\nthe blessed fruit of thy womb, Jesus.\nO clement, O loving, O sweet Virgin Mary.\nPray for us, O holy Mother of God,\nthat we may be made worthy of Christ's promises." },
];

const IT_SIDEBAR_PRAYERS: [Prayer; 7] = [
    Prayer { title: "Credo degli Apostoli", text: "Credo in Dio, Padre onnipotente, Creatore del cielo e della terra; e in Gesù Cristo, suo unico Figlio, nostro Signore, il quale fu concepito di Spirito Santo, nacque da Maria Vergine, patì sotto Ponzio Pilato, fu crocifisso, morì e fu sepolto; discese agli inferi; il terzo giorno risuscitò da morte; salì al cielo, siede alla destra di Dio Padre onnipotente; di là verrà a giudicare i vivi e i morti. Credo nello Spirito Santo, la santa Chiesa cattolica, la comunione dei santi, la remissione dei peccati, la risurrezione della carne, la vita eterna. Amen." },
    IT_PRAYERS[0],
    IT_PRAYERS[1],
    IT_PRAYERS[2],
    IT_PRAYERS[3],
    IT_PRAYERS[4],
    Prayer { title: "L'Eterno Riposo", text: "L'eterno riposo dona loro, o Signore,\ne splenda ad essi la luce perpetua.\nRiposino in pace. Amen." },
];

const EN_SIDEBAR_PRAYERS: [Prayer; 7] = [
    Prayer { title: "The Apostles' Creed", text: "I believe in God, the Father almighty, Creator of heaven and earth, and in Jesus Christ, his only Son, our Lord, who was conceived by the Holy Spirit, born of the Virgin Mary, suffered under Pontius Pilate, was crucified, died and was buried; he descended into hell; on the third day he rose again from the dead; he ascended into heaven, and is seated at the right hand of God the Father almighty; from there he will come to judge the living and the dead. I believe in the Holy Spirit, the holy catholic Church, the communion of saints, the forgiveness of sins, the resurrection of the body, and life everlasting. Amen." },
    EN_PRAYERS[0],
    EN_PRAYERS[1],
    EN_PRAYERS[2],
    EN_PRAYERS[3],
    EN_PRAYERS[4],
    Prayer { title: "Eternal Rest", text: "Eternal rest grant unto them, O Lord,\nand let perpetual light shine upon them.\nMay they rest in peace. Amen." },
];

const IT_POST_ROSARY_PRAYERS: [Prayer; 4] = [
    Prayer {
        title: "Tre L'Eterno Riposo per le anime del Purgatorio",
        text: "Da recitare tre volte:\nL'eterno riposo dona loro, o Signore,\ne splenda ad essi la luce perpetua.\nRiposino in pace. Amen.",
    },
    Prayer {
        title: "Per le intenzioni del Santo Padre",
        text: "1 Padre Nostro · 1 Ave Maria · 1 Gloria al Padre\n(offerti secondo le intenzioni del Papa, per ottenere le indulgenze quando previste).",
    },
    Prayer {
        title: "Preghiera all'Angelo Custode",
        text: "Angelo di Dio,\nche sei il mio custode,\nillumina, custodisci, reggi e governa me,\nche ti fui affidato dalla pietà celeste.\nAmen.",
    },
    Prayer {
        title: "Segno della Croce",
        text: "Nel nome del Padre e del Figlio e dello Spirito Santo. Amen.",
    },
];

const EN_POST_ROSARY_PRAYERS: [Prayer; 4] = [
    Prayer {
        title: "Three Eternal Rests for the souls in Purgatory",
        text: "Pray three times:\nEternal rest grant unto them, O Lord,\nand let perpetual light shine upon them.\nMay they rest in peace. Amen.",
    },
    Prayer {
        title: "For the intentions of the Holy Father",
        text: "One Our Father · one Hail Mary · one Glory Be\n(Offered for the Pope's intentions, to obtain indulgences where provided.)",
    },
    Prayer {
        title: "Prayer to the Guardian Angel",
        text: "Angel of God,\nmy guardian dear,\nto whom God's love commits me here,\never this day be at my side,\nto light and guard,\nto rule and guide. Amen.",
    },
    Prayer {
        title: "Sign of the Cross",
        text: "In the name of the Father, and of the Son, and of the Holy Spirit. Amen.",
    },
];

const IT_JOYFUL: [Mystery; 5] = [
    Mystery { icon: "🕊️", title: "L'Annunciazione", meditation: "L'arcangelo Gabriele annuncia a Maria che concepirà il Figlio di Dio. Maria risponde: «Ecco la serva del Signore: avvenga per me secondo la tua parola» (Lc 1,26-38).", fruit: "Umiltà" },
    Mystery { icon: "🤝", title: "La Visitazione", meditation: "Maria va a trovare sua cugina Elisabetta, che esclama: «Benedetta tu fra le donne e benedetto il frutto del tuo grembo» (Lc 1,39-56).", fruit: "Amore del prossimo" },
    Mystery { icon: "⭐", title: "La Natività", meditation: "Gesù nasce a Betlemme, portando salvezza al mondo. Maria lo depone in una mangiatoia perché non c'era posto nell'alloggio (Lc 2,1-20).", fruit: "Povertà" },
    Mystery { icon: "🕯️", title: "La Presentazione", meditation: "Maria e Giuseppe presentano Gesù al Tempio secondo la Legge. Simeone profetizza sulla caduta e risurrezione di molti in Israele (Lc 2,22-40).", fruit: "Obbedienza" },
    Mystery { icon: "🔍", title: "Il Ritrovamento al Tempio", meditation: "Gesù dodicenne viene ritrovato al Tempio tra i dottori: «Non sapevate che io devo occuparmi delle cose del Padre mio?» (Lc 2,41-52).", fruit: "Gioia nel ritrovamento" },
];
const IT_LUMINOUS: [Mystery; 5] = [
    Mystery { icon: "💧", title: "Il Battesimo al Giordano", meditation: "Gesù è battezzato e i cieli si aprono. Il Padre proclama: «Questi è il Figlio mio prediletto, nel quale mi sono compiaciuto» (Mt 3,13-17).", fruit: "Apertura allo Spirito" },
    Mystery { icon: "🍷", title: "Le Nozze di Cana", meditation: "Gesù compie il primo segno trasformando l'acqua in vino. Maria dice ai servi: «Qualsiasi cosa vi dica, fatela» (Gv 2,1-12).", fruit: "A Gesù per Maria" },
    Mystery { icon: "📜", title: "L'Annuncio del Regno", meditation: "Gesù proclama: «Il tempo è compiuto e il regno di Dio è vicino; convertitevi e credete al Vangelo» (Mc 1,14-15).", fruit: "Conversione" },
    Mystery { icon: "☀️", title: "La Trasfigurazione", meditation: "Gesù si trasfigura sul monte: il suo volto brilla come il sole e le sue vesti diventano candide come la luce (Mt 17,1-13).", fruit: "Desiderio di santità" },
    Mystery { icon: "🍞", title: "L'Eucaristia", meditation: "All'Ultima Cena Gesù istituisce l'Eucaristia: «Questo è il mio corpo che è dato per voi; fate questo in memoria di me» (Lc 22,14-20).", fruit: "Amore all'Eucaristia" },
];
const IT_SORROWFUL: [Mystery; 5] = [
    Mystery { icon: "🌿", title: "L'Agonia nell'Orto", meditation: "Gesù prega nel Getsemani: «Padre, se vuoi, allontana da me questo calice; tuttavia non sia fatta la mia, ma la tua volontà» (Lc 22,39-46).", fruit: "Conformità alla volontà di Dio" },
    Mystery { icon: "⛓️", title: "La Flagellazione", meditation: "Gesù è flagellato su comando di Pilato e poi consegnato perché sia crocifisso (Mt 27,24-26).", fruit: "Mortificazione" },
    Mystery { icon: "👑", title: "La Coronazione di Spine", meditation: "I soldati intrecciano una corona di spine e la pongono sul capo di Gesù, schernendolo: «Salve, re dei Giudei!» (Mt 27,27-31).", fruit: "Coraggio" },
    Mystery { icon: "✝️", title: "La Via Crucis", meditation: "Gesù porta la croce: «Non piangete su di me, ma piangete su voi stesse e sui vostri figli» (Lc 23,26-31).", fruit: "Pazienza" },
    Mystery { icon: "💔", title: "La Crocifissione", meditation: "Gesù è crocifisso e prega: «Padre, perdona loro perché non sanno quello che fanno» (Lc 23,33-46).", fruit: "Perdono" },
];
const IT_GLORIOUS: [Mystery; 5] = [
    Mystery { icon: "✨", title: "La Risurrezione", meditation: "Gesù risorge dai morti. L'angelo annuncia: «Non è qui; è risorto» (Mt 28,1-10).", fruit: "Fede" },
    Mystery { icon: "☁️", title: "L'Ascensione", meditation: "Gesù sale al cielo e affida agli apostoli la missione di essergli testimoni fino ai confini della terra (At 1,6-11).", fruit: "Speranza" },
    Mystery { icon: "🔥", title: "La Pentecoste", meditation: "Lo Spirito Santo discende sugli Apostoli. Tutti ne sono colmati e cominciano a parlare in altre lingue (At 2,1-13).", fruit: "Sapienza" },
    Mystery { icon: "🌸", title: "L'Assunzione di Maria", meditation: "Maria è assunta in corpo e anima in cielo: «D'ora in poi tutte le generazioni mi chiameranno beata» (Lc 1,48-49).", fruit: "Devozione a Maria" },
    Mystery { icon: "👑", title: "L'Incoronazione di Maria", meditation: "Maria è incoronata Regina del Cielo, la donna vestita di sole con una corona di dodici stelle (Ap 12,1).", fruit: "Felicità eterna" },
];

const EN_JOYFUL: [Mystery; 5] = [
    Mystery { icon: "🕊️", title: "The Annunciation", meditation: "The angel Gabriel announces that Mary will conceive the Son of God. Mary answers: “Let it be done to me according to your word” (Lk 1:26–38).", fruit: "Humility" },
    Mystery { icon: "🤝", title: "The Visitation", meditation: "Mary visits Elizabeth, who cries out: “Blessed are you among women, and blessed is the fruit of your womb” (Lk 1:39–56).", fruit: "Love of neighbor" },
    Mystery { icon: "⭐", title: "The Nativity", meditation: "Jesus is born in Bethlehem, bringing salvation to the world. Mary lays him in a manger because there is no room at the inn (Lk 2:1–20).", fruit: "Poverty of spirit" },
    Mystery { icon: "🕯️", title: "The Presentation", meditation: "Mary and Joseph present Jesus in the Temple according to the Law. Simeon prophesies over the child (Lk 2:22–40).", fruit: "Obedience" },
    Mystery { icon: "🔍", title: "The Finding in the Temple", meditation: "The twelve-year-old Jesus is found among the teachers: “Did you not know that I must be in my Father's house?” (Lk 2:41–52).", fruit: "Joy in finding Jesus" },
];
const EN_LUMINOUS: [Mystery; 5] = [
    Mystery { icon: "💧", title: "The Baptism in the Jordan", meditation: "Jesus is baptized and heaven opens. The Father proclaims: “This is my beloved Son, with whom I am well pleased” (Mt 3:13–17).", fruit: "Openness to the Holy Spirit" },
    Mystery { icon: "🍷", title: "The Wedding at Cana", meditation: "Jesus performs his first sign, changing water into wine. Mary tells the servants: “Do whatever he tells you” (Jn 2:1–12).", fruit: "To Jesus through Mary" },
    Mystery { icon: "📜", title: "The Proclamation of the Kingdom", meditation: "Jesus proclaims: “The kingdom of God is at hand. Repent, and believe in the gospel” (Mk 1:14–15).", fruit: "Conversion" },
    Mystery { icon: "☀️", title: "The Transfiguration", meditation: "Jesus is transfigured on the mountain; his face shines like the sun and his clothes become white as light (Mt 17:1–13).", fruit: "Desire for holiness" },
    Mystery { icon: "🍞", title: "The Institution of the Eucharist", meditation: "At the Last Supper Jesus says: “This is my body, which will be given for you; do this in memory of me” (Lk 22:14–20).", fruit: "Love of the Eucharist" },
];
const EN_SORROWFUL: [Mystery; 5] = [
    Mystery { icon: "🌿", title: "The Agony in the Garden", meditation: "Jesus prays in Gethsemane: “Father, not my will but yours be done” (Lk 22:39–46).", fruit: "Conformity to God's will" },
    Mystery { icon: "⛓️", title: "The Scourging at the Pillar", meditation: "At Pilate's command Jesus is scourged and handed over to be crucified (Mt 27:24–26).", fruit: "Purity and mortification" },
    Mystery { icon: "👑", title: "The Crowning with Thorns", meditation: "The soldiers weave a crown of thorns, place it on Jesus' head and mock him: “Hail, King of the Jews!” (Mt 27:27–31).", fruit: "Moral courage" },
    Mystery { icon: "✝️", title: "The Carrying of the Cross", meditation: "Jesus carries the cross and says: “Do not weep for me; weep instead for yourselves and for your children” (Lk 23:26–31).", fruit: "Patience" },
    Mystery { icon: "💔", title: "The Crucifixion", meditation: "Jesus is crucified and prays: “Father, forgive them; they know not what they do” (Lk 23:33–46).", fruit: "Forgiveness" },
];
const EN_GLORIOUS: [Mystery; 5] = [
    Mystery { icon: "✨", title: "The Resurrection", meditation: "Jesus rises from the dead. The angel announces: “He is not here, for he has been raised” (Mt 28:1–10).", fruit: "Faith" },
    Mystery { icon: "☁️", title: "The Ascension", meditation: "Jesus ascends into heaven and entrusts the apostles with being his witnesses to the ends of the earth (Acts 1:6–11).", fruit: "Hope" },
    Mystery { icon: "🔥", title: "The Descent of the Holy Spirit", meditation: "The Holy Spirit descends upon the Apostles. They are filled with the Spirit and begin to speak in other tongues (Acts 2:1–13).", fruit: "Wisdom" },
    Mystery { icon: "🌸", title: "The Assumption of Mary", meditation: "Mary is taken body and soul into heaven: “From now on all ages will call me blessed” (Lk 1:48–49).", fruit: "Devotion to Mary" },
    Mystery { icon: "👑", title: "The Coronation of Mary", meditation: "Mary is crowned Queen of Heaven, the woman clothed with the sun and wearing a crown of twelve stars (Rv 12:1).", fruit: "Eternal happiness" },
];

const IT_GROUPS: [MysteryGroup; 4] = [
    MysteryGroup {
        set: MysterySet::Joyful,
        mysteries: &IT_JOYFUL,
    },
    MysteryGroup {
        set: MysterySet::Luminous,
        mysteries: &IT_LUMINOUS,
    },
    MysteryGroup {
        set: MysterySet::Sorrowful,
        mysteries: &IT_SORROWFUL,
    },
    MysteryGroup {
        set: MysterySet::Glorious,
        mysteries: &IT_GLORIOUS,
    },
];
const EN_GROUPS: [MysteryGroup; 4] = [
    MysteryGroup {
        set: MysterySet::Joyful,
        mysteries: &EN_JOYFUL,
    },
    MysteryGroup {
        set: MysterySet::Luminous,
        mysteries: &EN_LUMINOUS,
    },
    MysteryGroup {
        set: MysterySet::Sorrowful,
        mysteries: &EN_SORROWFUL,
    },
    MysteryGroup {
        set: MysterySet::Glorious,
        mysteries: &EN_GLORIOUS,
    },
];

pub const IT: Translation = Translation {
    page_title: "Guida al Rosario", heading: "Guida al Rosario", language_label: "Lingua", theme_control_label: "Tema", dark_theme_label: "Passa al tema scuro", light_theme_label: "Passa al tema chiaro", skip_link: "Vai al contenuto", prayers_heading: "Preghiere del Rosario", prayers: &IT_PRAYERS, sidebar_prayers: &IT_SIDEBAR_PRAYERS,
    mystery_recommendation_title: "Misteri consigliati", mystery_recommendation_pray_label: "Misteri da pregare",
    mystery_recommendation_date_label: "Scegli una data (YYYY-MM-DD)", mystery_recommendation_date_help: "Usa il formato YYYY-MM-DD per esplorare i Misteri consigliati per un altro giorno.", mystery_recommendation_today_label: "Oggi", mystery_recommendation_selected_date_label: "Data selezionata", mystery_recommendation_reason_label: "Perché", mystery_recommendation_invalid_date: "Inserisci una data valida nel formato YYYY-MM-DD: la raccomandazione precedente non è stata modificata.",
    mystery_recommendation_basis: RecommendationBasisDescriptions { weekday: "La raccomandazione segue il giorno della settimana.", advent: "Nel tempo di Avvento si contemplano i Misteri Gaudiosi.", christmas_period: "Nel tempo di Natale si contemplano i Misteri Gaudiosi.", lent: "Nel tempo di Quaresima si contemplano i Misteri Dolorosi.", easter_season: "Nel tempo di Pasqua si contemplano i Misteri Gloriosi.", feast_override: "La celebrazione liturgica di questo giorno determina i Misteri consigliati." },
    creed_title: "Credo degli Apostoli", creed: "Credo in Dio, Padre onnipotente, Creatore del cielo e della terra; e in Gesù Cristo, suo unico Figlio, nostro Signore, il quale fu concepito di Spirito Santo, nacque da Maria Vergine, patì sotto Ponzio Pilato, fu crocifisso, morì e fu sepolto; discese agli inferi; il terzo giorno risuscitò da morte; salì al cielo, siede alla destra di Dio Padre onnipotente; di là verrà a giudicare i vivi e i morti. Credo nello Spirito Santo, la santa Chiesa cattolica, la comunione dei santi, la remissione dei peccati, la risurrezione della carne, la vita eterna. Amen.",
    guide_title: "Come recitare il Rosario", our_father: "Padre Nostro", hail_mary: "Ave Maria",
    steps: &["Segno della Croce", "Credo degli Apostoli", "Padre Nostro", "Tre Ave Maria", "Gloria al Padre", "O Mio Gesù", "Intenzione", "Annunciare e meditare il Mistero", "Ripetere per cinque decine"],
    ending_title: "Al termine", ending: "Salve Regina · Preghiera personale · Segno della Croce", decade_note: "Per ogni decina: annuncia il Mistero e meditalo mentre reciti un Padre Nostro, dieci Ave Maria, il Gloria e O Mio Gesù.",
    guided_start_label: "Avvia il Rosario guidato", guided_title: "Rosario guidato", guided_close_label: "Chiudi il Rosario guidato", guided_step_label: "Passo", guided_of_label: "di", guided_mystery_set_label: "Misteri scelti", guided_selected_mystery_label: "Mistero scelto", guided_decade_label: "Decina", guided_ten_hail_marys_label: "Dieci Ave Maria", guided_previous_label: "Indietro", guided_next_label: "Avanti", guided_reset_label: "Ricomincia", guided_finish_label: "Completa il Rosario", guided_restart_label: "Prega di nuovo", guided_completion_title: "Rosario completato", guided_completion_message: "Hai completato il Rosario. Prenditi un momento di silenzio prima di continuare.", post_rosary_prayers_title: "Preghiere facoltative dopo il Rosario", post_rosary_prayers: &IT_POST_ROSARY_PRAYERS, guided_pray_mystery_label: "Prega questo Mistero", guided_intention_label: "Intenzioni affidate",
    intention_title: "Intenzioni di preghiera", intention_label: "Nuova intenzione", intention_help: "Usa + per creare un'etichetta, poi premi Invio o fai clic altrove per confermarla. Rimane privata in questo browser; trascinala o usa i tasti freccia per riordinarla.", intention_add_label: "Aggiungi intenzione", intention_list_label: "Intenzioni salvate", intention_item_label: "Intenzione", intention_reorder_label: "Trascina per riordinare o usa i tasti freccia", intention_delete_label: "Elimina intenzione", intention_count_label: "Intenzioni", intention_character_count_label: "Caratteri", intention_empty_state: "Nessuna intenzione aggiunta.", intention_too_long_error: "Ogni intenzione non può superare 50 caratteri.", intention_too_many_error: "Puoi aggiungere al massimo 50 intenzioni.", intention_duplicate_error: "Questa intenzione è già presente.", intention_added_status: "Intenzione aggiunta.", intention_removed_status: "Intenzione eliminata.", intention_reordered_status: "Ordine delle intenzioni aggiornato.", intention_storage_error: "La modifica è attiva in questa pagina, ma il browser non ha permesso di salvarla.",
    mysteries_heading: "I venti Misteri", groups: &IT_GROUPS, fruit_label: "Frutto del Mistero", version: "Versione italiana",
};

pub const EN: Translation = Translation {
    page_title: "Guide to the Rosary", heading: "Guide to the Rosary", language_label: "Language", theme_control_label: "Theme", dark_theme_label: "Switch to dark theme", light_theme_label: "Switch to light theme", skip_link: "Skip to content", prayers_heading: "Prayers of the Rosary", prayers: &EN_PRAYERS, sidebar_prayers: &EN_SIDEBAR_PRAYERS,
    mystery_recommendation_title: "Recommended Mysteries", mystery_recommendation_pray_label: "Mysteries to pray",
    mystery_recommendation_date_label: "Choose a date (YYYY-MM-DD)", mystery_recommendation_date_help: "Use YYYY-MM-DD to explore the Mysteries recommended for another day.", mystery_recommendation_today_label: "Today", mystery_recommendation_selected_date_label: "Selected date", mystery_recommendation_reason_label: "Why", mystery_recommendation_invalid_date: "Enter a valid date in YYYY-MM-DD format; the previous recommendation has not changed.",
    mystery_recommendation_basis: RecommendationBasisDescriptions { weekday: "The recommendation follows the day of the week.", advent: "During Advent, the Joyful Mysteries are contemplated.", christmas_period: "During the Christmas season, the Joyful Mysteries are contemplated.", lent: "During Lent, the Sorrowful Mysteries are contemplated.", easter_season: "During the Easter season, the Glorious Mysteries are contemplated.", feast_override: "The liturgical celebration on this date determines the recommended Mysteries." },
    creed_title: "The Apostles' Creed", creed: "I believe in God, the Father almighty, Creator of heaven and earth, and in Jesus Christ, his only Son, our Lord, who was conceived by the Holy Spirit, born of the Virgin Mary, suffered under Pontius Pilate, was crucified, died and was buried; he descended into hell; on the third day he rose again from the dead; he ascended into heaven, and is seated at the right hand of God the Father almighty; from there he will come to judge the living and the dead. I believe in the Holy Spirit, the holy catholic Church, the communion of saints, the forgiveness of sins, the resurrection of the body, and life everlasting. Amen.",
    guide_title: "How to pray the Rosary", our_father: "Our Father", hail_mary: "Hail Mary",
    steps: &["Sign of the Cross", "Apostles' Creed", "Our Father", "Three Hail Marys", "Glory Be", "O My Jesus", "Prayer intention", "Announce and meditate on the Mystery", "Repeat for five decades"],
    ending_title: "To conclude", ending: "Hail, Holy Queen · Personal prayer · Sign of the Cross", decade_note: "For each decade: announce the Mystery and meditate on it while praying one Our Father, ten Hail Marys, the Glory Be and O My Jesus.",
    guided_start_label: "Start guided Rosary", guided_title: "Guided Rosary", guided_close_label: "Close guided Rosary", guided_step_label: "Step", guided_of_label: "of", guided_mystery_set_label: "Selected Mysteries", guided_selected_mystery_label: "Selected Mystery", guided_decade_label: "Decade", guided_ten_hail_marys_label: "Ten Hail Marys", guided_previous_label: "Previous", guided_next_label: "Next", guided_reset_label: "Reset", guided_finish_label: "Complete the Rosary", guided_restart_label: "Pray again", guided_completion_title: "Rosary complete", guided_completion_message: "You have completed the Rosary. Take a moment of silence before continuing.", post_rosary_prayers_title: "Optional prayers after the Rosary", post_rosary_prayers: &EN_POST_ROSARY_PRAYERS, guided_pray_mystery_label: "Pray this Mystery", guided_intention_label: "Prayer intentions",
    intention_title: "Prayer intentions", intention_label: "New intention", intention_help: "Use + to create a tag, then press Enter or click elsewhere to confirm it. It stays private in this browser; drag it or use the arrow keys to reorder it.", intention_add_label: "Add intention", intention_list_label: "Saved intentions", intention_item_label: "Intention", intention_reorder_label: "Drag to reorder or use the arrow keys", intention_delete_label: "Delete intention", intention_count_label: "Intentions", intention_character_count_label: "Characters", intention_empty_state: "No intentions added.", intention_too_long_error: "Each intention cannot exceed 50 characters.", intention_too_many_error: "You can add at most 50 intentions.", intention_duplicate_error: "This intention has already been added.", intention_added_status: "Intention added.", intention_removed_status: "Intention deleted.", intention_reordered_status: "Intention order updated.", intention_storage_error: "The change is active on this page, but the browser did not allow it to be saved.",
    mysteries_heading: "The twenty Mysteries", groups: &EN_GROUPS, fruit_label: "Fruit of the Mystery", version: "English version",
};

#[cfg(test)]
mod tests {
    use super::Language;

    #[test]
    fn resolves_saved_italian() {
        assert!(matches!(Language::resolve(Some("it")), Language::Italian));
    }

    #[test]
    fn resolves_saved_english() {
        assert!(matches!(Language::resolve(Some("en")), Language::English));
    }

    #[test]
    fn missing_preference_defaults_to_italian() {
        assert!(matches!(Language::resolve(None), Language::Italian));
    }

    #[test]
    fn invalid_preference_defaults_to_italian() {
        assert!(matches!(Language::resolve(Some("fr")), Language::Italian));
    }
}
