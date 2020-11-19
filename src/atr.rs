use crate::tr::TrueRange;
use wilders_rs::Wilders;
use ta_common::traits::Indicator;
#[doc(include = "../docs/atr.md")]
pub struct AvgTrueRange {
    period: u32,
    tr: TrueRange,
    wilders: Wilders,
}

impl AvgTrueRange {
    pub fn new(period: u32) -> AvgTrueRange {
        Self {
            period,
            tr: TrueRange::new(),
            wilders: Wilders::new(period),
        }
    }
}

impl Indicator<[f64; 3], Option<f64>> for AvgTrueRange {
    fn next(&mut self, input: [f64; 3]) -> Option<f64> {
        let tr = self.tr.next(input);
        println!("{}",tr);
        let wl = self.wilders.next(tr);
        wl
    }

    fn reset(&mut self) {
        self.tr.reset();
        self.wilders.reset();
    }
}


#[cfg(test)]
mod tests {
    use ta_common::traits::Indicator;

    use crate::atr::AvgTrueRange;

    #[test]
    fn avg_true_range_works() {
        let mut atr = AvgTrueRange::new(5);
        assert_eq!(atr.next([82.15, 81.29, 81.59, ]), None);
        assert_eq!(atr.next([81.89, 80.64, 81.06, ]), None);
        assert_eq!(atr.next([83.03, 81.31, 82.87, ]), None);
        assert_eq!(atr.next([83.30, 82.65, 83.00, ]), None);
        assert_eq!(atr.next([83.85, 83.07, 83.61, ]), Some(1.0627999999999962));
        assert_eq!(atr.next([83.90, 83.11, 83.15, ]), Some(1.0082399999999982));
        assert_eq!(atr.next([83.33, 82.49, 82.84, ]), Some(0.9745919999999992));
        assert_eq!(atr.next([84.30, 82.30, 83.99, ]), Some(1.1796735999999994));
        assert_eq!(atr.next([84.84, 84.15, 84.55, ]), Some(1.1137388800000012));
        assert_eq!(atr.next([85.00, 84.11, 84.36, ]), Some(1.068991104000001));
        assert_eq!(atr.next([85.90, 84.03, 85.53, ]), Some(1.2291928832000019));
        assert_eq!(atr.next([86.58, 85.39, 86.54, ]), Some(1.221354306560001));
        assert_eq!(atr.next([86.98, 85.76, 86.89, ]), Some(1.2210834452480006));
        assert_eq!(atr.next([88.00, 87.17, 87.77, ]), Some(1.1988667561984003));
        assert_eq!(atr.next([87.87, 87.01, 87.29, ]), Some(1.1310934049587202));
    }
}



