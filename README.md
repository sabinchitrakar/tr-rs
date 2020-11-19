<!-- START doctoc generated TOC please keep comment here to allow auto update -->
<!-- DON'T EDIT THIS SECTION, INSTEAD RE-RUN doctoc TO UPDATE -->


- [Average True Range](#average-true-range)
    - [Calculation](#calculation)
- [Normalized Average True Range](#normalized-average-true-range)
    - [Calculation](#calculation-1)
- [True Range](#true-range)
    - [Calculation](#calculation-2)

<!-- END doctoc generated TOC please keep comment here to allow auto update -->


<a name="atrmd"></a>

# Average True Range
```
use ta_common::traits::Indicator;
use tr_rs::AvgTrueRange;

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
```
### Calculation
ATR=wilders(TR(input));

<a name="natrmd"></a>

# Normalized Average True Range
```
use ta_common::traits::Indicator;
use tr_rs::NormAvgTrueRange;

let mut natr = NormAvgTrueRange::new(5);
assert_eq!(natr.next([82.15,	81.29,	81.59,]),None);
assert_eq!(natr.next([81.89,	80.64,	81.06,]),None);
assert_eq!(natr.next([83.03,	81.31,	82.87,]),None);
assert_eq!(natr.next([83.30,	82.65,	83.00,]),None);
assert_eq!(natr.next([83.85,	83.07,	83.61,]),Some(1.2711398158115013));
assert_eq!(natr.next([83.90,	83.11,	83.15,]),Some(1.2125556223692102));
assert_eq!(natr.next([83.33,	82.49,	82.84,]),Some(1.1764751327860927));
assert_eq!(natr.next([84.30,	82.30,	83.99,]),Some(1.40454054054054));
assert_eq!(natr.next([84.84,	84.15,	84.55,]),Some(1.3172547368421068));
assert_eq!(natr.next([85.00,	84.11,	84.36,]),Some(1.2671776955903284));
assert_eq!(natr.next([85.90,	84.03,	85.53,]),Some(1.4371482324330667));
assert_eq!(natr.next([86.58,	85.39,	86.54,]),Some(1.411317664155305));
assert_eq!(natr.next([86.98,	85.76,	86.89,]),Some(1.4053210326251588));
assert_eq!(natr.next([88.00,	87.17,	87.77,]),Some(1.3659186011147322));
assert_eq!(natr.next([87.87,	87.01,	87.29,]),Some(1.295788068459984));
```

### Calculation
NATR=100*(ATR(input)/input)

<a name="trmd"></a>

# True Range
```
use ta_common::traits::Indicator;
use tr_rs::TrueRange;
let mut tr = TrueRange::new();
assert_eq!(tr.next([82.15, 81.29, 81.59]), 0.8599999999999994);
assert_eq!(tr.next([81.89, 80.64, 81.06]), 1.25);
assert_eq!(tr.next([83.03, 81.31, 82.87]), 1.9699999999999989);
assert_eq!(tr.next([83.30, 82.65, 83.00]), 0.6499999999999915);
assert_eq!(tr.next([83.85, 83.07, 83.61]), 0.8499999999999943);
assert_eq!(tr.next([83.90, 83.11, 83.15]), 0.7900000000000063);
assert_eq!(tr.next([83.33, 82.49, 82.84]), 0.8400000000000034);
assert_eq!(tr.next([84.30, 82.30, 83.99]), 2.00);
assert_eq!(tr.next([84.84, 84.15, 84.55]), 0.8500000000000085);
assert_eq!(tr.next([85.00, 84.11, 84.36]), 0.8900000000000006);
assert_eq!(tr.next([85.90, 84.03, 85.53]), 1.8700000000000045);
assert_eq!(tr.next([86.58, 85.39, 86.54]), 1.1899999999999977);
assert_eq!(tr.next([86.98, 85.76, 86.89]), 1.2199999999999989);
assert_eq!(tr.next([88.00, 87.17, 87.77]), 1.1099999999999994);
assert_eq!(tr.next([87.87, 87.01, 87.29]), 0.8599999999999994);
```
### Calculation
TR=MAX(high-low | high-close<sub>p</sub> | low-close<sub>p</sub>)  