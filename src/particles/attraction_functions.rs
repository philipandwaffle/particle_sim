use std::{collections::HashMap, string};

use super::particle_metadata::AttractionFunc;

fn get_pos_fns() -> HashMap<String, AttractionFunc> {
    let range1x0_return0x01: AttractionFunc = |x| {
        if x < 1.0 {
            return 0.01;
        } else {
            return 0.0;
        }
    };
    let range1x5_return0x01: AttractionFunc = |x| {
        if x < 1.5 {
            return 0.01;
        } else {
            return 0.0;
        }
    };
    let range2x0_return0x01: AttractionFunc = |x| {
        if x < 2.0 {
            return 0.01;
        } else {
            return 0.0;
        }
    };
    let range2x5_return0x01: AttractionFunc = |x| {
        if x < 2.5 {
            return 0.01;
        } else {
            return 0.0;
        }
    };

    let range1x0_return0x02: AttractionFunc = |x| {
        if x < 1.0 {
            return 0.02;
        } else {
            return 0.0;
        }
    };
    let range1x5_return0x02: AttractionFunc = |x| {
        if x < 1.5 {
            return 0.02;
        } else {
            return 0.0;
        }
    };
    let range2x0_return0x02: AttractionFunc = |x| {
        if x < 2.0 {
            return 0.02;
        } else {
            return 0.0;
        }
    };
    let range2x5_return0x02: AttractionFunc = |x| {
        if x < 2.5 {
            return 0.02;
        } else {
            return 0.0;
        }
    };

    let range1x0_return0x03: AttractionFunc = |x| {
        if x < 1.0 {
            return 0.03;
        } else {
            return 0.0;
        }
    };
    let range1x5_return0x03: AttractionFunc = |x| {
        if x < 1.5 {
            return 0.03;
        } else {
            return 0.0;
        }
    };
    let range2x0_return0x03: AttractionFunc = |x| {
        if x < 2.0 {
            return 0.03;
        } else {
            return 0.0;
        }
    };
    let range2x5_return0x03: AttractionFunc = |x| {
        if x < 2.5 {
            return 0.03;
        } else {
            return 0.0;
        }
    };

    let range1x0_return0x04: AttractionFunc = |x| {
        if x < 1.0 {
            return 0.04;
        } else {
            return 0.0;
        }
    };

    let range1x5_return0x04: AttractionFunc = |x| {
        if x < 1.5 {
            return 0.04;
        } else {
            return 0.0;
        }
    };

    let range2x0_return0x04: AttractionFunc = |x| {
        if x < 2.0 {
            return 0.04;
        } else {
            return 0.0;
        }
    };

    let range2x5_return0x04: AttractionFunc = |x| {
        if x < 2.5 {
            return 0.04;
        } else {
            return 0.0;
        }
    };

    let range1x0_return0x05: AttractionFunc = |x| {
        if x < 1.0 {
            return 0.05;
        } else {
            return 0.0;
        }
    };
    let range1x5_return0x05: AttractionFunc = |x| {
        if x < 1.5 {
            return 0.05;
        } else {
            return 0.0;
        }
    };
    let range2x0_return0x05: AttractionFunc = |x| {
        if x < 2.0 {
            return 0.05;
        } else {
            return 0.0;
        }
    };
    let range2x5_return0x05: AttractionFunc = |x| {
        if x < 2.5 {
            return 0.05;
        } else {
            return 0.0;
        }
    };

    return HashMap::from([
        ("pos_range1x0_return0x01".into(), range1x0_return0x01),
        ("pos_range1x0_return0x01".into(), range1x0_return0x01),
        ("pos_range1x0_return0x01".into(), range1x0_return0x01),
        ("pos_range1x0_return0x01".into(), range1x0_return0x01),
        ("pos_range1x5_return0x01".into(), range1x5_return0x01),
        ("pos_range1x5_return0x01".into(), range1x5_return0x01),
        ("pos_range1x5_return0x01".into(), range1x5_return0x01),
        ("pos_range1x5_return0x01".into(), range1x5_return0x01),
        ("pos_range2x0_return0x01".into(), range2x0_return0x01),
        ("pos_range2x0_return0x01".into(), range2x0_return0x01),
        ("pos_range2x0_return0x01".into(), range2x0_return0x01),
        ("pos_range2x0_return0x01".into(), range2x0_return0x01),
        ("pos_range2x5_return0x01".into(), range2x5_return0x01),
        ("pos_range2x5_return0x01".into(), range2x5_return0x01),
        ("pos_range2x5_return0x01".into(), range2x5_return0x01),
        ("pos_range2x5_return0x01".into(), range2x5_return0x01),
        ("pos_range1x0_return0x01".into(), range1x0_return0x01),
        ("pos_range1x0_return0x01".into(), range1x0_return0x01),
        ("pos_range1x0_return0x01".into(), range1x0_return0x01),
        ("pos_range1x0_return0x01".into(), range1x0_return0x01),
        ("pos_range1x0_return0x02".into(), range1x0_return0x02),
        ("pos_range1x0_return0x02".into(), range1x0_return0x02),
        ("pos_range1x0_return0x02".into(), range1x0_return0x02),
        ("pos_range1x0_return0x02".into(), range1x0_return0x02),
        ("pos_range1x5_return0x02".into(), range1x5_return0x02),
        ("pos_range1x5_return0x02".into(), range1x5_return0x02),
        ("pos_range1x5_return0x02".into(), range1x5_return0x02),
        ("pos_range1x5_return0x02".into(), range1x5_return0x02),
        ("pos_range2x0_return0x02".into(), range2x0_return0x02),
        ("pos_range2x0_return0x02".into(), range2x0_return0x02),
        ("pos_range2x0_return0x02".into(), range2x0_return0x02),
        ("pos_range2x0_return0x02".into(), range2x0_return0x02),
        ("pos_range2x5_return0x02".into(), range2x5_return0x02),
        ("pos_range2x5_return0x02".into(), range2x5_return0x02),
        ("pos_range2x5_return0x02".into(), range2x5_return0x02),
        ("pos_range2x5_return0x02".into(), range2x5_return0x02),
        ("pos_range1x0_return0x02".into(), range1x0_return0x02),
        ("pos_range1x0_return0x02".into(), range1x0_return0x02),
        ("pos_range1x0_return0x02".into(), range1x0_return0x02),
        ("pos_range1x0_return0x02".into(), range1x0_return0x02),
        ("pos_range1x0_return0x03".into(), range1x0_return0x03),
        ("pos_range1x0_return0x03".into(), range1x0_return0x03),
        ("pos_range1x0_return0x03".into(), range1x0_return0x03),
        ("pos_range1x0_return0x03".into(), range1x0_return0x03),
        ("pos_range1x5_return0x03".into(), range1x5_return0x03),
        ("pos_range1x5_return0x03".into(), range1x5_return0x03),
        ("pos_range1x5_return0x03".into(), range1x5_return0x03),
        ("pos_range1x5_return0x03".into(), range1x5_return0x03),
        ("pos_range2x0_return0x03".into(), range2x0_return0x03),
        ("pos_range2x0_return0x03".into(), range2x0_return0x03),
        ("pos_range2x0_return0x03".into(), range2x0_return0x03),
        ("pos_range2x0_return0x03".into(), range2x0_return0x03),
        ("pos_range2x5_return0x03".into(), range2x5_return0x03),
        ("pos_range2x5_return0x03".into(), range2x5_return0x03),
        ("pos_range2x5_return0x03".into(), range2x5_return0x03),
        ("pos_range2x5_return0x03".into(), range2x5_return0x03),
        ("pos_range1x0_return0x03".into(), range1x0_return0x03),
        ("pos_range1x0_return0x03".into(), range1x0_return0x03),
        ("pos_range1x0_return0x03".into(), range1x0_return0x03),
        ("pos_range1x0_return0x03".into(), range1x0_return0x03),
        ("pos_range1x0_return0x04".into(), range1x0_return0x04),
        ("pos_range1x0_return0x04".into(), range1x0_return0x04),
        ("pos_range1x0_return0x04".into(), range1x0_return0x04),
        ("pos_range1x0_return0x04".into(), range1x0_return0x04),
        ("pos_range1x5_return0x04".into(), range1x5_return0x04),
        ("pos_range1x5_return0x04".into(), range1x5_return0x04),
        ("pos_range1x5_return0x04".into(), range1x5_return0x04),
        ("pos_range1x5_return0x04".into(), range1x5_return0x04),
        ("pos_range2x0_return0x04".into(), range2x0_return0x04),
        ("pos_range2x0_return0x04".into(), range2x0_return0x04),
        ("pos_range2x0_return0x04".into(), range2x0_return0x04),
        ("pos_range2x0_return0x04".into(), range2x0_return0x04),
        ("pos_range2x5_return0x04".into(), range2x5_return0x04),
        ("pos_range2x5_return0x04".into(), range2x5_return0x04),
        ("pos_range2x5_return0x04".into(), range2x5_return0x04),
        ("pos_range2x5_return0x04".into(), range2x5_return0x04),
        ("pos_range1x0_return0x04".into(), range1x0_return0x04),
        ("pos_range1x0_return0x04".into(), range1x0_return0x04),
        ("pos_range1x0_return0x04".into(), range1x0_return0x04),
        ("pos_range1x0_return0x04".into(), range1x0_return0x04),
        ("pos_range1x0_return0x05".into(), range1x0_return0x05),
        ("pos_range1x0_return0x05".into(), range1x0_return0x05),
        ("pos_range1x0_return0x05".into(), range1x0_return0x05),
        ("pos_range1x0_return0x05".into(), range1x0_return0x05),
        ("pos_range1x5_return0x05".into(), range1x5_return0x05),
        ("pos_range1x5_return0x05".into(), range1x5_return0x05),
        ("pos_range1x5_return0x05".into(), range1x5_return0x05),
        ("pos_range1x5_return0x05".into(), range1x5_return0x05),
        ("pos_range2x0_return0x05".into(), range2x0_return0x05),
        ("pos_range2x0_return0x05".into(), range2x0_return0x05),
        ("pos_range2x0_return0x05".into(), range2x0_return0x05),
        ("pos_range2x0_return0x05".into(), range2x0_return0x05),
        ("pos_range2x5_return0x05".into(), range2x5_return0x05),
        ("pos_range2x5_return0x05".into(), range2x5_return0x05),
        ("pos_range2x5_return0x05".into(), range2x5_return0x05),
        ("pos_range2x5_return0x05".into(), range2x5_return0x05),
        ("pos_range1x0_return0x05".into(), range1x0_return0x05),
        ("pos_range1x0_return0x05".into(), range1x0_return0x05),
        ("pos_range1x0_return0x05".into(), range1x0_return0x05),
        ("pos_range1x0_return0x05".into(), range1x0_return0x05),
    ]);
}

