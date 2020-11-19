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