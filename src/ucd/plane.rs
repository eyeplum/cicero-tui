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

#[derive(Debug, Eq, PartialEq)]
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_of_planes() {
        assert_eq!(PLANE_COUNT, 17);
        assert_eq!(PLANE_NAMES.len(), 17);
    }

    #[test]
    fn test_plane_of_character() {
        assert_eq!(
            Plane::of('\u{0060}'),
            Plane {
                name: PLANE_NAMES[0],
                range: Range {
                    start: 0x0,
                    end: 0xFFFF
                }
            }
        );
        assert_eq!(
            Plane::of('\u{10060}'),
            Plane {
                name: PLANE_NAMES[1],
                range: Range {
                    start: 0x10000,
                    end: 0x1FFFF
                }
            }
        );
        assert_eq!(
            Plane::of('\u{20060}'),
            Plane {
                name: PLANE_NAMES[2],
                range: Range {
                    start: 0x20000,
                    end: 0x2FFFF
                }
            }
        );
        assert_eq!(
            Plane::of('\u{30060}'),
            Plane {
                name: PLANE_NAMES[3],
                range: Range {
                    start: 0x30000,
                    end: 0x3FFFF
                }
            }
        );
        assert_eq!(
            Plane::of('\u{40060}'),
            Plane {
                name: PLANE_NAMES[4],
                range: Range {
                    start: 0x40000,
                    end: 0x4FFFF
                }
            }
        );
        assert_eq!(
            Plane::of('\u{50060}'),
            Plane {
                name: PLANE_NAMES[5],
                range: Range {
                    start: 0x50000,
                    end: 0x5FFFF
                }
            }
        );
        assert_eq!(
            Plane::of('\u{60060}'),
            Plane {
                name: PLANE_NAMES[6],
                range: Range {
                    start: 0x60000,
                    end: 0x6FFFF
                }
            }
        );
        assert_eq!(
            Plane::of('\u{70060}'),
            Plane {
                name: PLANE_NAMES[7],
                range: Range {
                    start: 0x70000,
                    end: 0x7FFFF
                }
            }
        );
        assert_eq!(
            Plane::of('\u{80060}'),
            Plane {
                name: PLANE_NAMES[8],
                range: Range {
                    start: 0x80000,
                    end: 0x8FFFF
                }
            }
        );
        assert_eq!(
            Plane::of('\u{90060}'),
            Plane {
                name: PLANE_NAMES[9],
                range: Range {
                    start: 0x90000,
                    end: 0x9FFFF
                }
            }
        );
        assert_eq!(
            Plane::of('\u{A0060}'),
            Plane {
                name: PLANE_NAMES[10],
                range: Range {
                    start: 0xA0000,
                    end: 0xAFFFF
                }
            }
        );
        assert_eq!(
            Plane::of('\u{B0060}'),
            Plane {
                name: PLANE_NAMES[11],
                range: Range {
                    start: 0xB0000,
                    end: 0xBFFFF
                }
            }
        );
        assert_eq!(
            Plane::of('\u{C0060}'),
            Plane {
                name: PLANE_NAMES[12],
                range: Range {
                    start: 0xC0000,
                    end: 0xCFFFF
                }
            }
        );
        assert_eq!(
            Plane::of('\u{D0060}'),
            Plane {
                name: PLANE_NAMES[13],
                range: Range {
                    start: 0xD0000,
                    end: 0xDFFFF
                }
            }
        );
        assert_eq!(
            Plane::of('\u{E0060}'),
            Plane {
                name: PLANE_NAMES[14],
                range: Range {
                    start: 0xE0000,
                    end: 0xEFFFF
                }
            }
        );
        assert_eq!(
            Plane::of('\u{F0060}'),
            Plane {
                name: PLANE_NAMES[15],
                range: Range {
                    start: 0xF0000,
                    end: 0xFFFFF
                }
            }
        );
        assert_eq!(
            Plane::of('\u{100060}'),
            Plane {
                name: PLANE_NAMES[16],
                range: Range {
                    start: 0x100000,
                    end: 0x10FFFF
                }
            }
        );
    }
}
