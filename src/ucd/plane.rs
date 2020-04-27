use crate::ucd::Range;

const PLANE_COUNT: u32 = 17;
const PLANE_SIZE: u32 = 0x1_0000;
const PLANE_NAMES: &[&str] = &[
    "Basic Multilingual Plane",
    "Supplementary Multilingual Plane",
    "Supplementary Ideographic Plane",
    "Tertiary Ideographic Plane",
    "Unassigned (Plane 4)",
    "Unassigned (Plane 5)",
    "Unassigned (Plane 6)",
    "Unassigned (Plane 7)",
    "Unassigned (Plane 8)",
    "Unassigned (Plane 9)",
    "Unassigned (Plane 10)",
    "Unassigned (Plane 11)",
    "Unassigned (Plane 12)",
    "Unassigned (Plane 13)",
    "Supplementary Special-purpose Plane",
    "Supplementary Private Use Area (Plane 15)",
    "Supplementary Private Use Area (Plane 16)",
];

pub struct Plane {
    pub name: &'static str,
    pub range: Range,
}

impl Plane {
    pub fn of(chr: char) -> Self {
        assert_eq!(PLANE_NAMES.len(), PLANE_COUNT as usize);

        let plane_index = chr as u32 / PLANE_SIZE;
        assert!(plane_index < PLANE_COUNT);

        Plane {
            name: PLANE_NAMES[plane_index as usize],
            range: Range {
                start: plane_index * PLANE_SIZE,
                end: (plane_index + 1) * PLANE_SIZE - 1,
            },
        }
    }
}

// TODO: Unit tests
