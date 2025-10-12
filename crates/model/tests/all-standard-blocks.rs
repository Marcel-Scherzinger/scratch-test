use std::collections::{BTreeSet, HashMap};

use model::{ProjectDoc, UnsupportedBlockKind, json_from_sb3_stream};

#[test]
fn read_all_standard_blocks() {
    let mut sb3_file =
        std::fs::File::open("../../sb3/all-predefined-blocks.sb3").expect("file to be present");

    let json_data = json_from_sb3_stream(&mut sb3_file).unwrap();

    let res = ProjectDoc::from_json(json_data).expect("valid document");

    let doc = match res.ensure_no_invalid_blocks() {
        Ok(doc) => {
            let invalid: HashMap<_, _> = doc.invalid_blocks().collect();
            panic!("document shouldn't be totally valid: {invalid:#?}\n{doc:#?}")
        }
        Err(doc) => doc,
    };

    assert!(doc.invalid_blocks().next().is_some());

    if doc.unknown_blocks().next().is_some() {
        let unknown: HashMap<_, _> = doc.unknown_blocks().collect();
        let ids: BTreeSet<_> = unknown.values().collect();

        panic!("document shouldn't contain unknown blocks: {ids:#?}\n{unknown:#?}");
    }
    use itertools::Itertools;

    let sorted_expected = ALL_UNSUPPORTED_STD_BLOCKS
        .iter()
        .sorted()
        .cloned()
        .unique()
        .collect_vec();
    let sorted_found = doc
        .unsupported_blocks()
        .map(|(_, o)| o.clone())
        .unique()
        .sorted()
        .collect_vec();

    assert!(
        sorted_found
            .iter()
            .contains(&&UnsupportedBlockKind::SensingTouchingcolor)
    );
    assert!(
        sorted_found
            .iter()
            .contains(&&UnsupportedBlockKind::SensingLoudness)
    );

    for (a, b) in sorted_expected.iter().zip(&sorted_found) {
        assert_eq!(a, b);
    }

    assert_eq!(sorted_expected, sorted_found);

    assert_eq!(
        75,
        doc.unsupported_blocks().map(|(_, o)| o).unique().count()
    )
}

const ALL_UNSUPPORTED_STD_BLOCKS: [UnsupportedBlockKind; 75] = [
    UnsupportedBlockKind::EventWhenthisspriteclicked,
    UnsupportedBlockKind::EventBroadcast,
    UnsupportedBlockKind::EventBroadcastandwait,
    UnsupportedBlockKind::EventWhenbackdropswitchesto,
    UnsupportedBlockKind::EventWhenbroadcastreceived,
    UnsupportedBlockKind::EventWhengreaterthan,
    UnsupportedBlockKind::LooksSize,
    UnsupportedBlockKind::LooksChangeeffectby,
    UnsupportedBlockKind::LooksBackdropnumbername,
    UnsupportedBlockKind::LooksCostume,
    UnsupportedBlockKind::LooksCostumenumbername,
    UnsupportedBlockKind::LooksCleargraphiceffects,
    UnsupportedBlockKind::LooksBackdrops,
    UnsupportedBlockKind::LooksChangesizeby,
    UnsupportedBlockKind::LooksGoforwardbackwardlayers,
    UnsupportedBlockKind::LooksGotofrontback,
    UnsupportedBlockKind::LooksHide,
    UnsupportedBlockKind::LooksNextbackdrop,
    UnsupportedBlockKind::LooksNextcostume,
    UnsupportedBlockKind::LooksSeteffectto,
    UnsupportedBlockKind::LooksSetsizeto,
    UnsupportedBlockKind::LooksSwitchbackdropto,
    UnsupportedBlockKind::LooksSwitchcostumeto,
    UnsupportedBlockKind::MotionGlideto,
    UnsupportedBlockKind::MotionSetrotationstyle,
    UnsupportedBlockKind::MotionChangexby,
    UnsupportedBlockKind::MotionChangeyby,
    UnsupportedBlockKind::MotionSetx,
    UnsupportedBlockKind::MotionGotoxy,
    UnsupportedBlockKind::MotionGotoMenu,
    UnsupportedBlockKind::MotionPointtowards,
    UnsupportedBlockKind::MotionDirection,
    UnsupportedBlockKind::MotionGlidesecstoxy,
    UnsupportedBlockKind::MotionGlidetoMenu,
    UnsupportedBlockKind::MotionGoto,
    UnsupportedBlockKind::MotionIfonedgebounce,
    UnsupportedBlockKind::MotionMovesteps,
    UnsupportedBlockKind::MotionPointindirection,
    UnsupportedBlockKind::MotionPointtowardsMenu,
    UnsupportedBlockKind::MotionSety,
    UnsupportedBlockKind::MotionTurnleft,
    UnsupportedBlockKind::MotionTurnright,
    UnsupportedBlockKind::MotionXposition,
    UnsupportedBlockKind::MotionYposition,
    UnsupportedBlockKind::ControlDeleteThisClone,
    UnsupportedBlockKind::ControlCreateCloneOf,
    UnsupportedBlockKind::ControlCreateCloneOfMenu,
    UnsupportedBlockKind::ControlStartAsClone,
    UnsupportedBlockKind::SoundSoundsMenu,
    UnsupportedBlockKind::SoundVolume,
    UnsupportedBlockKind::SoundChangeeffectby,
    UnsupportedBlockKind::SoundChangevolumeby,
    UnsupportedBlockKind::SoundCleareffects,
    UnsupportedBlockKind::SoundPlay,
    UnsupportedBlockKind::SoundPlayuntildone,
    UnsupportedBlockKind::SoundSeteffectto,
    UnsupportedBlockKind::SoundSetvolumeto,
    UnsupportedBlockKind::SoundStopallsounds,
    UnsupportedBlockKind::SensingKeypressed,
    UnsupportedBlockKind::SensingTouchingcolor,
    UnsupportedBlockKind::SensingLoudness,
    UnsupportedBlockKind::SensingColoristouchingcolor,
    UnsupportedBlockKind::SensingDayssince2000,
    UnsupportedBlockKind::SensingCurrent,
    UnsupportedBlockKind::SensingTimer,
    UnsupportedBlockKind::SensingOf,
    UnsupportedBlockKind::SensingOfObjectMenu,
    UnsupportedBlockKind::SensingKeyoptions,
    UnsupportedBlockKind::SensingMousedown,
    UnsupportedBlockKind::SensingMousex,
    UnsupportedBlockKind::SensingMousey,
    UnsupportedBlockKind::SensingSetdragmode,
    UnsupportedBlockKind::SensingTouchingobject,
    UnsupportedBlockKind::SensingTouchingobjectmenu,
    UnsupportedBlockKind::SensingUsername,
];