fn get_neg_fns() -> HashMap<String, AttractionFunc> {
    let range1x0_return0x01: AttractionFunc = |x| {
        if x < 1.0 {
            return 0.01;
        } else {
            return 0.0;
        }
    };
    let range1x5_return0x01: AttractionFunc = |x| {
        if x < 1.5 {
            return 0.01;
        } else {
            return 0.0;
        }
    };
    let range2x0_return0x01: AttractionFunc = |x| {
        if x < 2.0 {
            return 0.01;
        } else {
            return 0.0;
        }
    };
    let range2x5_return0x01: AttractionFunc = |x| {
        if x < 2.5 {
            return 0.01;
        } else {
            return 0.0;
        }
    };

    let range1x0_return0x02: AttractionFunc = |x| {
        if x < 1.0 {
            return 0.02;
        } else {
            return 0.0;
        }
    };
    let range1x5_return0x02: AttractionFunc = |x| {
        if x < 1.5 {
            return 0.02;
        } else {
            return 0.0;
        }
    };
    let range2x0_return0x02: AttractionFunc = |x| {
        if x < 2.0 {
            return 0.02;
        } else {
            return 0.0;
        }
    };
    let range2x5_return0x02: AttractionFunc = |x| {
        if x < 2.5 {
            return 0.02;
        } else {
            return 0.0;
        }
    };

    let range1x0_return0x03: AttractionFunc = |x| {
        if x < 1.0 {
            return 0.03;
        } else {
            return 0.0;
        }
    };
    let range1x5_return0x03: AttractionFunc = |x| {
        if x < 1.5 {
            return 0.03;
        } else {
            return 0.0;
        }
    };
    let range2x0_return0x03: AttractionFunc = |x| {
        if x < 2.0 {
            return 0.03;
        } else {
            return 0.0;
        }
    };
    let range2x5_return0x03: AttractionFunc = |x| {
        if x < 2.5 {
            return 0.03;
        } else {
            return 0.0;
        }
    };

    let range1x0_return0x04: AttractionFunc = |x| {
        if x < 1.0 {
            return 0.04;
        } else {
            return 0.0;
        }
    };

    let range1x5_return0x04: AttractionFunc = |x| {
        if x < 1.5 {
            return 0.04;
        } else {
            return 0.0;
        }
    };

    let range2x0_return0x04: AttractionFunc = |x| {
        if x < 2.0 {
            return 0.04;
        } else {
            return 0.0;
        }
    };

    let range2x5_return0x04: AttractionFunc = |x| {
        if x < 2.5 {
            return 0.04;
        } else {
            return 0.0;
        }
    };

    let range1x0_return0x05: AttractionFunc = |x| {
        if x < 1.0 {
            return 0.05;
        } else {
            return 0.0;
        }
    };
    let range1x5_return0x05: AttractionFunc = |x| {
        if x < 1.5 {
            return 0.05;
        } else {
            return 0.0;
        }
    };
    let range2x0_return0x05: AttractionFunc = |x| {
        if x < 2.0 {
            return 0.05;
        } else {
            return 0.0;
        }
    };
    let range2x5_return0x05: AttractionFunc = |x| {
        if x < 2.5 {
            return 0.05;
        } else {
            return 0.0;
        }
    };

    return HashMap::from([
        ("neg_range1x0_return0x01".into(), range1x0_return0x01),
        ("neg_range1x0_return0x01".into(), range1x0_return0x01),
        ("neg_range1x0_return0x01".into(), range1x0_return0x01),
        ("neg_range1x0_return0x01".into(), range1x0_return0x01),
        ("neg_range1x5_return0x01".into(), range1x5_return0x01),
        ("neg_range1x5_return0x01".into(), range1x5_return0x01),
        ("neg_range1x5_return0x01".into(), range1x5_return0x01),
        ("neg_range1x5_return0x01".into(), range1x5_return0x01),
        ("neg_range2x0_return0x01".into(), range2x0_return0x01),
        ("neg_range2x0_return0x01".into(), range2x0_return0x01),
        ("neg_range2x0_return0x01".into(), range2x0_return0x01),
        ("neg_range2x0_return0x01".into(), range2x0_return0x01),
        ("neg_range2x5_return0x01".into(), range2x5_return0x01),
        ("neg_range2x5_return0x01".into(), range2x5_return0x01),
        ("neg_range2x5_return0x01".into(), range2x5_return0x01),
        ("neg_range2x5_return0x01".into(), range2x5_return0x01),
        ("neg_range1x0_return0x01".into(), range1x0_return0x01),
        ("neg_range1x0_return0x01".into(), range1x0_return0x01),
        ("neg_range1x0_return0x01".into(), range1x0_return0x01),
        ("neg_range1x0_return0x01".into(), range1x0_return0x01),
        ("neg_range1x0_return0x02".into(), range1x0_return0x02),
        ("neg_range1x0_return0x02".into(), range1x0_return0x02),
        ("neg_range1x0_return0x02".into(), range1x0_return0x02),
        ("neg_range1x0_return0x02".into(), range1x0_return0x02),
        ("neg_range1x5_return0x02".into(), range1x5_return0x02),
        ("neg_range1x5_return0x02".into(), range1x5_return0x02),
        ("neg_range1x5_return0x02".into(), range1x5_return0x02),
        ("neg_range1x5_return0x02".into(), range1x5_return0x02),
        ("neg_range2x0_return0x02".into(), range2x0_return0x02),
        ("neg_range2x0_return0x02".into(), range2x0_return0x02),
        ("neg_range2x0_return0x02".into(), range2x0_return0x02),
        ("neg_range2x0_return0x02".into(), range2x0_return0x02),
        ("neg_range2x5_return0x02".into(), range2x5_return0x02),
        ("neg_range2x5_return0x02".into(), range2x5_return0x02),
        ("neg_range2x5_return0x02".into(), range2x5_return0x02),
        ("neg_range2x5_return0x02".into(), range2x5_return0x02),
        ("neg_range1x0_return0x02".into(), range1x0_return0x02),
        ("neg_range1x0_return0x02".into(), range1x0_return0x02),
        ("neg_range1x0_return0x02".into(), range1x0_return0x02),
        ("neg_range1x0_return0x02".into(), range1x0_return0x02),
        ("neg_range1x0_return0x03".into(), range1x0_return0x03),
        ("neg_range1x0_return0x03".into(), range1x0_return0x03),
        ("neg_range1x0_return0x03".into(), range1x0_return0x03),
        ("neg_range1x0_return0x03".into(), range1x0_return0x03),
        ("neg_range1x5_return0x03".into(), range1x5_return0x03),
        ("neg_range1x5_return0x03".into(), range1x5_return0x03),
        ("neg_range1x5_return0x03".into(), range1x5_return0x03),
        ("neg_range1x5_return0x03".into(), range1x5_return0x03),
        ("neg_range2x0_return0x03".into(), range2x0_return0x03),
        ("neg_range2x0_return0x03".into(), range2x0_return0x03),
        ("neg_range2x0_return0x03".into(), range2x0_return0x03),
        ("neg_range2x0_return0x03".into(), range2x0_return0x03),
        ("neg_range2x5_return0x03".into(), range2x5_return0x03),
        ("neg_range2x5_return0x03".into(), range2x5_return0x03),
        ("neg_range2x5_return0x03".into(), range2x5_return0x03),
        ("neg_range2x5_return0x03".into(), range2x5_return0x03),
        ("neg_range1x0_return0x03".into(), range1x0_return0x03),
        ("neg_range1x0_return0x03".into(), range1x0_return0x03),
        ("neg_range1x0_return0x03".into(), range1x0_return0x03),
        ("neg_range1x0_return0x03".into(), range1x0_return0x03),
        ("neg_range1x0_return0x04".into(), range1x0_return0x04),
        ("neg_range1x0_return0x04".into(), range1x0_return0x04),
        ("neg_range1x0_return0x04".into(), range1x0_return0x04),
        ("neg_range1x0_return0x04".into(), range1x0_return0x04),
        ("neg_range1x5_return0x04".into(), range1x5_return0x04),
        ("neg_range1x5_return0x04".into(), range1x5_return0x04),
        ("neg_range1x5_return0x04".into(), range1x5_return0x04),
        ("neg_range1x5_return0x04".into(), range1x5_return0x04),
        ("neg_range2x0_return0x04".into(), range2x0_return0x04),
        ("neg_range2x0_return0x04".into(), range2x0_return0x04),
        ("neg_range2x0_return0x04".into(), range2x0_return0x04),
        ("neg_range2x0_return0x04".into(), range2x0_return0x04),
        ("neg_range2x5_return0x04".into(), range2x5_return0x04),
        ("neg_range2x5_return0x04".into(), range2x5_return0x04),
        ("neg_range2x5_return0x04".into(), range2x5_return0x04),
        ("neg_range2x5_return0x04".into(), range2x5_return0x04),
        ("neg_range1x0_return0x04".into(), range1x0_return0x04),
        ("neg_range1x0_return0x04".into(), range1x0_return0x04),
        ("neg_range1x0_return0x04".into(), range1x0_return0x04),
        ("neg_range1x0_return0x04".into(), range1x0_return0x04),
        ("neg_range1x0_return0x05".into(), range1x0_return0x05),
        ("neg_range1x0_return0x05".into(), range1x0_return0x05),
        ("neg_range1x0_return0x05".into(), range1x0_return0x05),
        ("neg_range1x0_return0x05".into(), range1x0_return0x05),
        ("neg_range1x5_return0x05".into(), range1x5_return0x05),
        ("neg_range1x5_return0x05".into(), range1x5_return0x05),
        ("neg_range1x5_return0x05".into(), range1x5_return0x05),
        ("neg_range1x5_return0x05".into(), range1x5_return0x05),
        ("neg_range2x0_return0x05".into(), range2x0_return0x05),
        ("neg_range2x0_return0x05".into(), range2x0_return0x05),
        ("neg_range2x0_return0x05".into(), range2x0_return0x05),
        ("neg_range2x0_return0x05".into(), range2x0_return0x05),
        ("neg_range2x5_return0x05".into(), range2x5_return0x05),
        ("neg_range2x5_return0x05".into(), range2x5_return0x05),
        ("neg_range2x5_return0x05".into(), range2x5_return0x05),
        ("neg_range2x5_return0x05".into(), range2x5_return0x05),
        ("neg_range1x0_return0x05".into(), range1x0_return0x05),
        ("neg_range1x0_return0x05".into(), range1x0_return0x05),
        ("neg_range1x0_return0x05".into(), range1x0_return0x05),
        ("neg_range1x0_return0x05".into(), range1x0_return0x05),
    ]);
}

pub fn get_fns() -> HashMap<String, AttractionFunc> {
    let zero: AttractionFunc = |_| {
        return 0.0;
    };

    let mut res = HashMap::from([("zer_range0x0_return0x00".into(), zero)]);
    let pos = get_pos_fns();
    let neg = get_neg_fns();

    res.extend(pos);
    res.extend(neg);
    return res;
}
