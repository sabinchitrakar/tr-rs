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